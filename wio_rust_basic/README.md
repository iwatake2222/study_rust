# Basic project for Wio Terminal + Rust
- UART
- Panic Handler using UART
- LCD

## Setup (Windows)
- Install Rust
- Install Cross build tool + binary tool
```
rustup target add thumbv7em-none-eabihf
cargo install cargo-generate
cargo install hf2-cli
cargo install cargo-hf2
```
- Install SDL (optional)
    - https://www.libsdl.org/release/SDL2-devel-2.0.16-VC.zip
    - Copy `lib/x64/*.lib` to `C:\Users\iwatake\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib\.`
    - Copy `lib/x64/SDL2.dll` to `./SDL2.dll`

