# Planetary Orbital Simulation (100% rust)

This is a simple 2D simulation that models the movement of planets around a central body using Newtonian physics. The simulation uses the `macroquad` crate for graphics and user input handling in the Rust programming language.

## Features:
- Central planet with gravitational force.
- Two orbiting planets (satellites) with trails to visualize their paths.
- Real-time simulation of planetary orbits using basic physics formulas.

## Changelog

#### [0.0.3v] - 2025-02-04

##### Added
- **`create_satellite` function**: Centralized satellite creation logic to avoid code duplication for initializing satellites with orbital parameters.
- **`update_trails` function**: Unified trail management logic for satellites, reducing repetitive code.
- **`draw_orbit_trail` function**: Simplified trail rendering by encapsulating repetitive drawing logic.

##### Changed
- Refactored the main loop to use helper functions, improving readability and maintainability.
- Replaced repetitive trail updates with an iteration-based approach using the `update_trails` function.
- Reduced redundancy in satellite and orbit initialization.

##### Improved
- The structure of the code, making it modular and easier to extend.
- Readability of the code by extracting repeated logic into dedicated functions.
- Performance and clarity in managing satellite trails by consolidating trail operations.

#### [v0.0.2] - Initial Release

- Orbital simulation with two satellites and a central planet.
- Trail rendering for satellite paths with a maximum trail size of 500 points.
- Real-time physics simulation with gravitational forces and velocity updates.
- Visualization of satellite orbits and trails, along with dynamic speed information display.

## Dependencies:
- [Rust](https://www.rust-lang.org/)
- [Macroquad](https://docs.rs/macroquad/)

## Installation and Setup:

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/planetary-orbital-simulation.git
   cargo run
