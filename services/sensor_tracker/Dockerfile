FROM arm32v7/rust

RUN apt-get update

RUN rustup default nightly

COPY . .

RUN cargo install --path .

CMD ["sensor_tracker"]

