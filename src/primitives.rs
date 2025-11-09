use crate::serialized_model::*;
use glam::*;
use truck_meshalgo::{
    prelude::*
};
use truck_modeling::*;
use std::f64::consts::PI;

pub fn cube(scale: f64) -> SerializedModel {
    let v = builder::vertex(Point3::new(-scale / 2.0, -scale / 2.0, -scale / 2.0));
    let e = builder::tsweep(&v, Vector3::new(scale, 0.0, 0.0));
    let f = builder::tsweep(&e, Vector3::new(0.0, scale, 0.0));
    let cube = builder::tsweep(&f, Vector3::new(0.0, 0.0, scale));

    get_model(&cube)
}

pub fn sphere(scale: f64) -> SerializedModel {
    let v0 = builder::vertex(Point3::new(0.0, scale/2.0, 0.0));
    let wire: Wire = builder::rsweep(&v0, Point3::origin(), Vector3::unit_x(), Rad(PI));
    let shell = builder::cone(&wire, Vector3::unit_y(), Rad(7.0));
    let sphere = Solid::new(vec![shell]);

    get_model(&sphere)
}

pub fn cylinder(height: f64, radius: f64) -> SerializedModel {
   let vertex = builder::vertex(Point3::new(0.0, -height / 2.0, radius));
    let circle = builder::rsweep(&vertex, Point3::origin(), Vector3::unit_y(), Rad(7.0));
    let disk = builder::try_attach_plane(&[circle]).unwrap();
    let cylinder = builder::tsweep(&disk, Vector3::new(0.0, height, 0.0));

    get_model(&cylinder)
}

fn get_model(solid: &Solid) -> SerializedModel {
    let mut polygon_mesh = solid.triangulation(0.005).to_polygon();
    let triangulation = polygon_mesh.triangulate();
    let mut result = SerializedModel::new();
    result.meshes.push(SerializedMesh::new());

    for p in triangulation.positions() {
        result.meshes[0]
            .positions
            .push([p.x as f32, p.y as f32, p.z as f32]);
    }

    for n in triangulation.normals() {
        result.meshes[0]
            .normals
            .push([n.x as f32, n.y as f32, n.z as f32]);
    }

    for t in triangulation.uv_coords() {
        result.meshes[0].uvs.push([t.x as f32, t.y as f32])
    }

    let faces = triangulation.faces();

    let triangles = faces.tri_faces();

    for t in triangles {
        result.meshes[0].indices.push(t[0].pos as u32);
        result.meshes[0].indices.push(t[1].pos as u32);
        result.meshes[0].indices.push(t[2].pos as u32);
    }

    result.meshes[0].scale = [1.0, 1.0, 1.0];
    result.meshes[0].rotation = glam::Quat::IDENTITY.to_array();

    result
}
