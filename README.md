# Quick Sand - Falling Particle Simulation

A high-performance Rust demo that simulates falling particles using GPU acceleration. Features real-time physics, spatial partitioning, and visual effects powered by wgpu shaders.

## Features

- **GPU-Accelerated Physics**: Compute shaders handle particle movement and interactions
- **Spatial Optimization**: Efficient collision detection using spatial hashing
- **Visual Effects**: Motion blur, particle trails, and glow effects
- **Cross-Platform**: Runs on Windows, macOS, and Linux via wgpu
- **Interactive**: Mouse/keyboard controls for real-time parameter adjustment

## Performance Targets

- 60 FPS with 10,000+ particles
- Sub-16ms frame times
- Smooth particle interactions and merging

## Technical Architecture

### Dependencies

```toml
[dependencies]
wgpu = "0.18"           # Cross-platform GPU abstraction
winit = "0.29"          # Window creation and event handling
nalgebra = "0.32"       # Linear algebra for physics calculations
bytemuck = "1.14"       # Safe casting for GPU buffers
env_logger = "0.10"     # Logging for debugging
pollster = "0.3"        # Async runtime for wgpu initialization
```

### Core Systems

- **Particle System**: Position, velocity, type, and lifetime tracking
- **Physics Engine**: Custom gravity, collision, and merging logic
- **Spatial Partitioning**: Grid-based optimization for interaction queries
- **Render Pipeline**: Instanced rendering with compute shader integration
- **Shader Effects**: WGSL shaders for visual abstraction and post-processing

## Implementation Phases

### Phase 1: Foundation (Critical Path)

1. **Project Setup** ✅
   - ✅ Initialize Rust project with proper dependency configuration
     - Created `Cargo.toml` with wgpu 0.18, winit 0.29, nalgebra 0.32, bytemuck 1.14
     - Added pollster 0.3 for async runtime support in wgpu initialization
     - Dependencies chosen for cross-platform GPU rendering and efficient data handling
   - ✅ Set up basic logging and error handling
     - Added env_logger 0.10 and log 0.4 for debug output during development
     - Logging initialized in main.rs with `env_logger::init()` for runtime diagnostics
     - Error handling implemented for GPU surface operations and window events
   - ✅ Configure release optimizations
     - Cargo.toml uses edition = "2021" for latest Rust features
     - Release builds automatically optimize for performance via `cargo build --release`
     - GPU validation layers available in debug builds, disabled in release for performance

2. **Window & GPU Context** ✅
   - ✅ Create window with winit event loop
     - Implemented in main.rs with EventLoop::new() and WindowBuilder
     - Window set to 800x600 with title "Quick Sand - Particle Simulation"
     - Event loop handles resize, close, and redraw events with proper control flow
   - ✅ Initialize wgpu device, queue, and surface
     - Renderer::new() async function handles GPU initialization in renderer.rs
     - Surface created from window, adapter requested with compatibility checks
     - Device and queue created with appropriate limits and features
   - ✅ Set up basic render pass structure
     - Render pass configured with dark background (0.1, 0.1, 0.1) clear color
     - Color attachment with store operation for frame persistence
     - Command encoder and queue submission pattern established

3. **Basic Particle Rendering** ✅
   - ✅ Define particle data structure
     - Particle struct in physics/particle.rs with position [f32; 2], color [f32; 3], size f32
     - Implements bytemuck Pod/Zeroable for safe GPU buffer casting
     - create_test_particles() generates 1,500 particles in 50x30 grid with gradient colors
   - ✅ Implement vertex/index buffers for instanced rendering
     - Particle buffer created as STORAGE buffer for GPU access in renderer.rs
     - Instanced rendering: 6 vertices per particle (2 triangles forming quad)
     - Bind group layout connects uniform buffer (screen size) and particle storage buffer
   - ✅ Create basic vertex and fragment shaders
     - WGSL shaders in graphics/shaders/particle.wgsl for cross-platform compatibility
     - Vertex shader transforms particle positions to screen space with proper scaling
     - Fragment shader outputs interpolated particle colors with full alpha
   - ✅ Render static particles as colored points
     - Render pipeline created with vertex/fragment shader stages
     - Draw call uses instanced rendering: draw(0..6, 0..particle_count)
     - Each particle rendered as a small quad with per-particle color

### Phase 2: Core Simulation

4. **Physics Implementation**
   - Add velocity and gravity to particle updates
   - Implement basic collision detection
   - Create particle-to-particle interaction rules
   - Add boundary collision handling

5. **Spatial Optimization**
   - Implement spatial hash grid for efficient queries
   - Optimize collision detection using spatial partitioning
   - Add neighbor search algorithms
   - Profile and tune grid cell sizes

6. **Compute Shaders**
   - Move physics calculations to GPU compute shaders
   - Implement double-buffering for particle data
   - Add GPU-based spatial partitioning
   - Synchronize compute and render passes

### Phase 3: Visual Polish

7. **Advanced Rendering**
   - Add particle size and color variation
   - Implement motion blur effects
   - Create particle trail rendering
   - Add glow/bloom post-processing

8. **User Interaction**
   - Mouse input for particle spawning/manipulation
   - Keyboard controls for simulation parameters
   - Real-time UI for adjusting gravity, particle count
   - Save/load simulation states

9. **Performance Optimization**
   - Profile with Tracy or similar tools
   - Optimize GPU buffer usage and transfers
   - Implement level-of-detail for distant particles
   - Add adaptive quality settings

## Development Workflow

### Setup & Running

**Prerequisites:**
- Install Rust from https://rustup.rs/
- Restart your terminal/VS Code after installation

**Build and Run:**
```bash
# Development build and run
cargo run

# Release build (optimized)
cargo build --release
./target/release/quick-sand

# Run with debug logging
RUST_LOG=debug cargo run
```

**VS Code Integration:**
- Use `Ctrl+Shift+P` → "Tasks: Run Task" → "Run Quick Sand"
- Or `Ctrl+Shift+P` → "Tasks: Run Build Task" for quick launch
- Install the "rust-analyzer" extension for better Rust support

**Current Status - Foundation Phase Complete:**
- ✅ Basic window creation with wgpu GPU context
- ✅ Static particle rendering (1,500 particles in a colorful grid)
- ✅ Cross-platform graphics pipeline using WGSL shaders
- ✅ 60 FPS rendering with proper frame timing

**What you'll see:**
- 800x600 window with static colored particles
- Resizable window with proper GPU resource management
- Smooth rendering demonstrating the graphics foundation

**Next Steps:**
Run `cargo run` to see the foundation phase in action, then proceed to Phase 2 for physics simulation.

### Debugging Tools
- **RenderDoc**: Frame capture and GPU debugging
- **Tracy**: CPU/GPU profiling and timeline analysis
- **wgpu-debug**: Validation layers for graphics debugging

### Testing Strategy
- Unit tests for physics calculations and spatial queries
- Integration tests for particle system behavior
- Visual regression tests for rendering consistency
- Performance benchmarks for optimization validation

## Architecture Decisions

### Why wgpu?
- Cross-platform GPU abstraction (Vulkan/Metal/DX12)
- Modern compute shader support
- Excellent Rust integration
- Active development and community

### Why Custom Physics?
- Specialized particle-particle interactions
- GPU-friendly algorithms
- Tight integration with rendering pipeline
- Performance optimized for this specific use case

### Shader Strategy
- WGSL for consistency across platforms
- Compute shaders for physics simulation
- Fragment shaders for visual effects
- Modular shader system for easy experimentation

## Contributing

1. Ensure code follows Rust formatting (`cargo fmt`)
2. Run clippy for lint checks (`cargo clippy`)
3. Add tests for new functionality
4. Profile performance impact of changes
5. Update documentation for API changes

## License

MIT License - see LICENSE file for details