FROM prawnalith/local/rust

RUN apt-get install -y pkg-config libssl-dev

WORKDIR /prawnalith/services/redis_aggregator

RUN cargo build --release

ENTRYPOINT [ "target/release/redis_aggregator" ]
