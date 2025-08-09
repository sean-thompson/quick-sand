struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @builtin(instance_index) instance_index: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

struct Particle {
    position: vec2<f32>,
    _padding1: vec2<f32>,
    color: vec3<f32>,
    size: f32,
}

struct Uniforms {
    screen_size: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(0) @binding(1)
var<storage, read> particles: array<Particle>;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    let particle = particles[input.instance_index];
    
    // Generate vertex position based on vertex index using conditionals
    var vertex_pos: vec2<f32>;
    if (input.vertex_index == 0u) {
        vertex_pos = vec2<f32>(-1.0, -1.0);
    } else if (input.vertex_index == 1u) {
        vertex_pos = vec2<f32>( 1.0, -1.0);
    } else if (input.vertex_index == 2u) {
        vertex_pos = vec2<f32>(-1.0,  1.0);
    } else if (input.vertex_index == 3u) {
        vertex_pos = vec2<f32>( 1.0, -1.0);
    } else if (input.vertex_index == 4u) {
        vertex_pos = vec2<f32>( 1.0,  1.0);
    } else {
        vertex_pos = vec2<f32>(-1.0,  1.0);
    }
    
    // Scale by particle size
    let scaled_pos = vertex_pos * particle.size;
    
    // Translate to particle position
    let world_pos = scaled_pos + particle.position;
    
    // Convert to normalized device coordinates
    // Screen space: (0,0) top-left, (width,height) bottom-right
    // NDC space: (-1,-1) bottom-left, (1,1) top-right
    let ndc_x = (world_pos.x / (uniforms.screen_size.x * 0.5));
    let ndc_y = -(world_pos.y / (uniforms.screen_size.y * 0.5)); // Flip Y axis
    let ndc_pos = vec2<f32>(ndc_x, ndc_y);
    
    var output: VertexOutput;
    output.clip_position = vec4<f32>(ndc_pos, 0.0, 1.0);
    output.color = particle.color;
    
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color, 1.0);
}