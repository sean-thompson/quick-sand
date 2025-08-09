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
    
    // Create a quad for each particle (2 triangles = 6 vertices)
    let vertices = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>(-1.0,  1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>( 1.0,  1.0),
        vec2<f32>(-1.0,  1.0),
    );
    
    let vertex_pos = vertices[input.vertex_index];
    
    // Scale by particle size
    let scaled_pos = vertex_pos * particle.size;
    
    // Translate to particle position
    let world_pos = scaled_pos + particle.position;
    
    // Convert to normalized device coordinates
    let ndc_pos = (world_pos / uniforms.screen_size) * 2.0;
    
    var output: VertexOutput;
    output.clip_position = vec4<f32>(ndc_pos, 0.0, 1.0);
    output.color = particle.color;
    
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color, 1.0);
}