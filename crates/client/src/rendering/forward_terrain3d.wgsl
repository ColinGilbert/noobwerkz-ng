// Vertex shader
fn calculate_tangents_no_uv(position: vec3f, normal: vec3f) -> mat3x3f {
    // Get the screen-space change of the position
    let dpdx = dpdx(position);
    let dpdy = dpdy(position);

    // Compute cross products with the normal to find perpendicular vectors
    var tan = cross(dpdy, normal);
    var bitan = cross(normal, dpdx);

    // Guard against zero-length vectors (e.g., flat color or edge cases)
    let length_tan = length(tan);
    if (length_tan > 0.0) {
        tan = tan / length_tan;
    } else {
        // Fallback to an arbitrary orthogonal vector if derivatives fail
        tan = select(vec3f(1.0, 0.0, 0.0), vec3f(0.0, 0.0, 1.0), abs(normal.z) > 0.9);
        tan = normalize(cross(normal, tan));
    }

    // Recompute and normalize the bitangent to ensure strict orthogonality
    bitan = normalize(cross(normal, tan));

    // Return the complete TBN matrix
    return transpose(mat3x3f(tan, bitan, normal));
}

struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
}
@group(1) @binding(0)
var<uniform> camera: Camera;

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
    direction: vec3<f32>,
}
@group(2) @binding(0)
var<uniform> light: Light;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct InstanceInput {
    @location(2) model_matrix_0: vec4<f32>,
    @location(3) model_matrix_1: vec4<f32>,
    @location(4) model_matrix_2: vec4<f32>,
    @location(5) model_matrix_3: vec4<f32>,
    @location(6) normal_matrix_0: vec3<f32>,
    @location(7) normal_matrix_1: vec3<f32>,
    @location(8) normal_matrix_2: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) obj_position: vec3<f32>,
    @location(2) surface_to_light: vec3<f32>,
    @location(3) surface_to_view:  vec3<f32>,
    // @location(1) tangent_position: vec3<f32>,
    // @location(2) tangent_view_position: vec3<f32>,
    // @location(3) tangent_light_position: vec3<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    let normal_matrix = mat3x3<f32>(
        instance.normal_matrix_0,
        instance.normal_matrix_1,
        instance.normal_matrix_2,
    );

   
    let world_position = model_matrix * vec4<f32>(model.position, 1.0);
    // Construct the tangent matrix
    let world_normal = normalize(normal_matrix * model.normal);
    // let world_tangent = normalize(normal_matrix * model.tangent);
    // let world_bitangent = normalize(normal_matrix * model.bitangent);
    // let tangent_matrix = transpose(mat3x3<f32>(
    //     world_tangent,
    //     world_bitangent,
    //     world_normal,
    // ));


    
    var out: VertexOutput;
    out.clip_position = camera.view_proj * world_position;
    out.normal = world_normal;
    out.obj_position = model.position;
    out.surface_to_light = light.position - world_position.xyz;
    out.surface_to_view = camera.view_pos.xyz - world_position.xyz;
    // out.tangent_position = tangent_matrix * world_position.xyz;
    // out.tangent_view_position = tangent_matrix * camera.view_pos.xyz;
    // out.tangent_light_position = tangent_matrix * light.position;
    
    return out;
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;
@group(0)@binding(2)
var t_normal: texture_2d<f32>;
@group(0) @binding(3)
var s_normal: sampler;

struct PbrMaterial {
    color: vec4<f32>,
    metallic: f32,
    roughness: f32,
}

@group(0) @binding(4)
var<uniform> pbr: PbrMaterial;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    let position = normalize(in.obj_position);
    let object_normal = normalize(in.normal);
    let blending = normalize(abs(in.normal));
    let b = blending.x + blending.y + blending.z;    
    let final_blend = vec3(blending.x / b, blending.y/b, blending.z/b);

    // We don't need (or want) much ambient light, so 0.1 is fine
    let x_axis = textureSample(t_diffuse, s_diffuse, position.yz);
    let y_axis = textureSample(t_diffuse, s_diffuse, position.xz);
    let z_axis = textureSample(t_diffuse, s_diffuse, position.xy);
    let ambient_strength = 0.1;
    let ambient_light = light.color * ambient_strength;

   let directional_light = dot(object_normal, -light.direction);
    
    let surface_to_light_direction = normalize(in.surface_to_light);
    let surface_to_view_direction = normalize(in.surface_to_view);
    let half_vector = normalize(surface_to_light_direction + surface_to_view_direction);
    let specular_light = dot(object_normal, half_vector);

    // let tbn_matrix = calculate_tangents_no_uv(in.clip_position.xyz, in.normal);
    // let tangent = tbn_matrix[0];
    // let bitangent = tbn_matrix[1];
    // Create the lighting vectors
   
    // let light_dir = normalize(light.position - in.obj_position.xyz);
    // let view_dir = normalize(camera.view_pos.xyz - in.obj_position.xyz);
    // let half_dir = normalize(view_dir + light_dir);

    // let diffuse_strength = max(dot(in.normal, light_dir), 0.0);
    // let diffuse_color = light.color * diffuse_strength;

    // let specular_strength = pow(max(dot(in.normal, half_dir), 0.0), 32.0);
    // let specular_color = specular_strength * light.color;

    // let diffuse_strength = max(dot(tangent_normal, light_dir), 0.0);
    // let diffuse_color = light.color * diffuse_strength;

    // let specular_strength = pow(max(dot(tangent_normal, half_dir), 0.0), 32.0);
    // let specular_color = specular_strength * light.color;
    let object_color = x_axis * blending.x + y_axis * blending.y + z_axis * blending.z;
    let result = (ambient_light + directional_light + specular_light) * object_color.xyz;

    return vec4<f32>(result, 1.0);
}