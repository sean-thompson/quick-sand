mod graphics;
mod physics;

use graphics::Renderer;
use physics::{create_test_particles};
use std::sync::Arc;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    env_logger::init();
    log::info!("Starting Quick Sand particle simulation");

    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(WindowBuilder::new()
        .with_title("Quick Sand - Particle Simulation")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap());

    let mut renderer = pollster::block_on(Renderer::new(window.clone()));

    // Create static particles for foundation phase
    let particles = create_test_particles();
    renderer.update_particles(&particles);

    log::info!("Initialized with {} particles", particles.len());

    let window_for_event_loop = window.clone();
    let _ = event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window_for_event_loop.id() => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::Resized(physical_size) => {
                    renderer.resize(*physical_size);
                }
                WindowEvent::RedrawRequested => {
                    match renderer.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                _ => {}
            },
            Event::AboutToWait => {
                window_for_event_loop.request_redraw();
            }
            _ => {}
        }
    });
}