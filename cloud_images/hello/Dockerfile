FROM rust:1.30.1-stretch

RUN rustup default nightly

WORKDIR /usr/src/hello
COPY . .

RUN cargo install --path .

CMD ["hello"]