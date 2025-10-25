# Rust-Pong

Welcome to the **Rust-Pong** repository! This repository contains a small Pong clone implemented in Rust to demonstrate modular game structure, audio, and simple UI.

## 🚀 Getting Started

To get started with this project:

1. Clone the repository:
    ```bash
    git clone https://github.com/Vianpyro/Rust-Pong.git
    cd Rust-Pong
    ```
2. Install the Rust toolchain if you don't already have it: https://rustup.rs/
3. Build the project:
    ```bash
    cargo build
    ```
4. Run the game:
    ```bash
    cargo run --release
    ```

Notes:
- Assets (sound effects, etc.) are located in the `assets/` directory. Make sure they remain next to the executable when distributing or running the game.
- If you encounter platform-specific audio or windowing issues, ensure the required system libraries (for audio/display) are available for your OS.

## 📁 Project Structure

The repository contains the following directories and files (high level):

- `assets/` - Game assets (sounds, images, etc.)
- `src/` - Application source code
    - `audio/` - Audio handling
    - `game/` - Game objects and physics (ball, racket, score)
    - `player/` - Player and controller code
    - `ui/` - Menus, HUD, and screens
    - `main.rs` - Application entry point
    - `main_state.rs`, `debug.rs` - Game state and debugging helpers
- `Cargo.toml` - Rust package manifest
- `LICENSE` - Project license (see file for terms)

## 🛠 Features

- Rust-based Pong clone demonstrating basic game loop, physics, and UI.
- Modular code organization (audio, game logic, players, UI).
- Lightweight and easy to extend for experimentation or learning.

## 📖 Documentation

The code is organized into clear modules under `src/`. For details, explore the following files and folders:
- `src/game/` — core game logic and physics
- `src/audio/` — audio playback and resource handling
- `src/ui/` — UI screens (menu, HUD, pause, game over)

Expand this README as the project grows to include contribution guidelines, a development roadmap, and detailed architecture notes.

## 🤝 Contributing

Contributions are welcome. Suggested workflow:
1. Fork the repository.
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes and add tests where applicable.
4. Open a pull request to the main repository.

When opening issues or PRs, provide reproduction steps and any relevant logs or OS details.

## 📝 License

See the [`LICENSE`](/LICENSE) file in this repository for license terms.

Happy coding! 🎉
