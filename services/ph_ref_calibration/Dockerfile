FROM arm32v7/rust

RUN apt-get update

ENV NIGHTLY_DATE 2019-05-25

ENV RUST_BACKTRACE 1

# 🚀 rocket.rs requires nightly
# ⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️
# ⚠️ specify a known, working version of nightly 
# ⚠️ to avoid Signal 11 mem alloc failures 😩🔥
# ⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️
RUN rustup default nightly-${NIGHTLY_DATE}

WORKDIR /ph_ref_calibration

COPY . .

# 🏗 satisfy rocket, ring, cookie
RUN cargo update

RUN cargo install --path .

# 🛀 shrink image size
RUN sh shrink_docker_image.sh

CMD ["ph_ref_calibration"]
