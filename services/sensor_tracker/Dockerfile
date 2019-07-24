FROM arm32v7/rust

RUN apt-get update

RUN rustup default nightly-2019-05-25

COPY . .

RUN cargo install --path .

CMD ["sensor_tracker"]

