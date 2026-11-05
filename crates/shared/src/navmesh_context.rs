use rerecast::{Aabb3d, BuildContoursFlags};
use serde::{Deserialize};
use std::collections::HashMap;
pub struct NavMeshContext {
    pub recast_config: rerecast::Config,
    pub navmesh: polyanya::Mesh,
    pub terrain_bodies: HashMap<rapier3d::dynamics::RigidBodyHandle, rerecast::TriMesh>,
}

impl NavMeshContext {
    pub fn new(json_file_contents: String) -> Self {
        let deserialized_config: NavMeshConfig =
            serde_json::from_str(json_file_contents.as_str()).unwrap();
        let recast_config = rerecast::Config {
            width: deserialized_config.width,
            height: deserialized_config.height,
            tile_size: deserialized_config.tile_size,
            border_size: deserialized_config.border_size,
            cell_size: deserialized_config.cs,
            cell_height: deserialized_config.ch,
            aabb: Aabb3d {
                min: glam::Vec3::from_array(deserialized_config.bmin),
                max: glam::Vec3::from_array(deserialized_config.bmax),
            },
            walkable_slope_angle: deserialized_config.walkable_slope_angle,
            walkable_height: deserialized_config.walkable_height,
            walkable_climb: deserialized_config.walkable_climb,
            walkable_radius: deserialized_config.walkable_radius,
            max_edge_len: deserialized_config.max_edge_len,
            max_simplification_error: deserialized_config.max_simplification_error,
            min_region_area: deserialized_config.min_region_area,
            merge_region_area: deserialized_config.merge_region_area,
            max_vertices_per_polygon: deserialized_config.max_verts_per_poly,
            detail_sample_dist: deserialized_config.detail_sample_dist,
            detail_sample_max_error: deserialized_config.detail_sample_max_error,
            contour_flags: BuildContoursFlags::default(),
            area_volumes: Vec::new(),
        };
        Self {
            recast_config,
            navmesh: polyanya::Mesh::default(),
            terrain_bodies: HashMap::new(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
struct NavMeshConfig {
    pub width: u16,
    pub height: u16,
    #[serde(rename = "tileSize")]
    pub tile_size: u16,
    #[serde(rename = "borderSize")]
    pub border_size: u16,
    pub cs: f32,
    pub ch: f32,
    pub bmin: [f32; 3],
    pub bmax: [f32; 3],
    #[serde(rename = "walkableSlopeAngle")]
    pub walkable_slope_angle: f32,
    #[serde(rename = "walkableHeight")]
    pub walkable_height: u16,
    #[serde(rename = "walkableClimb")]
    pub walkable_climb: u16,
    #[serde(rename = "walkableRadius")]
    pub walkable_radius: u16,
    #[serde(rename = "maxEdgeLen")]
    pub max_edge_len: u16,
    #[serde(rename = "maxSimplificationError")]
    pub max_simplification_error: f32,
    #[serde(rename = "minRegionArea")]
    pub min_region_area: u16,
    #[serde(rename = "mergeRegionArea")]
    pub merge_region_area: u16,
    #[serde(rename = "maxVertsPerPoly")]
    pub max_verts_per_poly: u16,
    #[serde(rename = "detailSampleDist")]
    pub detail_sample_dist: f32,
    #[serde(rename = "detailSampleMaxError")]
    pub detail_sample_max_error: f32,
}
