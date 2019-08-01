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

#TODO
CMD ["tail","-f","/dev/null"]

#TODO 
#RUN cargo install --path .

# 🛀 shrink image size
#TODO
#RUN sh shrink_docker_image.sh

#TODO
#CMD ["sensor_tracker"]
