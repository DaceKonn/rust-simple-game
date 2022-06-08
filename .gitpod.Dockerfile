FROM gitpod/workspace-full:latest

RUN sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev
RUN sudo apt-get install libwayland-dev libxkbcommon-dev
RUN sudo apt-get install lld
RUN rustup update
RUN rustup target install wasm32-unknown-unknown
RUN cargo install wasm-server-runner
