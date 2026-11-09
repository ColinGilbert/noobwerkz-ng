// Vertex shader

struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
}
@group(1) @binding(0)
var<uniform> camera: Camera;

struct Light {
    position: vec3<f32>,
    point_light_intensity: f32,
    color: vec3<f32>,
    ambient_light_intensity: f32,
    direction: vec3<f32>,
    directional_light_intensity: f32,
}
@group(2) @binding(0)
var<uniform> light: Light;

struct BoneMatrix {
    data: array<mat4x4<f32>>,
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
    @location(1) normal: vec3<f32>,
    @location(2) surface_to_light: vec3<f32>,
    @location(3) surface_to_view: vec3<f32>,
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

    let offset = num_bones * model.instance_index;
    let bone_transform = mat4x4<f32>(
        (bone_matrices.data[offset + model.bone_indices.x] * model.bone_weights.x) + (bone_matrices.data[offset + model.bone_indices.y] * model.bone_weights.y) + (bone_matrices.data[offset + model.bone_indices.z] * model.bone_weights.z) + (bone_matrices.data[offset + model.bone_indices.w] * model.bone_weights.w )
    );
    // let bone_normal_mat = mat3x3<f32>(bone_transform[0].xyz, bone_transform[1].xyz, bone_transform[2].xyz);
    let world_matrix = model_matrix * bone_transform;
    let world_position = world_matrix * vec4<f32>(model.position, 1.0);
    let skinned_normal = (bone_transform * vec4<f32>(model.normal, 0.0)).xyz;
    // let transformed_tangent = bone_transform * vec4<f32>(model.tangent, 0.0);
    // let skinned_tangent = normalize(transformed_tangent.xyz);
    // let skinned_bitangent = cross(skinned_normal, skinned_tangent);
    // let tbn_matrix = transpose(mat3x3<f32>(
    //     skinned_tangent,
    //     skinned_bitangent,
    //     skinned_normal,
    // ));
    var out: VertexOutput;
    out.clip_position = camera.view_proj * world_position;
    out.tex_coords = model.tex_coords;
        out.surface_to_light = light.position.xyz - world_position.xyz;
    out.surface_to_view = camera.view_pos.xyz - world_position.xyz;
    // out.tangent_position = tbn_matrix * world_position.xyz;
    // out.tangent_view_position = tbn_matrix * camera.view_pos.xyz;
    // out.tangent_light_position = tbn_matrix * light.position;
    out.normal = skinned_normal;
    return out;
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;
@group(0) @binding(2)
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
    let tex_color: vec4<f32> = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    let object_color:vec4<f32> = pbr.color * tex_color;
    let object_normal: vec3<f32> = normalize(in.normal);//textureSample(t_normal, s_normal, in.tex_coords);

    // let object_normal: vec4<f32> = textureSample(t_normal, s_normal, in.tex_coords);
    
    // We don't need (or want) much ambient light, so 0.1 is fine
    let ambient_light = light.color.xyz * light.ambient_light_intensity;

    let directional_light = light.color.xyz * dot(object_normal, -light.direction.xyz) * light.directional_light_intensity;
    
    let surface_to_light_direction = normalize(in.surface_to_light);
    let surface_to_view_direction = normalize(in.surface_to_view);
    let half_vector = normalize(surface_to_light_direction + surface_to_view_direction);
    let specular_light = light.color.xyz * dot(object_normal, half_vector) * light.point_light_intensity;


    // // Create the lighting vectors
    // let tangent_normal = normalize(object_normal * 2.0 - 1.0);
    // // let tangent_normal = normalize(object_normal.xyz) * 2.0 - 1.0;
    // // let light_dir = normalize(in.tangent_light_position - in.tangent_position);
    // // let view_dir = normalize(in.tangent_view_position - in.tangent_position);
    // let light_dir = normalize(in.tangent_position - in.tangent_light_position);
    // let view_dir = normalize(in.tangent_position - in.tangent_view_position);
    // let half_dir = normalize(view_dir + light_dir);

    // let diffuse_strength = max(dot(tangent_normal, light_dir), 0.0);
    // let diffuse_color = light.color * diffuse_strength;

    // let specular_strength = pow(max(dot(tangent_normal, half_dir), 0.0), 32.0);
    // let specular_color = specular_strength * light.color;

    let result = (ambient_light + specular_light) * object_color.xyz;

    return vec4<f32>(result, object_color.a);
}