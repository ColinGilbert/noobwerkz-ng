use noobwerkz_client::camera3d::CameraContext;
use noobwerkz_client::client_user_context::ClientUserContext;
use noobwerkz_client::graphics::*;
use noobwerkz_client::light3d::*;
use noobwerkz_client::web_time::Duration;
use std::f32::consts::PI;

use crate::user_data::*;

// MODIFY ME!
pub fn user_update(
    gfx_ctx: &mut GraphicsContext,
    cam_ctx: &mut CameraContext,
    light_ctx: &mut Light3DContext,
    user_ctx: &mut ClientUserContext,
    dt: Duration,
) {
    let u = user_ctx;
    let scene_idx = u.active_scene;
    let s = &mut u.scenes[scene_idx];
    let cam_idx = s.active_camera;
    s.cameras[cam_idx].update();
    s.update_particles(dt);
    cam_ctx
        .uniform
        .update_view_proj(&s.cameras[cam_idx], &s.cameras[cam_idx].projection);
    gfx_ctx.queue.write_buffer(
        &cam_ctx.buffer,
        0,
        noobwerkz_client::bytemuck::cast_slice(&[cam_ctx.uniform]),
    );

    // Update the light
    let old_position: noobwerkz_client::glam::Vec3 = light_ctx.light_uniforms[0].position.into();
    light_ctx.light_uniforms[0].position = (noobwerkz_client::glam::Quat::from_axis_angle(
        noobwerkz_client::glam::Vec3::Y,
        PI * dt.as_secs_f32(),
    ) * old_position)
        .into();
    gfx_ctx.queue.write_buffer(
        &light_ctx.light_buffer,
        0,
        noobwerkz_client::bytemuck::cast_slice(&[light_ctx.light_uniforms[0]]),
    );

    let fps = Duration::from_micros((1_000_000.0 / 60.0) as u64);
    u.time_elapsed += dt.as_micros();
    if u.time_elapsed > fps.as_micros() {
        s.update_characters(dt, &u.asset_mgr.skinned_models, &gfx_ctx.queue);
        u.time_elapsed -= fps.as_micros();
    }

    // println!("Particle system empty? {}", s.particle_system.borrow().is_empty());

    let _user_data = USER_DATA
        .lock()
        .expect("UserData poisoned during user_update()");
}
