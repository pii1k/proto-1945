# proto-1945

A small Rust-based 2D shooting game prototype. It is intended for learning and includes basic gameplay elements such as the player, enemies, bullets, and collision handling.

**Key Features**
- Player movement and shooting
- Enemy spawning and collision handling
- Simple game state management

**Requirements**
- Rust and Cargo (stable)
- A GPU-capable environment for the Bevy engine

**Build**
```bash
cargo build --release
```

**Run**
```bash
cargo run --release
```

**Controls**
- Move: Arrow keys or `W`/`A`/`S`/`D`
- Fire: `J`
- Exit: Close the window

**Project Structure (important files)**
- `src/main.rs` — entry point
- `src/player.rs` — player spawn and movement logic
- `src/bullet.rs` — player bullet spawning and movement
- `src/enemy.rs` — enemy logic
- `src/collision.rs` — collision handling
- `src/game_state.rs` — game state management
- `src/consts.rs` — game constants