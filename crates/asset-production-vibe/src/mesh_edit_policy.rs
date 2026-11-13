//! Explicit admission for edits whose geometry kernels do not transport surface attributes.
use super::*;

pub(crate) fn edits_geometry(id: &str) -> bool {
    matches!(id,
        "hypermedia.manifold.boolean_3d" | "hypermedia.manifold.simplify"
        | "hypermedia.manifold.radial_euler" | "hypermedia.manifold.bevel"
        | "hypermedia.manifold.inset" | "hypermedia.manifold.solidify"
        | "hypermedia.manifold.bisect_plane" | "hypermedia.manifold.subdivide_catmull_clark"
        | "hypermedia.manifold.evaluate_field" | "hypermedia.manifold.realize_instances"
        | "hypermedia.manifold.mesh_to_sdf_brick"
        | "hypermedia.manifold.sweep_assembly_profile" | "hypermedia.surface.unwrap_slim")
}

impl AssetHost<'_> {
    pub(crate) fn prepare_geometry_edit(&mut self, id: &str, args: &Args) -> Result<Option<Args>, String> {
        if !edits_geometry(id) { return Ok(None); }
        let policy = args.get("attribute_policy").map(text).transpose()?.unwrap_or("preserve");
        if !matches!(policy, "preserve" | "discard") {
            return Err("attribute_policy must be preserve or discard".into());
        }
        let mut stack: Vec<&Value> = args.values().collect();
        let mut visited = 0usize;
        let mut attributed = false;
        while let Some(value) = stack.pop() {
            visited += 1;
            if visited > 100_000 { return Err("Geometry edit argument traversal exceeds budget".into()); }
            match value {
                Value::Record(rec) => {
                    if rec.get("type").and_then(|v| text(v).ok()) == Some("mesh") && rec.contains_key("handle") {
                        self.workspace.get(value)?;
                        let index = integer(required(rec, "handle")?, "handle")?;
                        if let Some(Some(source)) = self.workspace.imported.get(index) {
                            // Untextured atlas remapping has an explicit vertex correspondence.
                            if id != "hypermedia.surface.unwrap_slim" || !source.texture_dependencies.is_empty() {
                                attributed = true;
                            }
                        }
                    } else { stack.extend(rec.values()); }
                }
                Value::List(values) => stack.extend(values),
                _ => {}
            }
        }
        if attributed && policy == "preserve" {
            return Err(format!("{id} cannot preserve this mesh's surface attributes with its current geometry-only kernel. Keep the imported asset for compilation, or explicitly set attribute_policy=discard on a new geometry revision and rebind/bake its surface afterwards. The source handle is unchanged."));
        }
        if !args.contains_key("attribute_policy") { return Ok(None); }
        let mut prepared = Args::new();
        for (key, value) in args {
            if key == "attribute_policy" { continue; }
            prepared.insert(key.clone(), if policy == "discard" {
                self.geometry_only_copy(value, 0)?
            } else { value.clone() });
        }
        Ok(Some(prepared))
    }

    fn geometry_only_copy(&mut self, value: &Value, depth: usize) -> Result<Value, String> {
        if depth > 32 { return Err("Geometry edit nesting exceeds 32 levels".into()); }
        match value {
            Value::Record(rec) => {
                if rec.get("type").and_then(|v| text(v).ok()) == Some("mesh") && rec.contains_key("handle") {
                    let index = integer(required(rec, "handle")?, "handle")?;
                    if self.workspace.imported.get(index).is_some_and(Option::is_some) {
                        let Artifact::Mesh(mesh) = self.workspace.get(value)? else { return Err("Expected mesh".into()); };
                        return self.workspace.insert(Artifact::Mesh(mesh.clone()), self.limits);
                    }
                    return Ok(value.clone());
                }
                let mut copy = BTreeMap::new();
                for (key, item) in rec { copy.insert(key.clone(), self.geometry_only_copy(item, depth+1)?); }
                Ok(Value::Record(copy))
            }
            Value::List(values) => values.iter().map(|v| self.geometry_only_copy(v, depth+1)).collect::<Result<Vec<_>,_>>().map(Value::List),
            _ => Ok(value.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn attributed_edit_requires_explicit_new_geometry_revision() {
        let limits = ToolLimits::default(); let cancel = AtomicBool::new(false);
        let mut host = AssetHost { workspace:AssetWorkspace::default(), inputs:&[], limits:&limits,cancel:&cancel,work:0 };
        let mesh=Mesh { positions:vec![[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],triangles:vec![[0,1,2]],min:[0.;3],max:[1.,1.,0.] };
        let handle=host.workspace.insert(Artifact::Mesh(mesh.clone()),&limits).unwrap();
        host.workspace.imported[0]=Some(Arc::new(qualia_core_db::render::assets::ImportedMesh {
            mesh,normals:Some(vec![[0.,0.,1.];3]),tangents:None,
            texture_coordinates_0:Some(vec![[0.,0.],[1.,0.],[0.,1.]]), materials:vec![],texture_dependencies:vec![],submeshes:vec![],
        }));
        let mut args=Args::from([("mesh".into(),handle)]);
        assert!(host.prepare_geometry_edit("hypermedia.manifold.bevel",&args).unwrap_err().contains("attribute_policy=discard"));
        args.insert("attribute_policy".into(),Value::String("discard".into()));
        let copied=host.prepare_geometry_edit("hypermedia.manifold.bevel",&args).unwrap().unwrap();
        let index=integer(required(record(copied.get("mesh").unwrap()).unwrap(),"handle").unwrap(),"handle").unwrap();
        assert_ne!(index,0); assert!(host.workspace.imported[index].is_none());
        assert!(host.workspace.imported[0].is_some());
        assert!(!copied.contains_key("attribute_policy"));
    }
}
