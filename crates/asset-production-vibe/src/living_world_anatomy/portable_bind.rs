//! Strict optional world-space bind landmarks for portable anatomy scaffolds.
use qualia_core_db::render::skeletal::fauna_rig::FaunaBiomechanicalArchetype;
use qualia_core_db::render::vertebrate_anatomy::authored_skin::{
    AuthoredSkinBinding, AUTHORED_JOINT_COUNT,
};
use serde_json::Value;

const NAMES: [&str; AUTHORED_JOINT_COUNT] = [
    "pelvis",
    "spine",
    "chest",
    "neck",
    "head",
    "jaw",
    "left_hip",
    "left_knee",
    "left_ankle",
    "right_hip",
    "right_knee",
    "right_ankle",
    "left_shoulder",
    "left_elbow",
    "left_wrist",
    "tail_base",
    "tail_mid",
    "tail_tip",
    "right_shoulder",
    "right_elbow",
    "right_wrist",
];

pub(super) fn parse(
    value: Option<&Value>,
    archetype: FaunaBiomechanicalArchetype,
) -> Result<Option<AuthoredSkinBinding>, String> {
    let Some(value) = value else {
        return Ok(None);
    };
    let object = value
        .as_object()
        .ok_or("bind_landmarks must be a named world-position map")?;
    if object.len() != NAMES.len() || object.keys().any(|name| !NAMES.contains(&name.as_str())) {
        return Err(
            "bind_landmarks requires exactly the 21 named anatomy joints, including both forelimbs"
                .into(),
        );
    }
    let mut positions = [[0.0; 3]; AUTHORED_JOINT_COUNT];
    for (i, name) in NAMES.iter().enumerate() {
        let values = object
            .get(*name)
            .and_then(Value::as_array)
            .filter(|a| a.len() == 3)
            .ok_or_else(|| format!("bind_landmarks.{name} requires [x,y,z]"))?;
        for axis in 0..3 {
            positions[i][axis] = values[axis]
                .as_f64()
                .ok_or_else(|| format!("bind_landmarks.{name} must be numeric"))?;
        }
    }
    AuthoredSkinBinding::from_landmarks(archetype, &positions).map(Some)
}

#[cfg(test)]
mod knee_loft_regression {
    use qualia_core_db::render::vertebrate_anatomy::{
        AnatomicalLoftRing, AnatomyMeshAccumulator, ZoologicalAnatomicalLayer,
    };

    #[test]
    fn authored_wallaby_knee_has_continuous_ring_frames() {
        let centers = [
            [0.068, 0.335, -0.115], [0.078, 0.278, -0.068],
            [0.078, 0.222, -0.025], [0.072, 0.152, -0.082],
            [0.066, 0.086, -0.136],
        ];
        let rings: Vec<_> = centers.into_iter().map(|center| AnatomicalLoftRing {
            center, radii: [0.03, 0.03], dorsal_color: [1.0; 4],
            flank_color: [1.0; 4], ventral_color: [1.0; 4],
            primary_joint: 0, secondary_joint: 1, primary_weight: 0.8,
        }).collect();
        let mut mesh = AnatomyMeshAccumulator::new();
        mesh.push_countershaded_loft(&rings, 32, ZoologicalAnatomicalLayer::ExternalIntegument, true);
        for i in 0..rings.len()-1 {
            let a = mesh.normals[i*33];
            let b = mesh.normals[(i+1)*33];
            let dot = a[0]*b[0]+a[1]*b[1]+a[2]*b[2];
            assert!(dot > 0.5, "authored knee rings {i} and {} flipped: {dot}", i+1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn incomplete_or_unknown_bind_landmarks_refuse() {
        let archetype = FaunaBiomechanicalArchetype::MacropodSaltatorial;
        assert!(parse(Some(&serde_json::json!({"pelvis": [0,0.3,0]})), archetype).is_err());
        assert!(parse(Some(&serde_json::json!([])), archetype).is_err());
        assert!(parse(None, archetype).unwrap().is_none());
    }

    #[test]
    fn portable_authored_bind_seals_bilateral_weights_and_twenty_one_joints() {
        let mut landmarks = serde_json::Map::new();
        for (i, name) in NAMES.iter().enumerate() {
            landmarks.insert(
                (*name).into(),
                serde_json::json!([0.0, 0.3 + i as f64 * 0.01, 0.0]),
            );
        }
        landmarks.insert(
            "left_shoulder".into(),
            serde_json::json!([0.052, 0.438, 0.102]),
        );
        landmarks.insert(
            "left_elbow".into(),
            serde_json::json!([0.058, 0.365, 0.088]),
        );
        landmarks.insert(
            "right_shoulder".into(),
            serde_json::json!([-0.052, 0.438, 0.102]),
        );
        landmarks.insert(
            "right_elbow".into(),
            serde_json::json!([-0.058, 0.365, 0.088]),
        );
        let scaffold = serde_json::json!({
            "bind_landmarks": landmarks,
            "submeshes": [{"id":"forelimbs", "layer":"integument", "lofts":[{
                "radial_segs":8, "mirror_x":true, "joint1":"left_shoulder", "weight":1.0,
                "stations":[
                    {"center":[0.052,0.438,0.102],"radii":[0.01,0.01]},
                    {"center":[0.058,0.365,0.088],"radii":[0.01,0.01]}
                ]
            }]}]
        });
        let compiled = super::super::build_anatomy_from_portable_scaffold(
            &scaffold,
            FaunaBiomechanicalArchetype::MacropodSaltatorial,
            0.0,
            0.0,
            1.0,
            0.0,
        )
        .unwrap();
        let sealed = qualia_core_db::container_10d::skeletal_container::read_skeletal_section(
            &compiled.container_bytes,
        )
        .unwrap();
        assert_eq!(sealed.counts().joints, AUTHORED_JOINT_COUNT);
        assert_eq!(sealed.counts().vertices, compiled.vertex_count);
        let mut left = 0;
        let mut right = 0;
        for i in 0..compiled.vertex_count {
            let skin = sealed.skin(i).unwrap();
            assert_eq!(skin.joints, compiled.skin_joint_indices[i]);
            assert_eq!(skin.weights, compiled.skin_weights[i]);
            if compiled.positions[i][0] > 0.0 {
                assert_eq!(skin.joints[0], 12);
                left += 1;
            } else {
                assert_eq!(skin.joints[0], 18);
                right += 1;
            }
        }
        assert_eq!(left, right);
        assert!(left > 0);
        assert_eq!(
            compiled.authored_skin.as_ref().unwrap().joints.len(),
            sealed.counts().joints
        );
        // Decode the actual stored tracks, not the in-memory authoring rig. The
        // adjacent bones must still agree on each shared world-space endpoint.
        use qualia_core_db::container_10d::skeletal_section::clip_view;
        use qualia_core_db::render::skeletal::{compose_palette_into, sample_pose_into, Motor};
        let joints: Vec<_> = (0..sealed.counts().joints)
            .map(|i| sealed.joint(i).unwrap())
            .collect();
        let tracks: Vec<_> = (0..sealed.counts().tracks)
            .map(|i| sealed.track(i).unwrap())
            .collect();
        let keys: Vec<_> = (0..sealed.counts().keys)
            .map(|i| sealed.key(i).unwrap())
            .collect();
        let clip = clip_view(sealed.clip(0).unwrap(), &tracks, &keys).unwrap();
        for time in [0.0, 1.0 / 6.0, 1.0 / 3.0, 0.5, 2.0 / 3.0] {
            let mut local = vec![Motor::identity(); joints.len()];
            let mut globals = local.clone();
            let mut palette = local.clone();
            sample_pose_into(&joints, clip, time, None, 1.0, &mut local).unwrap();
            compose_palette_into(
                &joints,
                &local,
                Motor::identity(),
                &mut globals,
                &mut palette,
            )
            .unwrap();
            for (i, joint) in joints.iter().enumerate() {
                if let Some(parent) = joint.parent {
                    let landmark = &scaffold["bind_landmarks"][NAMES[i]];
                    let p = std::array::from_fn(|axis| landmark[axis].as_f64().unwrap());
                    let a = palette[i].transform_point(p);
                    let b = palette[parent as usize].transform_point(p);
                    for axis in 0..3 {
                        assert!(
                            (a[axis] - b[axis]).abs() < 1e-6,
                            "sealed joint {i} detached at {time}"
                        );
                    }
                }
            }
        }
    }
}
