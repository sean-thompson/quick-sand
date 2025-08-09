use bytemuck::{Pod, Zeroable};

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Particle {
    pub position: [f32; 2],
    pub _padding1: [f32; 2],   // Pad to 16 bytes for WGSL alignment
    pub color: [f32; 3], 
    pub size: f32,             // Size goes in the 4th component of color vec4
}

impl Particle {
    pub fn new(x: f32, y: f32, color: [f32; 3]) -> Self {
        Self {
            position: [x, y],
            _padding1: [0.0, 0.0],
            color,
            size: 2.0,
        }
    }
}

pub fn create_test_particles() -> Vec<Particle> {
    let mut particles = Vec::new();
    
    // Create a grid of colorful particles
    for i in 0..50 {
        for j in 0..30 {
            let x = (i as f32 - 25.0) * 8.0;
            let y = (j as f32 - 15.0) * 8.0;
            
            let color = [
                (i as f32 / 49.0),        // Red: 0.0 to 1.0 (left to right)
                (j as f32 / 29.0),        // Green: 0.0 to 1.0 (top to bottom)  
                0.3,                      // Blue: slight tint for visibility
            ];
            
            particles.push(Particle::new(x, y, color));
        }
    }
    
    log::info!("Generated {} particles with position range: x=({:.1} to {:.1}), y=({:.1} to {:.1})", 
        particles.len(), -25.0 * 8.0, 24.0 * 8.0, -15.0 * 8.0, 14.0 * 8.0);
    
    particles
}