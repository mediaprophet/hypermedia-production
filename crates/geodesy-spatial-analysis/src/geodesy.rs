//! Geodetic computations, planetary ellipsoids, geodesic direct/inverse solvers,
//! projections, discrete global grids, Tissot indicatrices, and antimeridian handling.
//!
//! Grounded in international human rights instruments (UDHR Art. 19, 27; ICESCR Art. 15)
//! and Informatics Fiduciary duties ensuring scientific precision and geographic accuracy.

use serde::{Deserialize, Serialize};
use std::f64::consts::{FRAC_PI_2, PI};

/// Mean Earth radius used for spherical approximations (metres).
pub const EARTH_RADIUS_M: f64 = 6_371_008.8;
/// WGS 84 semi-major axis (metres).
pub const WGS84_A: f64 = 6_378_137.0;
/// WGS 84 reciprocal flattening.
pub const WGS84_INV_F: f64 = 298.257_223_563;
/// WGS 84 flattening.
pub const WGS84_F: f64 = 1.0 / WGS84_INV_F;
/// WGS 84 semi-minor axis (metres).
pub const WGS84_B: f64 = WGS84_A * (1.0 - WGS84_F);
/// WGS 84 eccentricity squared.
pub const WGS84_E2: f64 = WGS84_F * (2.0 - WGS84_F);

/// A 2D geographic or projected coordinate (degrees lon/lat or metres x/y).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
}

impl Coord {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}

/// A 2D bounding extent.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Extent {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

impl Default for Extent {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Extent {
    pub const EMPTY: Self = Self {
        min_x: f64::INFINITY,
        min_y: f64::INFINITY,
        max_x: f64::NEG_INFINITY,
        max_y: f64::NEG_INFINITY,
    };

    pub const WORLD: Self = Self {
        min_x: -180.0,
        min_y: -90.0,
        max_x: 180.0,
        max_y: 90.0,
    };

    pub const fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
        Self { min_x, min_y, max_x, max_y }
    }

    pub fn from_points(a: Coord, b: Coord) -> Self {
        Self {
            min_x: a.x.min(b.x),
            min_y: a.y.min(b.y),
            max_x: a.x.max(b.x),
            max_y: a.y.max(b.y),
        }
    }

    pub fn contains(&self, c: Coord) -> bool {
        c.x >= self.min_x && c.x <= self.max_x && c.y >= self.min_y && c.y <= self.max_y
    }

    pub fn width(&self) -> f64 {
        if self.max_x >= self.min_x { self.max_x - self.min_x } else { 0.0 }
    }

    pub fn height(&self) -> f64 {
        if self.max_y >= self.min_y { self.max_y - self.min_y } else { 0.0 }
    }
}

/// A celestial body rotational ellipsoid.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Body {
    pub id: &'static str,
    pub name: &'static str,
    pub a: f64,
    pub inv_f: f64,
}

pub const EARTH: Body = Body {
    id: "earth",
    name: "Earth",
    a: 6_378_137.0,
    inv_f: 298.257_223_563,
};

pub const BODIES: [Body; 12] = [
    EARTH,
    Body { id: "moon", name: "Moon", a: 1_737_400.0, inv_f: 0.0 },
    Body { id: "mars", name: "Mars", a: 3_396_190.0, inv_f: 169.894_447 },
    Body { id: "callisto", name: "Callisto", a: 2_410_300.0, inv_f: 0.0 },
    Body { id: "charon", name: "Charon", a: 606_000.0, inv_f: 0.0 },
    Body { id: "europa", name: "Europa", a: 1_560_800.0, inv_f: 0.0 },
    Body { id: "ganymede", name: "Ganymede", a: 2_631_200.0, inv_f: 0.0 },
    Body { id: "io", name: "Io", a: 1_821_600.0, inv_f: 0.0 },
    Body { id: "mercury", name: "Mercury", a: 2_439_400.0, inv_f: 0.0 },
    Body { id: "pluto", name: "Pluto", a: 1_188_300.0, inv_f: 0.0 },
    Body { id: "titan", name: "Titan", a: 2_574_730.0, inv_f: 0.0 },
    Body { id: "venus", name: "Venus", a: 6_051_800.0, inv_f: 0.0 },
];

impl Body {
    pub fn find(s: &str) -> Option<&'static Body> {
        let s = s.trim();
        BODIES.iter().find(|b| b.id.eq_ignore_ascii_case(s) || b.name.eq_ignore_ascii_case(s))
    }

    pub fn get(id: &str) -> &'static Body {
        Self::find(id).unwrap_or(&EARTH)
    }

    pub fn is_earth(&self) -> bool {
        self.id == EARTH.id
    }

    pub fn b(&self) -> f64 {
        if self.inv_f > 0.0 { self.a * (1.0 - 1.0 / self.inv_f) } else { self.a }
    }

    pub fn mean_radius_m(&self) -> f64 {
        (2.0 * self.a + self.b()) / 3.0
    }

    pub fn ratio(&self) -> f64 {
        if self.is_earth() { 1.0 } else { self.mean_radius_m() / EARTH_RADIUS_M }
    }

    pub fn length(&self, earth_m: f64) -> f64 {
        earth_m * self.ratio()
    }

    pub fn area(&self, earth_m2: f64) -> f64 {
        earth_m2 * self.ratio() * self.ratio()
    }
}

/// Great-circle Haversine distance in metres between two points in degrees (lon, lat).
pub fn haversine_m(c1: Coord, c2: Coord) -> f64 {
    let lat1 = c1.y.to_radians();
    let lat2 = c2.y.to_radians();
    let dlat = (c2.y - c1.y).to_radians();
    let dlon = (c2.x - c1.x).to_radians();

    let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().clamp(-1.0, 1.0).asin();
    EARTH_RADIUS_M * c
}

/// WGS84 geodesic inverse problem: computes distance in metres and initial/final azimuths in degrees.
/// Uses Vincenty's equations with robust spherical fallback if antipodal iteration fails.
pub fn wgs84_inverse(lat1_deg: f64, lon1_deg: f64, lat2_deg: f64, lon2_deg: f64) -> (f64, f64, f64) {
    let lat1 = lat1_deg.to_radians();
    let lon1 = lon1_deg.to_radians();
    let lat2 = lat2_deg.to_radians();
    let lon2 = lon2_deg.to_radians();

    let l = lon2 - lon1;
    let u1 = ((1.0 - WGS84_F) * lat1.tan()).atan();
    let u2 = ((1.0 - WGS84_F) * lat2.tan()).atan();
    let (sin_u1, cos_u1) = u1.sin_cos();
    let (sin_u2, cos_u2) = u2.sin_cos();

    let mut lambda = l;
    let mut lambda_prev;
    let mut iter_limit = 100;
    let mut sin_sigma = 0.0;
    let mut cos_sigma = 0.0;
    let mut sigma = 0.0;
    let mut cos_sq_alpha = 0.0;
    let mut cos_2sigma_m = 0.0;

    let converged = loop {
        let (sin_lambda, cos_lambda) = lambda.sin_cos();
        sin_sigma = ((cos_u2 * sin_lambda).powi(2)
            + (cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda).powi(2))
        .sqrt();

        if sin_sigma.abs() < 1e-12 {
            // Coincident points
            return (0.0, 0.0, 0.0);
        }

        cos_sigma = sin_u1 * sin_u2 + cos_u1 * cos_u2 * cos_lambda;
        sigma = sin_sigma.atan2(cos_sigma);
        let sin_alpha = cos_u1 * cos_u2 * sin_lambda / sin_sigma;
        cos_sq_alpha = 1.0 - sin_alpha * sin_alpha;

        cos_2sigma_m = if cos_sq_alpha.abs() > 1e-12 {
            cos_sigma - 2.0 * sin_u1 * sin_u2 / cos_sq_alpha
        } else {
            0.0
        };

        let c = WGS84_F / 16.0 * cos_sq_alpha * (4.0 + WGS84_F * (4.0 - 3.0 * cos_sq_alpha));
        lambda_prev = lambda;
        lambda = l
            + (1.0 - c)
                * WGS84_F
                * sin_alpha
                * (sigma
                    + c * sin_sigma
                        * (cos_2sigma_m
                            + c * cos_sigma * (-1.0 + 2.0 * cos_2sigma_m * cos_2sigma_m)));

        iter_limit -= 1;
        if (lambda - lambda_prev).abs() < 1e-12 || iter_limit == 0 {
            break iter_limit > 0;
        }
    };

    if !converged {
        // Fallback to haversine for antipodal edge cases
        let dist = haversine_m(Coord::new(lon1_deg, lat1_deg), Coord::new(lon2_deg, lat2_deg));
        let dlon = lon2 - lon1;
        let y = dlon.sin() * lat2.cos();
        let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * dlon.cos();
        let az = y.atan2(x).to_degrees().rem_euclid(360.0);
        return (dist, az, az);
    }

    let u_sq = cos_sq_alpha * (WGS84_A * WGS84_A - WGS84_B * WGS84_B) / (WGS84_B * WGS84_B);
    let a_coeff = 1.0 + u_sq / 16384.0 * (4096.0 + u_sq * (-768.0 + u_sq * (320.0 - 175.0 * u_sq)));
    let b_coeff = u_sq / 1024.0 * (256.0 + u_sq * (-128.0 + u_sq * (74.0 - 47.0 * u_sq)));
    let delta_sigma = b_coeff
        * sin_sigma
        * (cos_2sigma_m
            + b_coeff
                / 4.0
                * (cos_sigma * (-1.0 + 2.0 * cos_2sigma_m * cos_2sigma_m)
                    - b_coeff / 6.0
                        * cos_2sigma_m
                        * (-3.0 + 4.0 * sin_sigma * sin_sigma)
                        * (-3.0 + 4.0 * cos_2sigma_m * cos_2sigma_m)));

    let s12 = WGS84_B * a_coeff * (sigma - delta_sigma);
    let (sin_lambda, cos_lambda) = lambda.sin_cos();
    let alpha1 = (cos_u2 * sin_lambda)
        .atan2(cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda)
        .to_degrees()
        .rem_euclid(360.0);
    let alpha2 = (cos_u1 * sin_lambda)
        .atan2(-sin_u1 * cos_u2 + cos_u1 * sin_u2 * cos_lambda)
        .to_degrees()
        .rem_euclid(360.0);

    (s12, alpha1, alpha2)
}

/// WGS84 geodesic direct problem: computes destination (lat, lon) and arrival azimuth from start, azimuth, and distance.
pub fn wgs84_direct(lat1_deg: f64, lon1_deg: f64, alpha1_deg: f64, s_m: f64) -> (f64, f64, f64) {
    let lat1 = lat1_deg.to_radians();
    let lon1 = lon1_deg.to_radians();
    let alpha1 = alpha1_deg.to_radians();

    let (sin_alpha1, cos_alpha1) = alpha1.sin_cos();
    let tan_u1 = (1.0 - WGS84_F) * lat1.tan();
    let cos_u1 = 1.0 / (1.0 + tan_u1 * tan_u1).sqrt();
    let sin_u1 = tan_u1 * cos_u1;

    let sigma1 = tan_u1.atan2(cos_alpha1);
    let sin_alpha = cos_u1 * sin_alpha1;
    let cos_sq_alpha = 1.0 - sin_alpha * sin_alpha;

    let u_sq = cos_sq_alpha * (WGS84_A * WGS84_A - WGS84_B * WGS84_B) / (WGS84_B * WGS84_B);
    let a_coeff = 1.0 + u_sq / 16384.0 * (4096.0 + u_sq * (-768.0 + u_sq * (320.0 - 175.0 * u_sq)));
    let b_coeff = u_sq / 1024.0 * (256.0 + u_sq * (-128.0 + u_sq * (74.0 - 47.0 * u_sq)));

    let mut sigma = s_m / (WGS84_B * a_coeff);
    let mut sigma_prev;
    let mut iter = 100;
    let mut cos_2sigma_m = 0.0;

    while iter > 0 {
        cos_2sigma_m = (2.0 * sigma1 + sigma).cos();
        let (sin_sigma, cos_sigma) = sigma.sin_cos();
        let delta_sigma = b_coeff
            * sin_sigma
            * (cos_2sigma_m
                + b_coeff
                    / 4.0
                    * (cos_sigma * (-1.0 + 2.0 * cos_2sigma_m * cos_2sigma_m)
                        - b_coeff / 6.0
                            * cos_2sigma_m
                            * (-3.0 + 4.0 * sin_sigma * sin_sigma)
                            * (-3.0 + 4.0 * cos_2sigma_m * cos_2sigma_m)));
        sigma_prev = sigma;
        sigma = s_m / (WGS84_B * a_coeff) + delta_sigma;
        if (sigma - sigma_prev).abs() < 1e-12 {
            break;
        }
        iter -= 1;
    }

    let (sin_sigma, cos_sigma) = sigma.sin_cos();
    let tmp = sin_u1 * sin_sigma - cos_u1 * cos_sigma * cos_alpha1;
    let lat2 = (sin_u1 * cos_sigma + cos_u1 * sin_sigma * cos_alpha1)
        .atan2((1.0 - WGS84_F) * (sin_alpha * sin_alpha + tmp * tmp).sqrt());

    let lambda = (sin_sigma * sin_alpha1)
        .atan2(cos_u1 * cos_sigma - sin_u1 * sin_sigma * cos_alpha1);
    let c = WGS84_F / 16.0 * cos_sq_alpha * (4.0 + WGS84_F * (4.0 - 3.0 * cos_sq_alpha));
    let omega = lambda
        - (1.0 - c)
            * WGS84_F
            * sin_alpha
            * (sigma
                + c * sin_sigma
                    * (cos_2sigma_m
                        + c * cos_sigma * (-1.0 + 2.0 * cos_2sigma_m * cos_2sigma_m)));

    let lon2 = lon1 + omega;
    let alpha2 = sin_alpha.atan2(-tmp);

    (
        lat2.to_degrees(),
        ((lon2.to_degrees() + 180.0).rem_euclid(360.0) - 180.0),
        alpha2.to_degrees().rem_euclid(360.0),
    )
}

/// Area of a closed polygon ring on the spherical earth (square metres, always positive).
pub fn geodesic_ring_area_m2(ring: &[Coord]) -> f64 {
    let n = ring.len();
    if n < 3 {
        return 0.0;
    }
    let mut total = 0.0;
    for i in 0..n {
        let p1 = ring[i];
        let p2 = ring[(i + 1) % n];
        let mut dlon = p2.x - p1.x;
        if dlon.abs() > 180.0 && dlon.abs() < 360.0 {
            dlon = (dlon + 180.0).rem_euclid(360.0) - 180.0;
        }
        total += dlon.to_radians() * (2.0 + p1.y.to_radians().sin() + p2.y.to_radians().sin());
    }
    (total * EARTH_RADIUS_M * EARTH_RADIUS_M / 2.0).abs()
}

/// Azimuthal Equidistant projection (Aeqd) centred at (lon0, lat0).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Aeqd {
    pub lon0: f64,
    pub lat0: f64,
}

impl Aeqd {
    pub fn new(centre: Coord) -> Self {
        Self {
            lon0: centre.x,
            lat0: centre.y.clamp(-90.0, 90.0),
        }
    }

    /// Forward projection: lon/lat in degrees -> planar metres east and north of centre.
    pub fn forward(&self, c: Coord) -> Coord {
        let (dist, az, _) = wgs84_inverse(self.lat0, self.lon0, c.y.clamp(-90.0, 90.0), c.x);
        let (sa, ca) = az.to_radians().sin_cos();
        Coord::new(dist * sa, dist * ca)
    }

    /// Inverse projection: planar metres -> lon/lat in degrees.
    pub fn inverse(&self, p: Coord) -> Coord {
        if !p.is_finite() {
            return Coord::new(f64::NAN, f64::NAN);
        }
        let dist = p.x.hypot(p.y);
        let az = p.x.atan2(p.y).to_degrees().rem_euclid(360.0);
        let (lat2, lon2, _) = wgs84_direct(self.lat0, self.lon0, az, dist);
        Coord::new(lon2, lat2)
    }
}

/// Sinusoidal projection: equal-area pseudocylindrical projection.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sinusoidal {
    pub a: f64,
    pub es: f64,
    pub lon0: f64,
    pub x0: f64,
    pub y0: f64,
    en: [f64; 5],
}

impl Sinusoidal {
    pub fn new(a: f64, es: f64, lon0: f64, x0: f64, y0: f64) -> Option<Self> {
        let ok = [a, es, lon0, x0, y0].iter().all(|v| v.is_finite()) && a > 0.0 && (0.0..1.0).contains(&es);
        ok.then(|| Self {
            a,
            es,
            lon0,
            x0,
            y0,
            en: mlfn_coefficients(es),
        })
    }

    pub fn wgs84(lon0: f64) -> Self {
        Self::new(WGS84_A, WGS84_E2, lon0, 0.0, 0.0).unwrap_or(Self {
            a: WGS84_A,
            es: 0.0,
            lon0,
            x0: 0.0,
            y0: 0.0,
            en: [1.0, 0.0, 0.0, 0.0, 0.0],
        })
    }

    /// Forward projection: lon/lat degrees -> projected metres.
    pub fn forward(&self, c: Coord) -> Coord {
        let lat = c.y.clamp(-90.0, 90.0).to_radians();
        let dlon = (c.x - self.lon0).to_radians();

        let x = if self.es == 0.0 {
            self.a * dlon * lat.cos()
        } else {
            self.a * dlon * lat.cos() / (1.0 - self.es * lat.sin().powi(2)).sqrt()
        };

        let y = if self.es == 0.0 {
            self.a * lat
        } else {
            self.a * mlfn(lat, lat.sin(), lat.cos(), &self.en)
        };

        Coord::new(x + self.x0, y + self.y0)
    }

    /// Inverse projection: projected metres -> lon/lat degrees.
    pub fn inverse(&self, p: Coord) -> Coord {
        let y = p.y - self.y0;
        let x = p.x - self.x0;

        let lat = if self.es == 0.0 {
            y / self.a
        } else {
            inv_mlfn(y / self.a, self.es, &self.en)
        };

        let lon = if lat.cos().abs() < 1e-10 {
            self.lon0
        } else if self.es == 0.0 {
            self.lon0 + (x / (self.a * lat.cos())).to_degrees()
        } else {
            let scale = (1.0 - self.es * lat.sin().powi(2)).sqrt() / (self.a * lat.cos());
            self.lon0 + (x * scale).to_degrees()
        };

        Coord::new((lon + 180.0).rem_euclid(360.0) - 180.0, lat.to_degrees())
    }
}

fn mlfn_coefficients(es: f64) -> [f64; 5] {
    let mut en = [0.0; 5];
    en[0] = 1.0 - es * (1.0 / 4.0 + es * (3.0 / 64.0 + es * (5.0 / 256.0)));
    en[1] = es * (3.0 / 8.0 + es * (3.0 / 32.0 + es * (45.0 / 1024.0)));
    en[2] = es * es * (15.0 / 256.0 + es * (45.0 / 1024.0));
    en[3] = es * es * es * (35.0 / 3072.0);
    en[4] = 0.0;
    en
}

fn mlfn(phi: f64, sphi: f64, cphi: f64, en: &[f64; 5]) -> f64 {
    let c = sphi * cphi;
    let s2 = sphi * sphi;
    en[0] * phi - c * (en[1] + s2 * (en[2] + s2 * en[3]))
}

fn inv_mlfn(arg: f64, es: f64, en: &[f64; 5]) -> f64 {
    let mut phi = arg;
    for _ in 0..10 {
        let (sphi, cphi) = phi.sin_cos();
        let k = 1.0 - es * sphi * sphi;
        let d = mlfn(phi, sphi, cphi, en) - arg;
        phi -= d * (k * k.sqrt());
        if d.abs() < 1e-11 {
            break;
        }
    }
    phi
}

/// Tissot's Indicatrix distortion characteristics.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Distortion {
    pub h: f64,
    pub k: f64,
    pub a: f64,
    pub b: f64,
    pub area: f64,
    pub max_angle_deg: f64,
}

/// Computes Tissot distortion of a projection at a given coordinate using Snyder's central differences.
pub fn tissot_distortion<F>(project: F, c: Coord) -> Option<Distortion>
where
    F: Fn(Coord) -> Option<Coord>,
{
    let step = 1e-4;
    let centre = project(c)?;
    let east = project(Coord::new(c.x + step, c.y))?;
    let west = project(Coord::new(c.x - step, c.y))?;
    let north = project(Coord::new(c.x, c.y + step))?;
    let south = project(Coord::new(c.x, c.y - step))?;

    let d_rad = 2.0 * step.to_radians();
    let x_lam = (east.x - west.x) / d_rad;
    let y_lam = (east.y - west.y) / d_rad;
    let x_phi = (north.x - south.x) / d_rad;
    let y_phi = (north.y - south.y) / d_rad;

    let phi = c.y.to_radians();
    let cos_phi = phi.cos().abs();
    if cos_phi < 1e-6 {
        return None;
    }

    let r_lam = WGS84_A * cos_phi / (1.0 - WGS84_E2 * phi.sin().powi(2)).sqrt();
    let r_phi = WGS84_A * (1.0 - WGS84_E2) / (1.0 - WGS84_E2 * phi.sin().powi(2)).powf(1.5);

    let h = (x_phi * x_phi + y_phi * y_phi).sqrt() / r_phi;
    let k = (x_lam * x_lam + y_lam * y_lam).sqrt() / r_lam;

    let a_prime = (h * h + k * k + 2.0 * (x_lam * y_phi - y_lam * x_phi) / (r_lam * r_phi)).max(0.0).sqrt();
    let b_prime = (h * h + k * k - 2.0 * (x_lam * y_phi - y_lam * x_phi) / (r_lam * r_phi)).max(0.0).sqrt();

    let a = (a_prime + b_prime) / 2.0;
    let b = (a_prime - b_prime).abs() / 2.0;
    let area = h * k;
    let max_angle = if a + b > 1e-9 {
        2.0 * ((a - b) / (a + b)).asin().to_degrees()
    } else {
        0.0
    };

    Some(Distortion {
        h,
        k,
        a,
        b,
        area,
        max_angle_deg: max_angle,
    })
}

/// Discrete Global Grid System (DGGS) A5/H3 hierarchical indexing support.
pub struct Dggs;

impl Dggs {
    /// Nominal area of a DGGS cell at resolution `res` (0..=30) in square metres.
    pub fn cell_area_m2(resolution: u32) -> f64 {
        let total_earth_area = 4.0 * PI * EARTH_RADIUS_M * EARTH_RADIUS_M;
        // Resolution 0 divides the globe into 20 icosahedral faces
        // Each successive resolution level subdivides cells by a factor of 5 (A5 pentagonal/hexagonal)
        let base_cells = 20.0;
        let factor = 5.0f64.powi(resolution.min(30) as i32);
        total_earth_area / (base_cells * factor)
    }

    /// Generates boundary vertices for an approximate DGGS hexagonal/pentagonal cell given its centre and resolution.
    pub fn cell_boundary(centre: Coord, resolution: u32) -> Vec<Coord> {
        let area = Self::cell_area_m2(resolution);
        let radius_m = (area / PI).sqrt();
        let sides = 6;
        let mut boundary = Vec::with_capacity(sides + 1);

        for i in 0..sides {
            let az = (i as f64) * 360.0 / (sides as f64);
            let (lat2, lon2, _) = wgs84_direct(centre.y, centre.x, az, radius_m);
            boundary.push(Coord::new(lon2, lat2));
        }
        boundary.push(boundary[0]); // close the ring
        boundary
    }
}

/// Buffers a single point by `distance_m` on the geodetic manifold, returning a polygon ring.
pub fn geodesic_buffer_point(centre: Coord, distance_m: f64, num_points: usize) -> Vec<Coord> {
    let n = num_points.max(8);
    let mut ring = Vec::with_capacity(n + 1);
    for i in 0..n {
        let az = (i as f64) * 360.0 / (n as f64);
        let (lat2, lon2, _) = wgs84_direct(centre.y, centre.x, az, distance_m);
        ring.push(Coord::new(lon2, lat2));
    }
    ring.push(ring[0]);
    ring
}

/// Splits a polyline at the antimeridian (where consecutive points jump across > 180 degrees longitude).
pub fn cut_antimeridian_line(line: &[Coord]) -> Vec<Vec<Coord>> {
    if line.len() < 2 {
        return vec![line.to_vec()];
    }
    let mut parts = Vec::new();
    let mut current = Vec::new();
    current.push(line[0]);

    for window in line.windows(2) {
        let p1 = window[0];
        let p2 = window[1];
        let dlon = p2.x - p1.x;

        if dlon.abs() > 180.0 {
            // Crossing antimeridian: interpolate boundary point at +-180
            let sign = if dlon > 0.0 { -1.0 } else { 1.0 };
            let cross_lon = sign * 180.0;
            let ratio = (cross_lon - p1.x) / (p2.x - (p1.x + sign * 360.0));
            let cross_lat = p1.y + ratio * (p2.y - p1.y);

            current.push(Coord::new(cross_lon, cross_lat));
            parts.push(current);

            current = Vec::new();
            current.push(Coord::new(-cross_lon, cross_lat));
        }
        current.push(p2);
    }

    if !current.is_empty() {
        parts.push(current);
    }
    parts
}
