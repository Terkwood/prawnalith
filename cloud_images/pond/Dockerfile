# ATTRIBUTION!  Thanks https://github.com/emk/rust-musl-builder/blob/master/examples/using-diesel/Dockerfile
#               The bulk of the goodness in this file is derived from the above example.
#
# Builds a Rust executable using this image,
# and deploys it to a tiny Alpine Linux container.

# Our first FROM statement declares the build environment.
FROM ekidd/rust-musl-builder AS builder

# Switch to nightly, since 2018 edition (and others) require it
RUN rustup default nightly

RUN rustup target install x86_64-unknown-linux-musl

# Add our source code.
ADD . ./

# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
RUN cargo build --release

# Now, we need to build our _real_ Docker container, copying in `pond`.
FROM alpine:latest
RUN apk --no-cache add ca-certificates curl
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/pond \
    /usr/local/bin/

# .env, Rocket.toml, etc will be mounted here
WORKDIR /data

HEALTHCHECK CMD curl --fail --insecure https://0.0.0.0:8000/ping || exit 1

CMD [ "/usr/local/bin/pond" ]
