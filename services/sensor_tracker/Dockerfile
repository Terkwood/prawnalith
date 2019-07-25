FROM arm32v7/rust

RUN apt-get update

ENV NIGHTLY_DATE 2019-05-25

ENV RUST_BACKTRACE 1

# ⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️
# ⚠️ specify a known, working version of nightly 
# ⚠️ to avoid Signal 11 mem alloc failures 😩🔥
# ⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️
RUN rustup default nightly-${NIGHTLY_DATE}

WORKDIR /sensor_tracker

COPY . .

# 🏗 make sure rand_hc builds
# 👀 https://github.com/actix/actix/issues/184
# 👀 https://github.com/rust-random/rand/issues/645
RUN cargo update

RUN cargo install --path .

# 🛀 shrink image size
RUN cargo clean
RUN rustup uninstall nightly-${NIGHTLY_DATE}
RUN rustup uninstall 1.36.0-armv7-unknown-linux-gnueabihf
RUN rm -rf /usr/local/cargo/registry
RUN rm -rf /usr/local/cargo/git

CMD ["sensor_tracker"]
