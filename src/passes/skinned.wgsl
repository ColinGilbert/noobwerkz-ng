// Vertex shader
struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
}
@group(1) @binding(0)
var<uniform> camera: Camera;

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
}
@group(2) @binding(0)
var<uniform> light: Light;

struct BoneMatrix {
    values: array<mat4x4<f32>>,
};

@group(3) @binding(0)
var<storage, read> bone_matrices: BoneMatrix;
@group(3) @binding(1)
var<uniform> num_bones: u32;

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) tangent: vec3<f32>,
    @location(4) bitangent: vec3<f32>,
    @location(5) bone_indices: vec4<u32>,
    @location(6) bone_weights: vec4<f32>,
}
struct InstanceInput {
    @location(7) model_matrix_0: vec4<f32>,
    @location(8) model_matrix_1: vec4<f32>,
    @location(9) model_matrix_2: vec4<f32>,
    @location(10) model_matrix_3: vec4<f32>,
    @location(11) normal_matrix_0: vec3<f32>,
    @location(12) normal_matrix_1: vec3<f32>,
    @location(13) normal_matrix_2: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) tangent_position: vec3<f32>,
    @location(2) tangent_light_position: vec3<f32>,
    @location(3) tangent_view_position: vec3<f32>,
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
    let bone_transform = mat4x4<f32>(
        (model.bone_weights.x * bone_matrices.values[num_bones * model.instance_index + model.bone_indices.x]) + (model.bone_weights.y * bone_matrices.values[num_bones * model.instance_index + model.bone_indices.y]) + (model.bone_weights.z * bone_matrices.values[num_bones * model.instance_index + model.bone_indices.z]) + (model.bone_weights.w * bone_matrices.values[num_bones * model.instance_index + model.bone_indices.w])
    );

    var skinned_position: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    for (var i: u32 = 0u; i < 4u; i = i + 1u) {
        let bone_index = model.bone_indices[i];
        let weight = model.bone_weights[i];
        let bone_matrix = bone_matrices.values[num_bones * model.instance_index + bone_index];
        
        // Accumulate the weighted bone transformation
        skinned_position += (bone_matrix * vec4<f32>(in.position, 1.0)) * weight;
    }

    let world = model_matrix * bone_transform;

    let skinned_normal = normalize(mat3x3<f32>(world[0].xyz, world[1].xyz, world[2].xyz) * model.normal);

    let transformed_tangent = bone_transform * vec4<f32>(model.tangent, 0.0);
    let skinned_tangent = normalize(transformed_tangent.xyz);

    // Calculate bitangent from the normal and tangent
    let skinned_bitangent = cross(skinned_normal, skinned_tangent);


    // Construct the tangent matrix
    //let world_normal = normalize(normal_matrix * model.normal);
    // let world_tangent = normalize(normal_matrix * model.tangent);
    // let world_bitangent = normalize(normal_matrix * model.bitangent);
    let tbn_matrix = transpose(mat3x3<f32>(
        skinned_tangent,
        skinned_bitangent,
        skinned_normal,
    ));

    let world_position = world * vec4<f32>(model.position, 1.0);

    var out: VertexOutput;
    out.clip_position = camera.view_proj * model_matrix * skinned_position;
    //out.world_normal = v_normal;
    out.tex_coords = model.tex_coords;
    out.tangent_position = tbn_matrix * skinned_position.xyz;
    out.tangent_view_position = tbn_matrix * camera.view_pos.xyz;
    out.tangent_light_position = tbn_matrix * light.position;
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

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let object_color: vec4<f32> = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    let object_normal: vec4<f32> = textureSample(t_normal, s_normal, in.tex_coords);
    
    // We don't need (or want) much ambient light, so 0.1 is fine
    let ambient_strength = 0.1;
    let ambient_color = light.color * ambient_strength;

    // Create the lighting vectors
    let tangent_normal = normalize(object_normal.xyz) * 2.0 - 1.0;
    let light_dir = normalize(in.tangent_light_position - in.tangent_position);
    let view_dir = normalize(in.tangent_view_position - in.tangent_position);
    let half_dir = normalize(view_dir + light_dir);

    let diffuse_strength = max(dot(tangent_normal, light_dir), 0.0);
    let diffuse_color = light.color * diffuse_strength;

    let specular_strength = pow(max(dot(tangent_normal, half_dir), 0.0), 32.0);
    let specular_color = specular_strength * light.color;

    let result = (ambient_color + diffuse_color + specular_color) * object_color.xyz;

    return vec4<f32>(result, object_color.a);
}