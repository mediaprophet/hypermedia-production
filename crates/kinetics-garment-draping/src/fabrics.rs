//! Calibrated orthotropic textile library.
//! Provides physical fabric mechanical parameters based on physical textile testing datasets.

use crate::energy::FabricMaterial;
use serde::{Deserialize, Serialize};

pub const REFERENCE_TRACTION: f64 = 100.0; // N/m

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FabricPreset {
    pub id: String,
    pub name: String,
    pub gsm: f64,
    pub thickness_mm: f64,
    pub extension: [f64; 2],
    pub shear: f64,
    pub bending: [f64; 2],
    pub notes: String,
}

impl FabricPreset {
    pub fn to_material(&self) -> FabricMaterial {
        let density = self.gsm / 1000.0;
        let thickness = self.thickness_mm / 1000.0;

        // Convert extension % under 100 N/m to stiffness: stiffness ~ REFERENCE_TRACTION / (ext / 100)
        let warp_stiffness = (REFERENCE_TRACTION / (self.extension[0] / 100.0).max(1e-4)).clamp(10.0, 5000.0);
        let weft_stiffness = (REFERENCE_TRACTION / (self.extension[1] / 100.0).max(1e-4)).clamp(10.0, 5000.0);

        FabricMaterial {
            warp: warp_stiffness,
            weft: weft_stiffness,
            shear: self.shear,
            compression_ratio: 0.15,
            compression_transition: 0.005,
            bend_warp: self.bending[0],
            bend_weft: self.bending[1],
            density,
            thickness,
            grain_degrees: 0.0,
        }
    }
}

pub fn standard_fabrics() -> Vec<FabricPreset> {
    vec![
        FabricPreset {
            id: "cotton".to_string(),
            name: "Light Woven Cotton".to_string(),
            gsm: 99.6,
            thickness_mm: 0.19,
            extension: [3.0, 5.0],
            shear: 120.0,
            bending: [0.00003, 0.00002],
            notes: "Light plain-weave cotton shirting".to_string(),
        },
        FabricPreset {
            id: "jersey".to_string(),
            name: "Cotton-Poly Jersey".to_string(),
            gsm: 187.0,
            thickness_mm: 0.6,
            extension: [15.0, 35.0],
            shear: 25.0,
            bending: [0.000015, 0.00001],
            notes: "Interlock knit t-shirt textile".to_string(),
        },
        FabricPreset {
            id: "wool".to_string(),
            name: "Thick Woven Wool".to_string(),
            gsm: 384.6,
            thickness_mm: 1.12,
            extension: [6.0, 9.0],
            shear: 100.0,
            bending: [0.00018, 0.00012],
            notes: "Heavy suiting/overcoat wool".to_string(),
        },
        FabricPreset {
            id: "denim".to_string(),
            name: "Cotton Denim Twill".to_string(),
            gsm: 296.7,
            thickness_mm: 0.61,
            extension: [1.5, 3.0],
            shear: 300.0,
            bending: [0.0004, 0.00025],
            notes: "Sturdy 3/1 right-hand twill denim".to_string(),
        },
        FabricPreset {
            id: "leather".to_string(),
            name: "Nappa Leather".to_string(),
            gsm: 750.0,
            thickness_mm: 1.0,
            extension: [4.0, 6.0],
            shear: 500.0,
            bending: [0.001, 0.0008],
            notes: "Supple full-grain nappa garment leather".to_string(),
        },
        FabricPreset {
            id: "silk".to_string(),
            name: "Light Habotai Silk".to_string(),
            gsm: 45.0,
            thickness_mm: 0.1,
            extension: [2.5, 4.0],
            shear: 60.0,
            bending: [0.000008, 0.000005],
            notes: "Draping lightweight silk habotai".to_string(),
        },
    ]
}

pub fn get_fabric_by_id(id: &str) -> Option<FabricPreset> {
    standard_fabrics().into_iter().find(|f| f.id.eq_ignore_ascii_case(id))
}
