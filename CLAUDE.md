# Claude Context for Quick Sand

## Project Overview
High-performance Rust particle simulation using wgpu for GPU acceleration. Simulates falling sand/particles with real-time physics and visual effects.

## Build & Run Commands
```bash
# Development build and run
cargo run

# Release build (optimized)
cargo build --release && target/release/quick-sand

# Run with debug logging
RUST_LOG=debug cargo run

# Format code
cargo fmt

# Lint check
cargo clippy

# Run tests
cargo test
```

## Key Architecture
- **Graphics**: wgpu for cross-platform GPU abstraction
- **Physics**: Custom particle system with spatial partitioning
- **Shaders**: WGSL compute shaders for physics, fragment shaders for effects
- **Performance Target**: 60 FPS with 10,000+ particles

## Project Structure
```
src/
├── main.rs           # Entry point and event loop
├── graphics/         # Rendering pipeline and shaders
│   ├── renderer.rs   # Main render system
│   ├── shaders/      # WGSL shader files
│   └── buffers.rs    # GPU buffer management
├── physics/          # Particle simulation
│   ├── particle.rs   # Particle data structures
│   ├── simulation.rs # Physics calculations
│   └── spatial.rs    # Spatial partitioning
└── input/            # User interaction handling
    └── controls.rs   # Mouse/keyboard input
```

## Implementation Phases
1. **Foundation**: Basic window + GPU context + static particle rendering
2. **Core Systems**: Physics simulation + spatial optimization + compute shaders
3. **Polish**: Visual effects + user interaction + performance tuning

## Dependencies Rationale
- `wgpu 0.18`: Modern GPU abstraction with compute shader support
- `winit 0.29`: Cross-platform windowing and input
- `nalgebra 0.32`: Linear algebra for physics calculations
- `bytemuck 1.14`: Safe casting for GPU buffer data

## Performance Considerations
- Use compute shaders for physics to leverage GPU parallelism
- Implement spatial hashing for O(1) neighbor queries
- Double-buffer particle data for GPU synchronization
- Profile with Tracy or similar tools

## Common Issues
- **Shader compilation**: Ensure WGSL syntax is correct, use wgpu validation layers
- **GPU buffer overflow**: Monitor particle count vs buffer capacity
- **Frame timing**: Use VSync and proper frame pacing
- **Windows line endings**: Git will handle CRLF conversion automatically

## Testing Strategy
- Unit tests for physics calculations and spatial queries
- Integration tests for particle system behavior  
- Visual tests for rendering correctness
- Performance benchmarks for optimization validation