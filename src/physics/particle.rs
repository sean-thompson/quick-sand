use bytemuck::{Pod, Zeroable};
use nalgebra::Vector2;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Particle {
    pub position: [f32; 2],
    pub color: [f32; 3],
    pub size: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32, color: [f32; 3]) -> Self {
        Self {
            position: [x, y],
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
                (i as f32 / 50.0),
                (j as f32 / 30.0),
                0.8,
            ];
            
            particles.push(Particle::new(x, y, color));
        }
    }
    
    particles
}