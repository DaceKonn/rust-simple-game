# Gitpod config alternate

```yaml
tasks:
  - before: |
      sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev &&
      sudo apt-get install libwayland-dev libxkbcommon-dev
    init: |
      rustup target install wasm32-unknown-unknown &&
      cargo install wasm-server-runner &&
      cargo build
    command: |
      cargo run --target wasm32-unknown-unknown
vscode:
  extensions:
    - matklad.rust-analyzer
    - bungcip.better-toml
    - streetsidesoftware.code-spell-checker
    - serayuzgur.crates
```