FROM gitpod/workspace-full:latest

RUN sudo install-packages g++ pkg-config libx11-dev libasound2-dev libudev-dev
RUN sudo install-packages libwayland-dev libxkbcommon-dev
RUN sudo install-packages lld
RUN rustup update
RUN rustup target install wasm32-unknown-unknown
RUN cargo install wasm-server-runner
