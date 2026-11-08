//! Multi-point geodesic ruler measurement stack with undo/redo and segment distance accumulation.
//!
//! Evaluates geodesic and spherical distances in pure Rust, without panics or unwraps.

use std::collections::VecDeque;

const MAX_HISTORY_STEPS: usize = 100;
const EARTH_RADIUS_KM: f64 = 6371.0088;
const WGS84_SEMI_MAJOR_KM: f64 = 6378.137;
const WGS84_FLATTENING: f64 = 1.0 / 298.257223563;

/// Surface model for geodesic distance measurement.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EarthModel {
    PerfectSphere,
    #[default]
    Wgs84,
}

impl EarthModel {
    pub const ALL: [Self; 2] = [Self::Wgs84, Self::PerfectSphere];

    pub fn code(self) -> &'static str {
        match self {
            Self::PerfectSphere => "perfect_sphere",
            Self::Wgs84 => "wgs84",
        }
    }

    pub fn distance_km(self, start: (f64, f64), end: (f64, f64)) -> f64 {
        match self {
            Self::PerfectSphere => haversine_km(start, end),
            Self::Wgs84 => vincenty_wgs84_km(start, end),
        }
    }

    pub fn central_angle_degrees(self, start: (f64, f64), end: (f64, f64)) -> f64 {
        let km = self.distance_km(start, end);
        (km / EARTH_RADIUS_KM).to_degrees()
    }
}

/// Great-circle distance between two (latitude, longitude) pairs in degrees.
pub fn haversine_km(start: (f64, f64), end: (f64, f64)) -> f64 {
    let dlat = (end.0 - start.0).to_radians();
    let dlon = (end.1 - start.1).to_radians();
    let lat1 = start.0.to_radians();
    let lat2 = end.0.to_radians();
    let a = (dlat * 0.5).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon * 0.5).sin().powi(2);
    let c = 2.0 * a.clamp(0.0, 1.0).sqrt().atan2((1.0 - a).clamp(0.0, 1.0).sqrt());
    EARTH_RADIUS_KM * c
}

/// Vincenty inverse geodesic calculation on the WGS84 ellipsoid.
pub fn vincenty_wgs84_km(start: (f64, f64), end: (f64, f64)) -> f64 {
    if (start.0 - end.0).abs() < 1e-12 && (start.1 - end.1).abs() < 1e-12 {
        return 0.0;
    }
    let a = WGS84_SEMI_MAJOR_KM;
    let f = WGS84_FLATTENING;
    let b = a * (1.0 - f);

    let phi1 = start.0.to_radians();
    let phi2 = end.0.to_radians();
    let l = (end.1 - start.1).to_radians();

    let u1 = ((1.0 - f) * phi1.tan()).atan();
    let u2 = ((1.0 - f) * phi2.tan()).atan();

    let sin_u1 = u1.sin();
    let cos_u1 = u1.cos();
    let sin_u2 = u2.sin();
    let cos_u2 = u2.cos();

    let mut lambda = l;

    for _ in 0..100 {
        let sin_lambda = lambda.sin();
        let cos_lambda = lambda.cos();
        let sin_sigma = ((cos_u2 * sin_lambda).powi(2)
            + (cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda).powi(2))
        .sqrt();
        if sin_sigma.abs() < 1e-12 {
            return 0.0;
        }
        let cos_sigma = sin_u1 * sin_u2 + cos_u1 * cos_u2 * cos_lambda;
        let sigma = sin_sigma.atan2(cos_sigma);
        let sin_alpha = cos_u1 * cos_u2 * sin_lambda / sin_sigma;
        let cos2_alpha = 1.0 - sin_alpha * sin_alpha;
        let cos2_sigma_m = if cos2_alpha.abs() > 1e-12 {
            cos_sigma - 2.0 * sin_u1 * sin_u2 / cos2_alpha
        } else {
            0.0
        };
        let c = f / 16.0 * cos2_alpha * (4.0 + f * (4.0 - 3.0 * cos2_alpha));
        let lambda_prev = lambda;
        lambda = l
            + (1.0 - c)
                * f
                * sin_alpha
                * (sigma
                    + c * sin_sigma
                        * (cos2_sigma_m
                            + c * cos_sigma * (-1.0 + 2.0 * cos2_sigma_m * cos2_sigma_m)));
        if (lambda - lambda_prev).abs() < 1e-12 {
            let u_sq = cos2_alpha * (a * a - b * b) / (b * b);
            let big_a = 1.0
                + u_sq / 16384.0 * (4096.0 + u_sq * (-768.0 + u_sq * (320.0 - 175.0 * u_sq)));
            let big_b =
                u_sq / 1024.0 * (256.0 + u_sq * (-128.0 + u_sq * (74.0 - 47.0 * u_sq)));
            let delta_sigma = big_b
                * sin_sigma
                * (cos2_sigma_m
                    + big_b / 4.0
                        * (cos_sigma * (-1.0 + 2.0 * cos2_sigma_m * cos2_sigma_m)
                            - big_b / 6.0
                                * cos2_sigma_m
                                * (-3.0 + 4.0 * sin_sigma * sin_sigma)
                                * (-3.0 + 4.0 * cos2_sigma_m * cos2_sigma_m)));
            return b * big_a * (sigma - delta_sigma);
        }
    }
    haversine_km(start, end)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RulerPointId {
    pub path: usize,
    pub point: usize,
}

pub type RulerPaths = Vec<Vec<(f64, f64)>>;

/// Multi-point geodesic ruler state with bounded undo/redo history.
#[derive(Default)]
pub struct GeodesicRuler {
    paths: RulerPaths,
    undo: VecDeque<RulerPaths>,
    redo: Vec<RulerPaths>,
    move_before: Option<RulerPaths>,
}

pub type Ruler = GeodesicRuler;

impl GeodesicRuler {
    pub fn add_point(&mut self, position: (f64, f64), start_path: bool) {
        self.end_move();
        self.record(self.paths.clone());
        if self
            .paths
            .last()
            .is_none_or(|path| start_path && !path.is_empty())
        {
            self.paths.push(Vec::new());
        }
        if let Some(path) = self.paths.last_mut() {
            path.push(position);
        }
    }

    pub fn paths(&self) -> impl Iterator<Item = &[(f64, f64)]> {
        self.paths.iter().map(Vec::as_slice)
    }

    pub fn points(&self) -> impl Iterator<Item = &(f64, f64)> {
        self.paths.iter().flatten()
    }

    pub fn point_count(&self) -> usize {
        self.paths.iter().map(Vec::len).sum()
    }

    pub fn point(&self, id: RulerPointId) -> Option<(f64, f64)> {
        self.paths.get(id.path)?.get(id.point).copied()
    }

    pub fn begin_move(&mut self) {
        self.end_move();
        self.move_before = Some(self.paths.clone());
    }

    pub fn move_point(&mut self, id: RulerPointId, position: (f64, f64)) {
        if self
            .paths
            .get(id.path)
            .and_then(|path| path.get(id.point))
            .is_none_or(|old| *old == position)
        {
            return;
        }
        if self.move_before.is_none() {
            self.record(self.paths.clone());
        }
        if let Some(path) = self.paths.get_mut(id.path) {
            if let Some(pt) = path.get_mut(id.point) {
                *pt = position;
            }
        }
    }

    pub fn end_move(&mut self) {
        if let Some(before) = self.move_before.take() {
            if before != self.paths {
                self.record(before);
            }
        }
    }

    fn record(&mut self, before: RulerPaths) {
        if self.undo.len() == MAX_HISTORY_STEPS {
            self.undo.pop_front();
        }
        self.undo.push_back(before);
        self.redo.clear();
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
            || self
                .move_before
                .as_ref()
                .is_some_and(|before| *before != self.paths)
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
            && self
                .move_before
                .as_ref()
                .is_none_or(|before| *before == self.paths)
    }

    pub fn undo(&mut self) -> bool {
        self.end_move();
        let Some(previous) = self.undo.pop_back() else {
            return false;
        };
        self.redo.push(std::mem::replace(&mut self.paths, previous));
        true
    }

    pub fn redo(&mut self) -> bool {
        self.end_move();
        let Some(next) = self.redo.pop() else {
            return false;
        };
        self.undo
            .push_back(std::mem::replace(&mut self.paths, next));
        true
    }

    pub fn remove_point(&mut self, id: RulerPointId) {
        self.end_move();
        if self
            .paths
            .get(id.path)
            .and_then(|path| path.get(id.point))
            .is_none()
        {
            return;
        }
        self.record(self.paths.clone());
        if let Some(path) = self.paths.get_mut(id.path) {
            if id.point < path.len() {
                path.remove(id.point);
                if path.is_empty() && id.path + 1 < self.paths.len() {
                    self.paths.remove(id.path);
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.end_move();
        if self.paths.is_empty() {
            return;
        }
        let before = std::mem::take(&mut self.paths);
        self.record(before);
    }

    pub fn distance_km(&self, model: EarthModel) -> f64 {
        let mut total = 0.0;
        for path in &self.paths {
            if path.len() < 2 {
                continue;
            }
            for i in 0..path.len() - 1 {
                if let (Some(&p0), Some(&p1)) = (path.get(i), path.get(i + 1)) {
                    total += model.distance_km(p0, p1);
                }
            }
        }
        total
    }

    pub fn total_distance_km(&self) -> f64 {
        self.distance_km(EarthModel::PerfectSphere)
    }

    pub fn angular_distance_degrees(&self, model: EarthModel) -> f64 {
        let mut total = 0.0;
        for path in &self.paths {
            if path.len() < 2 {
                continue;
            }
            for i in 0..path.len() - 1 {
                if let (Some(&p0), Some(&p1)) = (path.get(i), path.get(i + 1)) {
                    total += model.central_angle_degrees(p0, p1);
                }
            }
        }
        total
    }

    pub fn last_leg(&self) -> Option<[(f64, f64); 2]> {
        let path = self.paths.last()?;
        if path.len() < 2 {
            return None;
        }
        let p0 = *path.get(path.len() - 2)?;
        let p1 = *path.last()?;
        Some([p0, p1])
    }
}
