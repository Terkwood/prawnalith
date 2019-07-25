FROM arm32v7/rust

RUN apt-get update

# ⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️
# ⚠️ specify a known, working version of nightly 
# ⚠️ to avoid Signal 11 mem alloc failures 😩🔥
# ⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️
RUN rustup default nightly-2019-05-25

WORKDIR /sensor_tracker

COPY . .

# 🏗 make sure rand_hc builds
# 👀 https://github.com/actix/actix/issues/184
# 👀 https://github.com/rust-random/rand/issues/645
RUN cargo update

ENV RUST_BACKTRACE 1

RUN cargo install --path .

CMD ["sensor_tracker"]
