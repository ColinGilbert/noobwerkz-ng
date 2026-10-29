use noobwerkz_client::camera3d::*;
use noobwerkz_client::client_user_context::ClientUserContext;
use noobwerkz_client::glam::Vec3;
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

use image::*;

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
        0.0001,
        1000.0,
    );

    let mut s = Scene3D::new(&noobwerkz_client::glam::Vec3 {
        x: 0.0,
        y: -9.81,
        z: 0.0,
    });

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

    let mut path = std::path::PathBuf::new();
    path.push("res");
    let mut cesium_man_path = path.clone();
    cesium_man_path.push("cesium-man-model.bin");

    let cesium_man = u.asset_mgr.load_skinned_3d_model_from_file(
        &cesium_man_path,
        "Cesium Man",
        &mut gfx_ctx.device,
        &mut gfx_ctx.queue,
        &gfx_ctx.debug_material,
        &gfx_ctx.texture_bind_group_layout,
    );

    const SPACE_BETWEEN: f32 = 1.6;
    let mut cesium_man_skeletal_anim_instances = Vec::<Instance3D>::new();
    let mut outer = 0;
    while outer < 10 {
        let mut inner = 0;
        while inner < 10 {
            cesium_man_skeletal_anim_instances.push(Instance3D {
                position: noobwerkz_client::glam::Vec3A::from_array([
                    SPACE_BETWEEN * outer as f32,
                    3.0,
                    SPACE_BETWEEN * inner as f32,
                ]),
                orientation: noobwerkz_client::glam::Quat::IDENTITY,
                scale: noobwerkz_client::glam::Vec3A::splat(1.0),
            });
            inner += 1;
        }
        outer += 1;
    }

    // let mut skeletal_anim_instances_two = Vec::<Instance3D>::new();
    // let mut outer = 0;
    // while outer < 2 {
    //     let mut inner = 0;
    //     while inner < 2 {
    //         skeletal_anim_instances_two.push(Instance3D {
    //             position: noobwerkz_client::glam::Vec3A::from_array([
    //                 SPACE_BETWEEN * outer as f32,
    //                 0.0,
    //                 SPACE_BETWEEN * inner as f32,
    //             ]),
    //             orientation: noobwerkz_client::glam::Quat::IDENTITY,
    //             scale: noobwerkz_client::glam::Vec3A::splat(1.0),
    //         });
    //         inner += 1;
    //     }
    //     outer += 1;
    // }

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

    match cesium_man {
        Ok(val) => {
            s.add_characters(
                &mut gfx_ctx.device,
                &gfx_ctx.bone_matrices_bind_group_layout,
                val,
                &cesium_man_skeletal_anim_instances,
                &cesium_man_animgraph_definition,
                u.asset_mgr.skinned_models[val].animations.clone(),
                u.asset_mgr.skinned_models[val].skeleton.clone(),
                &u.asset_mgr.skinned_models[val].animations_idx_by_name,
                "cesium-man".to_owned(),
            );
            // s.add_characters(
            //     &mut gfx_ctx.device,
            //     &gfx_ctx.bone_matrices_bind_group_layout,
            //     val,
            //     &skeletal_anim_instances_two,
            //     &animgraph_definition,
            //     u.asset_mgr.skinned_models[val].animations.clone(),
            //     u.asset_mgr.skinned_models[val].skeleton.clone(),
            //     &u.asset_mgr.skinned_models[val].animations_idx_by_name,
            //     "cesium-man-two".to_owned(),
            // );
        }
        Err(err) => {
            println!(
                "Could not load skinned model for Cesium Man. Error: {}",
                err
            );
        }
    }

    let mut king_path = path.clone();
    king_path.push("King.bin");

    let king = u.asset_mgr.load_skinned_3d_model_from_file(
        &king_path,
        "King",
        &mut gfx_ctx.device,
        &mut gfx_ctx.queue,
        &gfx_ctx.debug_material,
        &gfx_ctx.texture_bind_group_layout,
    );

    let mut king_skeletal_anim_instances = Vec::<Instance3D>::new();
    let mut outer = 0;
    while outer < 1 {
        let mut inner = 0;
        while inner < 1 {
            king_skeletal_anim_instances.push(Instance3D {
                position: noobwerkz_client::glam::Vec3A::from_array([
                    SPACE_BETWEEN * outer as f32,
                    -2.0,
                    SPACE_BETWEEN * inner as f32,
                ]),
                orientation: noobwerkz_client::glam::Quat::from_axis_angle(noobwerkz_client::glam::Vec3::Y, 90.0),
                scale: noobwerkz_client::glam::Vec3A::splat(1.0),
            });
            inner += 1;
        }
        outer += 1;
    }


    let mut king_animgraph_definition = AnimationGraphDefinition::new();
    let root_node_def = GenericNodeDefinition::Sampler(SamplerNodeDefinition {
        speed: 1.0,
        animation_name: "CharacterArmature|Walk".to_owned(),
        looping: true,
        name: "Walk".to_owned(),
    });

    let root = king_animgraph_definition
        .graph
        .add_node(root_node_def);
    king_animgraph_definition.root = Some(root);

    match king {
        Ok(val) => {
            s.add_characters(
                &mut gfx_ctx.device,
                &gfx_ctx.bone_matrices_bind_group_layout,
                val,
                &king_skeletal_anim_instances,
                &king_animgraph_definition,
                u.asset_mgr.skinned_models[val].animations.clone(),
                u.asset_mgr.skinned_models[val].skeleton.clone(),
                &u.asset_mgr.skinned_models[val].animations_idx_by_name,
                "king".to_owned(),
            );
        }
        Err(err) => {
            println!("Could not load skinned mode for King. Error: {}", err);
        }
    }

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
            instances.push(Instance3D {
                position: noobwerkz_client::glam::Vec3A::from_array([0.0, 0.0, 0.0]),
                orientation: noobwerkz_client::glam::Quat::IDENTITY,
                scale: noobwerkz_client::glam::Vec3A::splat(10.0),
            });
            s.model_nodes.push(Model3DNode::new(val, instances));
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

    s.cameras.push(c);

    let mut particles_definition_path = path.clone();
    particles_definition_path.push("spark.particle");

    let mut particles_image_path = path.clone();
    particles_image_path.push("sparkling-fireball.png");

    let image = image::ImageReader::open(particles_image_path)
        .unwrap()
        .decode()
        .unwrap();

    let rgba = image.as_bytes();
    println!("Image width: {}, height: {}", image.width(), image.height());
    gfx_ctx.particles_ctx.borrow_mut().register_texture(
        &gfx_ctx.device,
        &gfx_ctx.queue,
        "yellow-spark",
        rgba,
        image.width(),
        image.height(),
        &gfx_ctx.texture_bind_group_layout,
    );

    s.particle_system.borrow_mut().register(
        "yellow-spark",
        ParticleEffect::parse(
            std::fs::read_to_string(particles_definition_path)
                .unwrap()
                .as_str(),
        ),
    );

    let emitter = s
        .particle_system
        .borrow_mut()
        .spawn("yellow-spark", [0.0, 1.0, 0.0]);

    println!(
        "(Scene setup): Particle system empty? {}",
        s.particle_system.borrow().is_empty()
    );
    println!("Emitter ID: {:?}", emitter);
    u.scenes.push(s);
}
