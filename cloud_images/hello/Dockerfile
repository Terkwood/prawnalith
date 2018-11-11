FROM rust:1.30.1-stretch

WORKDIR /usr/src/hello
COPY . .

RUN rustup default nightly

RUN cargo install --path .

CMD ["hello"]