FROM prawnalith/local/rust

WORKDIR "/Rocket"
RUN git checkout v0.3.16
WORKDIR "/Rocket/examples/hello_world"
RUN rustup default nightly
RUN rustup update && cargo update
RUN cargo build --release

EXPOSE 8000
ENTRYPOINT ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=80 /Rocket/target/release/hello_world

