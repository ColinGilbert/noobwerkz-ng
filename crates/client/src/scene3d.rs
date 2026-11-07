use crate::asset_manager::AssetManager;
// This is where our game world resides.
use crate::ecs::*;
#[allow(unused)]
use crate::ecs_components_common::*;
use crate::model_material::*;

use rerecast::{DetailNavmesh, HeightfieldBuilder};

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    path::PathBuf,
    rc::Rc,
    sync::Arc,
};

use noobwerkz_shared::{
    navmesh_context::NavMeshContext, physics3d_context::*, skeleton::*, shapes::*,
};
use noobwerkz_skeletal_animation::{
    animation::Animation,
    animation_graph::{AnimationGraph, AnimationIndex, Animations},
    animation_graph_definition::AnimationGraphDefinition,
};

use rapier3d::geometry::{Group, InteractionGroups};
use simple_particles::ParticleSystem;

use crate::{
    camera3d::*, character3d_renderable::Character3DRenderable, index_types::*,
    instance3d::Instance3D, model3d_node::Model3DNode, skinned_model3d::SkinnedModel3D,
    skinned_model3d_node::SkinnedModel3DNode,
};

use rapier3d::prelude::*;
use rayon::prelude::*;

pub struct Characters3DContext {
    pub characters: Characters3D<Character3DRenderable>,
    pub skinned_model_node: SkinnedModel3DNode,
}

pub struct TerrainContext {
    pub model_node_indices: HashMap<Model3DIndex, HashSet<usize>>, // HashSet contains the indices of the model instances
}

impl TerrainContext {
    pub fn new() -> Self {
        Self {
            model_node_indices: HashMap::new(),
        }
    }
}

pub struct Scene3D {
    pub cameras: Cameras3D<Camera3D>,
    pub model_nodes: Model3DNodes<Model3DNode>,
    pub active_camera: Camera3DIndex,
    pub physics_context: Physics3DContext,
    pub characters_contexts: Vec<Characters3DContext>,
    pub character_types_by_name: HashMap<String, Characters3DContextIndex>,
    pub particle_system: Rc<RefCell<ParticleSystem>>,
    pub ecs: World,
    pub navmesh_context: NavMeshContext,
    pub terrain_context: TerrainContext,
}

impl Scene3D {
    pub fn new(gravity: &glam::Vec3, navmesh_config_path: PathBuf) -> Self {
        let config_str = std::fs::read_to_string(navmesh_config_path).unwrap();
        let navmesh_context = NavMeshContext::new(config_str);
        Self {
            cameras: Cameras3D::new(),
            model_nodes: Model3DNodes::new(),
            active_camera: Camera3DIndex::new(0),
            physics_context: Physics3DContext::new(gravity),
            characters_contexts: Vec::new(),
            character_types_by_name: HashMap::new(),
            particle_system: Rc::new(RefCell::new(ParticleSystem::new())),
            ecs: World::default(),
            navmesh_context,
            terrain_context: TerrainContext::new(),
        }
    }

    pub fn step_physics(&mut self) {
        self.physics_context.rigid_world.step()
    }

    pub fn update_particles(&mut self, dt: web_time::Duration) {
        self.particle_system.borrow_mut().update(dt.as_secs_f32());
        // println!("Particle system empty? {}", self.particle_system.borrow().is_empty());
    }

    pub fn build_navmesh(&mut self) {
        self.navmesh_context.navmesh = polyanya::Mesh::default();
        let mut trimesh = rerecast::TriMesh::default();
        for (rigid_body_handle, _rigid_body) in self.physics_context.rigid_world.rigid_bodies() {
            if self
                .navmesh_context
                .terrain_bodies
                .contains_key(&rigid_body_handle)
            {
                let temp = &self.navmesh_context.terrain_bodies[&rigid_body_handle];
                // println!("Found terrain to add to mesh! Verts {}, indices {}", temp.vertices.len(), temp.indices.len());
                // for v in &temp.vertices {
                //     trimesh.vertices.push(*v);
                // }
                // for i in &temp.indices {
                //     trimesh.indices.push(*i);
                // }
                // for a in &temp.area_types {
                //     trimesh.area_types.push(*a);
                // }
                trimesh.extend(temp.clone());
            }
        }
        trimesh.mark_walkable_triangles(self.navmesh_context.recast_config.walkable_slope_angle);
        let aabb = trimesh.compute_aabb().unwrap();
        println!("Navmesh AABB: {:?}", aabb);

        let mut heightfield = HeightfieldBuilder {
            aabb,
            cell_size: self.navmesh_context.recast_config.cell_size,
            cell_height: self.navmesh_context.recast_config.cell_height,
        }
        .build()
        .unwrap();

        heightfield
            .rasterize_triangles(&trimesh, self.navmesh_context.recast_config.walkable_climb)
            .unwrap();
        heightfield.filter_low_hanging_walkable_obstacles(
            self.navmesh_context.recast_config.walkable_climb,
        );
        heightfield.filter_ledge_spans(
            self.navmesh_context.recast_config.walkable_height,
            self.navmesh_context.recast_config.walkable_climb,
        );
        heightfield
            .filter_walkable_low_height_spans(self.navmesh_context.recast_config.walkable_height);

        let mut compact_heightfield = heightfield
            .into_compact(
                self.navmesh_context.recast_config.walkable_height,
                self.navmesh_context.recast_config.walkable_climb,
            )
            .unwrap();

        compact_heightfield.erode_walkable_area(self.navmesh_context.recast_config.walkable_radius);

        // Add obstacles...
        /*
                let volumes = load_json::<CppVolumes>(project, "convex_volumes");
                for volume in volumes.volumes {
                    let volume = ConvexVolume {
                        vertices: volume
                            .verts
                            .iter()
                            .map(|[x, _y, z]| Vec2::new(*x, *z))
                            .collect(),
                        min_y: volume.hmin,
                        max_y: volume.hmax,
                        area: AreaType::from(volume.area),
                    };
                    compact_heightfield.mark_convex_poly_area(&volume);
                }
        */

        compact_heightfield.build_distance_field();

        compact_heightfield
            .build_regions(
                self.navmesh_context.recast_config.border_size,
                self.navmesh_context.recast_config.min_region_area,
                self.navmesh_context.recast_config.merge_region_area,
            )
            .unwrap();

        let contours = compact_heightfield.build_contours(
            self.navmesh_context.recast_config.max_simplification_error,
            self.navmesh_context.recast_config.max_edge_len,
            self.navmesh_context.recast_config.contour_flags,
        );
        // compact_heightfield.assert_eq(project, "compact_heightfield_contours");

        let poly_mesh = contours
            .into_polygon_mesh(self.navmesh_context.recast_config.max_vertices_per_polygon)
            .unwrap();

        let detail_mesh = DetailNavmesh::new(
            &poly_mesh,
            &compact_heightfield,
            self.navmesh_context.recast_config.detail_sample_dist,
            self.navmesh_context.recast_config.detail_sample_max_error,
        )
        .unwrap();

        self.navmesh_context.navmesh = polyanya::RecastFullMesh::new(poly_mesh, detail_mesh)
            .try_into()
            .unwrap();

        self.navmesh_context.navmesh.bake();
    }

    pub fn add_terrain(
        &mut self,
        shape: Shape3D,
        translation: glam::Vec3,
        orientation: glam::Quat,
        asset_mgr: &mut AssetManager,
        device: &mut wgpu::Device,
        queue: &mut wgpu::Queue,
        default_material: &ModelMaterial,
        texture_layout: &wgpu::BindGroupLayout,
    ) {
        let euler = orientation.to_euler(glam::EulerRot::default());
        let rigid_body = RigidBodyBuilder::fixed()
            .translation(translation)
            .rotation(Vec3::new(euler.0, euler.1, euler.2));
        let collider = shape_to_collider(&shape);
        if collider.is_none() {
            println!("Could not add nonexistent collider to scene terrain.");
            return;
        }
        let mut collider = collider.unwrap();
        let groups = InteractionGroups::new(
            Group::GROUP_1,
            Group::GROUP_2,
            rapier3d::geometry::InteractionTestMode::And,
        );
        collider.set_collision_groups(groups);
        let (rigid_body_handle, _collider_handle) = self
            .physics_context
            .rigid_world
            .insert(rigid_body, collider);


        let serialized_model = shape_to_serialized_model(&shape);
        if serialized_model.is_none() {
            println!("Could not create serialized model for scene terrain.")
        }
        
        let mesh = &serialized_model.as_ref().unwrap().meshes[0];
        self.navmesh_context
            .terrain_bodies
            .insert(rigid_body_handle, mesh_to_recast_mesh(mesh));

        let mut texture_filepath = PathBuf::new();
        texture_filepath.push("res");
        /*

                &mut self,
        serialized: &mut SerializedModel3D,
        name: &str,
        texture_filepath: &PathBuf,
        device: &mut wgpu::Device,
        queue: &mut wgpu::Queue,
        default_material: &ModelMaterial,
        texture_layout: &wgpu::BindGroupLayout,

         */
        let terrain_model = asset_mgr.load_serialized_3d_model_from_mem(
            &mut serialized_model.unwrap(),
            "",
            &texture_filepath,
            device,
            queue,
            default_material,
            texture_layout,
        );
        match terrain_model {
            Ok(val) => {
                let mut instances = Instances::new();
                instances.push(Instance3D {
                    position: translation,
                    orientation,
                    scale: glam::Vec3::splat(1.0),
                });
                let model_node = Model3DNode::new(val, instances);
                self.model_nodes.push(model_node);
            }
            Err(err) => {
                println!("Could not add terrain model: {}", err)
            }
        }
    }

    pub fn update_characters(
        &mut self,
        dt: web_time::Duration,
        skinned_models: &SkinnedModels3D<SkinnedModel3D>,
        queue: &wgpu::Queue,
    ) {
        self.ecs.update_positions(
            &DeltaTimeResource {
                val: dt.as_secs_f32(),
            },
            &mut CharactersContextsResource::new(&mut self.characters_contexts),
        );
        // println!("Number of characters: {}", self.characters_contexts.len());
        self.characters_contexts.par_iter_mut().for_each(
            |characters_ctx: &mut Characters3DContext| {
                characters_ctx.skinned_model_node.instances.clear();
                characters_ctx.skinned_model_node.bone_matrices.clear();

                for c in &mut characters_ctx.characters {
                    let r = c.anim_graph.evaluate(dt);
                    match r {
                        Ok(_) => {}
                        Err(err) => {
                            println!("Could not evaluate character's anim graph: {}", err)
                        }
                    }
                    let instance = Instance3D {
                        position: c.common.position,
                        orientation: c.common.orientation,
                        scale: glam::Vec3::splat(1.0),
                    };

                    characters_ctx.skinned_model_node.instances.push(instance);

                    let output = &c.anim_graph.output_model;

                    if output.model_matrices.len()
                        != skinned_models[characters_ctx.skinned_model_node.skinned_model_idx]
                            .skeleton
                            .inverse_bind_matrices
                            .len()
                    {
                        println!("Discrepancy in size between output model matrices and IBM's");
                    }

                    for (i, o) in output.model_matrices.iter().enumerate() {
                        characters_ctx.skinned_model_node.bone_matrices.push(
                            o * skinned_models[characters_ctx.skinned_model_node.skinned_model_idx]
                                .skeleton
                                .inverse_bind_matrices[i],
                        );
                    }
                    let bones_storage_buffer =
                        &characters_ctx.skinned_model_node.bones_storage_buffer;

                    queue.write_buffer(
                        &bones_storage_buffer,
                        0,
                        bytemuck::cast_slice(&characters_ctx.skinned_model_node.bone_matrices),
                    );
                }
            },
        );
    }

    pub fn find_path(&self, from: glam::Vec3, to: glam::Vec3) -> Option<VecDeque<glam::Vec3>> {
        let path = self.navmesh_context.navmesh.path_3d(from, to);
        let mut results = VecDeque::new();
        match path {
            Some(val) => {
                for v in val {
                    results.push_back(v);
                }
                Some(results)
            }
            None => None,
        }
    }

    pub fn add_characters(
        &mut self,
        device: &mut wgpu::Device,
        bone_matrices_bind_group_layout: &wgpu::BindGroupLayout,
        skinned_model_idx: SkinnedModel3DIndex,
        instances: &Vec<Instance3D>,
        animgraph_definition: &AnimationGraphDefinition,
        animations: Animations<Animation>,
        skeleton: Arc<Skeleton>,
        animations_by_name: &HashMap<String, AnimationIndex>,
        name: String,
    ) -> Option<Characters3DContextIndex> {
        self.characters_contexts.push(Characters3DContext {
            characters: Characters3D::new(),
            skinned_model_node: SkinnedModel3DNode::new(
                device,
                bone_matrices_bind_group_layout,
                skinned_model_idx,
                instances,
                skeleton.clone(),
            ),
        });
        println!("ADDING CHARACTERS: Instance count: {}", instances.len());
        let characters_ctx_idx = Characters3DContextIndex::new(self.characters_contexts.len() - 1);
        for instance in instances {
            let character = Character3DRenderable::new(
                skeleton.clone(),
                instance.position.into(),
                instance.orientation,
                animgraph_definition,
                animations.clone(),
                animations_by_name,
            );
            match character {
                Some(val) => {
                    self.characters_contexts[characters_ctx_idx.get()]
                        .characters
                        .push(val);
                    let i = self.characters_contexts[characters_ctx_idx.get()]
                        .characters
                        .len()
                        - 1;
                    self.ecs.create(CharacterEntity {
                        type_idx: CharacterTypeIndexComponent {
                            val: characters_ctx_idx,
                        },
                        instance_idx: CharacterInstanceIndexComponent {
                            val: Character3DIndex::new(i),
                        },
                        position: PositionComponent {
                            val: instance.position,
                        },
                        velocity: VelocityComponent {
                            val: glam::Vec3::ZERO,
                        },
                        orientation: OrientationComponent {
                            val: instance.orientation,
                        },
                        speed: SpeedComponent { val: 1.0 },
                        path: PathComponent {
                            val: VecDeque::new(),
                        },
                        ..Default::default()
                    });
                }
                None => {
                    println!("[Scene] add_character: Could not create anim graph from definition");
                    return None;
                }
            }
        }

        self.character_types_by_name
            .insert(name, characters_ctx_idx);

        Some(characters_ctx_idx)
    }

    pub fn change_anim_graphs(
        &mut self,
        character_type_idx: Characters3DContextIndex,
        definition: &AnimationGraphDefinition,
        skeleton: Arc<Skeleton>,
        animations: Animations<Animation>,
        animations_by_name: &HashMap<String, AnimationIndex>,
    ) -> bool {
        for c in self.characters_contexts[character_type_idx.get()]
            .characters
            .iter_mut()
        {
            let updated_anim_graph = AnimationGraph::new(
                skeleton.clone(),
                animations.clone(),
                definition,
                animations_by_name,
            );
            match updated_anim_graph {
                Ok(val) => c.anim_graph = val,
                Err(err) => {
                    println!(
                        "[Scene] Trying to change character to use an anim graph with an invalid definition, {}",
                        err
                    );
                    return false;
                }
            }
        }
        true
    }
}
