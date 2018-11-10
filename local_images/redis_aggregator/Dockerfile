FROM prawnalith/local/rust

RUN apt-get install -y pkg-config libssl-dev

WORKDIR /prawnalith/services/redis_aggregator

ENV BUILD_MODE="release"
ENV CARGO_FLAGS="--${BUILD_MODE}"

RUN cargo build ${CARGO_FLAGS}

ENTRYPOINT [ "target/${BUILD_MODE}/redis_aggregator" ]
