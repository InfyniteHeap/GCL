# Contributing to Grid Craft Launcher

Thank you for your interest in contributing to Grid Craft Launcher!

## Build This Project

Before getting start, you **MUST** install these components:

- [Rust](https://www.rust-lang.org/)
- [NodeJS](https://nodejs.org/)
- [pnpm](https://pnpm.io/)
- [Tauri](https://tauri.app/)

After things getting ready, enter in the root directory of this project, and run these commands:

```bash
pnpm install
pnpm tauri dev # You can also run `cargo tauri dev`, but you must install Tauri CLI in advance: `cargo install tauri-cli --locked`
```

> [!IMPORTANT]
> **DO NOT** enter in `src-tauri` and run `cargo build`, or the webpage (the GUI) will NOT be included in the program!

And later a window will display on your screen!

> I don't think you want to run and debug this program on WSL. (ᗜ ˰ ᗜ)

To build as release mode, run:

```bash
pnpm tauri build
```