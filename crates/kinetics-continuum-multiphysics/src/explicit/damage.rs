//! Johnson-Cook viscoplastic damage and element erosion criterion.
//!
//! Grounded in continuum mechanics and fracture mechanics validity (UDHR Art. 19, 27).

use serde::{Deserialize, Serialize};

/// Johnson-Cook dynamic fracture parameters.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct JohnsonCookDamage {
    pub d1: f64,
    pub d2: f64,
    pub d3: f64,
    pub d4: f64,
    pub d5: f64,
    pub ref_strain_rate: f64,
    pub melting_temperature: f64,
    pub room_temperature: f64,
}

impl JohnsonCookDamage {
    /// Fracture strain eps_f = [d1 + d2 * exp(d3 * sigma*)] * [1 + d4 * ln(eps_dot*)] * [1 + d5 * T*]
    /// where sigma* = p / q is stress triaxiality (hydrostatic / von Mises).
    pub fn fracture_strain(
        &self,
        stress_triaxiality: f64,
        strain_rate: f64,
        temperature: f64,
    ) -> f64 {
        let triaxiality_term = self.d1 + self.d2 * (self.d3 * stress_triaxiality).exp();

        let eps_dot_star = (strain_rate / self.ref_strain_rate.max(1e-6)).max(1.0);
        let rate_term = 1.0 + self.d4 * eps_dot_star.ln();

        let t_star = ((temperature - self.room_temperature)
            / (self.melting_temperature - self.room_temperature).max(1.0))
            .clamp(0.0, 1.0);
        let temp_term = 1.0 + self.d5 * t_star;

        (triaxiality_term * rate_term * temp_term).max(1e-4)
    }

    /// Update damage index D = sum(delta_eps_p / eps_f). Returns true if element is eroded (D >= 1.0).
    pub fn update_damage(
        &self,
        current_damage: f64,
        delta_plastic_strain: f64,
        stress_triaxiality: f64,
        strain_rate: f64,
        temperature: f64,
    ) -> (f64, bool) {
        let eps_f = self.fracture_strain(stress_triaxiality, strain_rate, temperature);
        let new_damage = current_damage + delta_plastic_strain / eps_f;
        let eroded = new_damage >= 1.0;
        (new_damage, eroded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn johnson_cook_damage_erosion() {
        let jc = JohnsonCookDamage {
            d1: 0.05,
            d2: 3.44,
            d3: -2.12,
            d4: 0.002,
            d5: 0.61,
            ref_strain_rate: 1.0,
            melting_temperature: 1800.0,
            room_temperature: 293.0,
        };

        let eps_f = jc.fracture_strain(0.333, 1.0, 293.0);
        assert!(eps_f > 0.0);

        let (d, eroded) = jc.update_damage(0.0, eps_f * 1.05, 0.333, 1.0, 293.0);
        assert!(eroded);
        assert!(d >= 1.0);
    }
}
