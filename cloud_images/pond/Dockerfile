FROM rust:1.30.1-stretch

RUN rustup default nightly

WORKDIR /var/pond
COPY . .

RUN cargo install --path .

CMD ["pond"]
