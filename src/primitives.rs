use crate::serialized_model::*;
use glam::*;
use std::f64::consts::PI;
use truck_meshalgo::prelude::*;
use truck_modeling::*;

pub fn cube(scale: f64) -> SerializedModel {
    let v = builder::vertex(Point3::new(-scale / 2.0, -scale / 2.0, -scale / 2.0));
    let e = builder::tsweep(&v, Vector3::new(scale, 0.0, 0.0));
    let f = builder::tsweep(&e, Vector3::new(0.0, scale, 0.0));
    let cube = builder::tsweep(&f, Vector3::new(0.0, 0.0, scale));

    get_model(&cube)
}

pub fn cuboid(x: f64, y: f64, z: f64) -> SerializedModel {
    let v = builder::vertex(Point3::new(-x / 2.0, -y / 2.0, -z / 2.0));
    let e = builder::tsweep(&v, Vector3::new(x, 0.0, 0.0));
    let f = builder::tsweep(&e, Vector3::new(0.0, y, 0.0));
    let cuboid = builder::tsweep(&f, Vector3::new(0.0, 0.0, z));

    get_model(&cuboid)
}

pub fn sphere(scale: f64) -> SerializedModel {
    let v0 = builder::vertex(Point3::new(0.0, scale / 2.0, 0.0));
    let wire = builder::rsweep(&v0, Point3::origin(), Vector3::unit_x(), Rad(PI));
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

pub fn cone(height: f64, radius: f64) -> SerializedModel {
    let v0 = builder::vertex(Point3::new(0.0, height / 2.0, 0.0));
    let v1 = builder::vertex(Point3::new(0.0, -height / 2.0, radius));
    let v2 = builder::vertex(Point3::new(0.0, -height / 2.0, 0.0));
    let wire = vec![builder::line(&v0, &v1), builder::line(&v1, &v2)].into();
    let shell = builder::cone(&wire, Vector3::unit_y(), Rad(7.0));
    let cone = Solid::new(vec![shell]);

    get_model(&cone)
}

pub fn capsule(height: f64, radius: f64) -> SerializedModel {
    // let v0 = builder::vertex(Point3::new(0.0, height / 2.0, 0.0));
    // let v1 = builder::vertex(Point3::new(0.0, (height / 2.0) - radius, radius ));
    // let v2 = builder::vertex(Point3::new(0.0, (-height / 2.0) + radius, radius ));
    // let v3 = builder::vertex(Point3::new(0.0, -height / 2.0, 0.0));
    // let arc1 = builder::circle_arc(&v0, &v1, Point3::new(0.0, (height / 2.0) - (libm::sqrt(2.0)/2.0)*radius, (libm::sqrt(2.0)/2.0)*radius));
    // let arc2 = builder::circle_arc(&v2, &v3, Point3::new(0.0, (-height / 2.0) + (libm::sqrt(2.0)/2.0)*radius, (libm::sqrt(2.0)/2.0)*radius));

    let v0 = builder::vertex(Point3::new(0.0, 0.5 , 0.0));
    let v1 = builder::vertex(Point3::new(0.0, 0.5, 0.5));
    let v2 = builder::vertex(Point3::new(0.0, -0.5, 0.5));
    let v3 = builder::vertex(Point3::new(0.0, -0.5, 0.0));
    let arc1 = builder::circle_arc(
        &v0,
        &v1,
        Point3::new(
            0.0,
            libm::sqrt(2.0) / 2.0,
            libm::sqrt(2.0) / 2.0,
        ),
    );
    let arc2 = builder::circle_arc(
        &v2,
        &v3,
        Point3::new(
            0.0,
            libm::sqrt(2.0) / 2.0,
            libm::sqrt(2.0) / 2.0,
        ),
    );

    let wire = vec![arc1, builder::line(&v1, &v2), arc2].into();
    let shell = builder::cone(&wire, Vector3::unit_y(), Rad(7.0));
    let capsule = Solid::new(vec![shell]);

    get_model(&capsule)
}

pub fn get_model(solid: &Solid) -> SerializedModel {
    let mut polygon_mesh = solid.triangulation(0.005).to_polygon();
    let triangulation = polygon_mesh.triangulate();
    let mut result = SerializedModel::new();
    result.meshes.push(SerializedMesh::new());
    let mut max_extents = [0.0 as f32; 3];
    let mut min_extents = [0.0 as f32; 3];
    for p in triangulation.positions() {
        result.meshes[0]
            .positions
            .push([p.x as f32, p.y as f32, p.z as f32]);
        let mut i = 0;
        while i < 3 {
            let biggest: f32;
            if max_extents[i] > p[i] as f32 {
                biggest = max_extents[i] as f32;
            } else {
                biggest = p[i] as f32;
            }
            max_extents[i] = biggest;

            let smallest: f32;
            if min_extents[i] < p[i] as f32 {
                smallest = min_extents[i];
            } else {
                smallest = p[i] as f32;
            }
            min_extents[i] = smallest;

            i += 1;
        }
    }

    result.meshes[0].max_extents = max_extents;
    result.meshes[0].min_extents = min_extents;

    let mut dims = [0.0 as f32; 3];
    let mut i = 0;
    while i < 3 {
        dims[i] = max_extents[i] - min_extents[i];
        i += 1;
    }
    result.meshes[0].dimensions = dims;

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
