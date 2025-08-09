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

1. **Project Setup**
   - Initialize Rust project with proper dependency configuration
   - Set up basic logging and error handling
   - Configure release optimizations

2. **Window & GPU Context**
   - Create window with winit event loop
   - Initialize wgpu device, queue, and surface
   - Set up basic render pass structure

3. **Basic Particle Rendering**
   - Define particle data structure
   - Implement vertex/index buffers for instanced rendering
   - Create basic vertex and fragment shaders
   - Render static particles as colored points

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

### Setup
```bash
# Clone and build
cargo build --release

# Run with debug logging
RUST_LOG=debug cargo run

# Profile performance
cargo build --release --features=profiling
```

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