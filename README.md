# Planetary Orbital Simulation (100% rust)

This is a simple 2D simulation that models the movement of planets around a central body using Newtonian physics. The simulation uses the `macroquad` crate for graphics and user input handling in the Rust programming language.

## Features:
- Central planet with gravitational force.
- Two orbiting planets (satellites) with trails to visualize their paths.
- Real-time simulation of planetary orbits using basic physics formulas.

## Changelog

#### [0.0.4v] - 2025-02-04

##### Added
- **`draw_planet_with_shadow` function**: implemented shadows on planets based on the light source (star).

#### [v0.0.3] - Initial Release

- **`create_satellite` function**: Centralized satellite creation logic to avoid code duplication for initializing satellites with orbital parameters.
- **`update_trails` function**: Unified trail management logic for satellites, reducing repetitive code.
- **`draw_orbit_trail` function**: Simplified trail rendering by encapsulating repetitive drawing logic.

## Dependencies:
- [Rust](https://www.rust-lang.org/)
- [Macroquad](https://docs.rs/macroquad/)

## Installation and Setup:

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/planetary-orbital-simulation.git
   cargo run
