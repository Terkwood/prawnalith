FROM prawnalith/local/rust

RUN apt-get update
RUN apt-get install -y pkg-config libssl-dev

WORKDIR /prawnalith/services/redis_aggregator

RUN cargo build --release

RUN cargo install --path . 

WORKDIR /data

ENTRYPOINT [ "redis_aggregator" ]
