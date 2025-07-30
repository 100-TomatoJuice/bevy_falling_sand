# ⏳ Bevy Falling Sand
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/100-TomatoJuice/bevy_falling_sand#license)

![falling_sand](https://github.com/100-TomatoJuice/bevy_falling_sand/assets/67799071/b77d049c-5f99-407d-baab-67077cd72b7c) 

A velocity-based falling sand simulation built with Rust and Bevy!

This simulation uses [bevy_rapier](https://crates.io/crates/bevy_rapier2d) to generate colliders that regular rigidbodies can interact with. 
This allows for a player to affect the simulation and, since each particle type can have its separate collider, particle-based status effects!

Currently, the simulation is chunk-based, meaning that only chunks who have updating particles are simulated.

## ⚛ Particle Mappings

| Particle  | Mapping |
|-----------|---------|
| Sand      | 1       |
| Water     | 2       |
| Stone     | 3       |
| Acid      | 4       |
| Wood      | 5       |
| Spark     | 6       |
| Lava      | 7       |
| Oil       | 8       |
| Gunpowder | 9       |
| TNT       | 0       |
| Dirt      | -       |
| Grass     | =       |


## ⚙️ How to Test

- Download the `.zip`
- Extract the `.zip` and open the folder in preferred IDE
- In the terminal, type `cargo run --release` to run the project
