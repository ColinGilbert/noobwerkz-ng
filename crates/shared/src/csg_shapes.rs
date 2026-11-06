use crate::serialized_model3d::*;

use glam::*;
use monstertruck::meshing::tessellation::{MeshableShape, MeshedShape};
use monstertruck::modeling::*;
use rerecast::AreaType;

pub fn cube(scale: f64) -> Solid {
    let v = builder::vertex(Point3::new(-scale / 2.0, -scale / 2.0, -scale / 2.0));
    let e = builder::extrude(&v, Vector3::new(scale, 0.0, 0.0));
    let f = builder::extrude(&e, Vector3::new(0.0, scale, 0.0));
    let cube = builder::extrude(&f, Vector3::new(0.0, 0.0, scale));

    cube
}

pub fn cuboid(x: f64, y: f64, z: f64) -> Solid {
    let v = builder::vertex(Point3::new(-x / 2.0, -y / 2.0, -z / 2.0));
    let e = builder::extrude(&v, Vector3::new(x, 0.0, 0.0));
    let f = builder::extrude(&e, Vector3::new(0.0, y, 0.0));
    let cuboid = builder::extrude(&f, Vector3::new(0.0, 0.0, z));

    cuboid
}

pub fn sphere(radius: f64) -> Solid {
    let v0 = builder::vertex(Point3::new(0.0, radius, 0.0));
    let wire = builder::revolve(
        &v0,
        Point3::origin(),
        Vector3::unit_x(),
        builder::SweepAngle::Closed,
        3,
    );
    let shell = builder::revolve_wire(
        &wire,
        Point3::origin(),
        Vector3::unit_y(),
        builder::SweepAngle::Closed,
        3,
    );
    let sphere = Solid::new(vec![shell]);

    sphere
}

pub fn cylinder(height: f64, radius: f64) -> Solid {
    let vertex = builder::vertex(Point3::new(0.0, -height / 2.0, radius));
    let circle = builder::revolve(
        &vertex,
        Point3::origin(),
        Vector3::unit_y(),
        builder::SweepAngle::Closed,
        3,
    );
    let disk = builder::try_attach_plane(&[circle]).unwrap();
    let cylinder = builder::extrude(&disk, Vector3::new(0.0, height, 0.0));

    cylinder
}

pub fn cone(height: f64, radius: f64) -> Solid {
    let v0 = builder::vertex(Point3::new(0.0, height / 2.0, 0.0));
    let v1 = builder::vertex(Point3::new(0.0, -height / 2.0, radius));
    let v2 = builder::vertex(Point3::new(0.0, -height / 2.0, 0.0));
    let wire = vec![builder::line(&v0, &v1), builder::line(&v1, &v2)].into();
    let shell = builder::revolve_wire(
        &wire,
        Point3::origin(),
        Vector3::unit_y(),
        builder::SweepAngle::Closed,
        3,
    );
    let cone = Solid::new(vec![shell]);

    cone
}

pub fn capsule_y(length: f64, radius: f64) -> Solid {
    let residual_length = length - 2.0 * radius;

    let v0 = builder::vertex(Point3::new(0.0, length / 2.0, 0.0));
    let v1 = builder::vertex(Point3::new(0.0, residual_length / 2.0, radius));
    let v2 = builder::vertex(Point3::new(0.0, -residual_length / 2.0, radius));
    let v3 = builder::vertex(Point3::new(0.0, -length / 2.0, 0.0));
    let arc1 = builder::circle_arc(
        &v0,
        &v1,
        Point3::new(
            0.0,
            (residual_length / 2.0) + (libm::sqrt(2.0) / 2.0) * radius,
            (libm::sqrt(2.0) / 2.0) * radius,
        ),
    );
    let arc2 = builder::circle_arc(
        &v2,
        &v3,
        Point3::new(
            0.0,
            -(residual_length / 2.0) - (libm::sqrt(2.0) / 2.0) * radius,
            libm::sqrt(2.0) / 2.0 * radius,
        ),
    );

    let wire = vec![arc1, builder::line(&v1, &v2), arc2].into();
    let shell = builder::revolve_wire(
        &wire,
        Point3::origin(),
        Vector3::unit_y(),
        builder::SweepAngle::Closed,
        3,
    );
    let capsule = Solid::new(vec![shell]);

    capsule
}

pub fn get_model(solid: &Solid) -> SerializedModel3D {
    let mut result = SerializedModel3D::new();
    result.meshes.push(SerializedMesh3D::new());
    let polygon_mesh = solid.triangulation(0.005).to_polygon();

    for f in polygon_mesh.face_iter() {
        for ff in f {
            result.meshes[0].indices.push(ff.pos as u32);
        }
    }
    let mut max_extents = [0.0 as f32; 3];
    let mut min_extents = [0.0 as f32; 3];
    for p in polygon_mesh.positions() {
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

    for n in polygon_mesh.normals() {
        result.meshes[0]
            .normals
            .push([n.x as f32, n.y as f32, n.z as f32]);
    }

    for t in polygon_mesh.uv_coords() {
        result.meshes[0].uvs.push([t.x as f32, t.y as f32])
    }

    result.meshes[0].scale = [1.0, 1.0, 1.0];
    result.meshes[0].rotation = glam::Quat::IDENTITY.to_array();

    result
}

pub fn get_rerecast_mesh(solid: &Solid) -> rerecast::TriMesh {
    let mut result = rerecast::TriMesh::default();
    let polygon_mesh = solid.triangulation(0.005).to_polygon();
    for f in polygon_mesh.face_iter() {
        let face = glam::UVec3::new(f[0].pos as u32, f[1].pos as u32, f[2].pos as u32);
        result.indices.push(face);
        result.area_types.push(AreaType::NOT_WALKABLE);
    }

    for p in polygon_mesh.positions() {
        result.vertices.push(glam::Vec3A::from_array([
            p.x as f32, p.y as f32, p.z as f32,
        ]));
    }
    result
}
