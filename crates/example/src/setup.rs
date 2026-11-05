use noobwerkz_client::camera3d::*;
use noobwerkz_client::client_user_context::ClientUserContext;
// use noobwerkz_client::glam::Vec3;
use noobwerkz_client::graphics::*;
use noobwerkz_client::index_types::*;
use noobwerkz_client::instance3d::*;
use noobwerkz_client::light3d::Light3DUniform;
use noobwerkz_client::model3d_node::*;
use noobwerkz_client::scene3d::*;
use noobwerkz_client::simple_particles::ParticleEffect;
use noobwerkz_skeletal_animation::animation_graph_definition::*;
// use noobwerkz::simple_animgraph::edge_definitions::TransitionDefinition;
use crate::user_data::*;

// use image::*;

// MODIFY ME!
pub fn user_setup(
    gfx_ctx: &mut GraphicsContext,
    user_ctx: &mut ClientUserContext,
    lights: &mut Vec<Light3DUniform>,
) {
    let u = user_ctx;
    let user_data = USER_DATA
        .lock()
        .expect("UserData poisoned during user_update()");

    if user_data.works {
        println!("User data initialized");
    }
    let projection = Projection3D::new(
        gfx_ctx.config.height,
        gfx_ctx.config.width,
        degrees_to_radians(45.0),
        0.01,
        100.0,
    );
    
    let mut path = std::path::PathBuf::new();
    path.push("res");
    let mut navmesh_config_path = path.clone();
    navmesh_config_path.push("navmesh_config.json");

    let s = Scene3D::new(&noobwerkz_client::glam::Vec3 {
        x: 0.0,
        y: -9.81,
        z: 0.0,
    }, navmesh_config_path);

    let current_scene = u.add_scene(s);

    let c = Camera3D::new(
        &noobwerkz_client::glam::Vec3 {
            x: 0.0,
            y: 1.0,
            z: 3.0,
        },
        &noobwerkz_client::glam::Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        &noobwerkz_client::glam::Vec3::Y,
        0.1,
        0.1,
        projection,
    );

    let mut cesium_man_path = path.clone();
    cesium_man_path.push("cesium-man-model.bin");

    const SPACE_BETWEEN: f32 = 1.6;
    let mut cesium_man_instances = Vec::<Instance3D>::new();
    for outer in 0..10 {
        for inner in 0..10 {
            cesium_man_instances.push(Instance3D {
                position: noobwerkz_client::glam::Vec3::from_array([
                    SPACE_BETWEEN * outer as f32,
                    3.0,
                    SPACE_BETWEEN * inner as f32,
                ]),
                orientation: noobwerkz_client::glam::Quat::IDENTITY, //from_rotation_z(degrees_to_radians(90.0)) * noobwerkz_client::glam::Quat::from_rotation_x(degrees_to_radians(-90.0)),
                scale: noobwerkz_client::glam::Vec3::splat(1.0),
            });
        }
    }

    let mut cesium_man_animgraph_definition = AnimationGraphDefinition::new();
    let root_node_def = GenericNodeDefinition::Sampler(SamplerNodeDefinition {
        speed: 1.0,
        animation_name: "0".to_owned(),
        looping: true,
        name: "walk".to_owned(),
    });

    let root = cesium_man_animgraph_definition
        .graph
        .add_node(root_node_def);
    cesium_man_animgraph_definition.root = Some(root);

    u.load_character(
        &cesium_man_path,
        "CesiumMan",
        current_scene,
        &cesium_man_instances,
        &cesium_man_animgraph_definition,
        &mut gfx_ctx.device,
        &mut gfx_ctx.queue,
        &gfx_ctx.debug_material,
        &gfx_ctx.texture_bind_group_layout,
        &gfx_ctx.bone_matrices_bind_group_layout,
    );

    let mut king_path = path.clone();
    king_path.push("King.bin");

    let mut king_instances = Vec::<Instance3D>::new();
    for outer in 0..10 {
        for inner in 0..10 {
            king_instances.push(Instance3D {
                position: noobwerkz_client::glam::Vec3::from_array([
                    SPACE_BETWEEN * outer as f32,
                    -2.0,
                    SPACE_BETWEEN * inner as f32,
                ]),
                orientation: noobwerkz_client::glam::Quat::IDENTITY, //from_rotation_x(degrees_to_radians(-90.0)) * noobwerkz_client::glam::Quat::from_rotation_z(degrees_to_radians(-180.0)),
                scale: noobwerkz_client::glam::Vec3::splat(1.0),
            });
        }
    }

    let mut king_animgraph_definition = AnimationGraphDefinition::new();
    let root_node_def = GenericNodeDefinition::Sampler(SamplerNodeDefinition {
        speed: 1.0,
        animation_name: "Walk".to_owned(),
        looping: true,
        name: "walk".to_owned(),
    });

    let root = king_animgraph_definition.graph.add_node(root_node_def);

    king_animgraph_definition.root = Some(root);

    u.load_character(
        &king_path,
        "King",
        current_scene,
        &king_instances,
        &king_animgraph_definition,
        &mut gfx_ctx.device,
        &mut gfx_ctx.queue,
        &gfx_ctx.debug_material,
        &gfx_ctx.texture_bind_group_layout,
        &gfx_ctx.bone_matrices_bind_group_layout,
    );

    let mut avocado_path = path.clone();
    avocado_path.push("Avocado.bin");

    let avocado = u.asset_mgr.load_3d_model_from_file(
        &avocado_path,
        "Avocado",
        &mut gfx_ctx.device,
        &mut gfx_ctx.queue,
        &gfx_ctx.debug_material,
        &gfx_ctx.texture_bind_group_layout,
    );

    match avocado {
        Ok(val) => {
            let mut instances = Instances::<Instance3D>::new();
            for x in 0..10 {
                for z in 0..10 {
                    instances.push(Instance3D {
                        position: noobwerkz_client::glam::Vec3::from_array([
                            SPACE_BETWEEN * x as f32,
                            0.0,
                            SPACE_BETWEEN * z as f32,
                        ]),
                        orientation: noobwerkz_client::glam::Quat::IDENTITY,
                        scale: noobwerkz_client::glam::Vec3::splat(10.0),
                    });
                }
            }
            u.scenes[current_scene]
                .model_nodes
                .push(Model3DNode::new(val, instances));
        }
        Err(err) => {
            println!("Could not load model from file. Error {}", err)
        }
    }

    lights.push(Light3DUniform::new(
        noobwerkz_client::glam::Vec3 {
            x: 3.0,
            y: 2.0,
            z: 3.0,
        },
        noobwerkz_client::glam::Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    ));

    u.scenes[current_scene].cameras.push(c);

    let mut particles_definition_path = path.clone();
    particles_definition_path.push("spark.particle");

    let mut particles_image_path = path.clone();
    particles_image_path.push("sparkling-fireball.png");

    let image = image::ImageReader::open(particles_image_path)
        .unwrap()
        .decode()
        .unwrap();

    let rgba = image.as_bytes();
    println!(
        "Particle image width: {}, height: {}",
        image.width(),
        image.height()
    );
    gfx_ctx.particles_ctx.borrow_mut().register_texture(
        &gfx_ctx.device,
        &gfx_ctx.queue,
        "spark",
        rgba,
        image.width(),
        image.height(),
        &gfx_ctx.texture_bind_group_layout,
    );

    u.scenes[current_scene]
        .particle_system
        .borrow_mut()
        .register(
            "yellow-spark",
            ParticleEffect::parse(
                std::fs::read_to_string(particles_definition_path)
                    .unwrap()
                    .as_str(),
            ),
        );

    for x in 0..10 {
        for z in 0..10 {
            u.scenes[current_scene].particle_system.borrow_mut().spawn(
                "yellow-spark",
                [SPACE_BETWEEN * x as f32, 1.0, SPACE_BETWEEN * z as f32],
            );
        }
    }
}
