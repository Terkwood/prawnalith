FROM arm32v7/debian

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.29.0

# provides mosquitto
RUN echo "deb http://archive.raspbian.org/raspbian jessie main contrib non-free" >> /etc/apt/sources.list
RUN apt-get update
RUN apt-get install -my wget gnupg curl
RUN wget http://repo.mosquitto.org/debian/mosquitto-repo.gpg.key 
RUN apt-key add mosquitto-repo.gpg.key
RUN wget -P /etc/apt/sources.list.d/ http://repo.mosquitto.org/debian/mosquitto-jessie.list
RUN apt-get install -y apt-transport-https
# Required for old libssl which mosquitto depends on
# see https://github.com/eclipse/mosquitto/issues/529#issuecomment-356787605
RUN echo "deb http://archive.raspbian.org/raspbian jessie main contrib non-free" >> /etc/apt/sources.list
RUN apt-get install -y mosquitto --allow-unauthenticated
RUN apt-get install -y mosquitto-clients --allow-unauthenticated

RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
        amd64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='f69dafcca62fe70d7882113e21bb96a2cbdf4fc4636d25337d6de9191bdec8da' ;; \
        armhf) rustArch='armv7-unknown-linux-gnueabihf'; rustupSha256='eee969b9fd128e8dc9b4ec44acde46735cf8e612d06495e9d022517849aba2d6' ;; \
        arm64) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='cdc48b7882582fd8475107a406dd86df85c7d72e5deea99ff8940c8e11531285' ;; \
        i386) rustArch='i686-unknown-linux-gnu'; rustupSha256='3bad3945452509ac28ba4113e198323daab57488d6885bb31ac30c9eecd88825' ;; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    url="https://static.rust-lang.org/rustup/archive/1.13.0/${rustArch}/rustup-init"; \
    wget "$url"; \
    echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain $RUST_VERSION; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
rustc --version;

RUN apt-get install -y git
#RUN git clone https://github.com/SergioBenitez/Rocket
#WORKDIR "/Rocket"
#RUN git checkout v0.3.16
#WORKDIR "/Rocket/examples/hello_world"
#RUN apt-get install -y gcc
#RUN rustup default nightly
#RUN cargo build

# Expose for mosquitto
EXPOSE 1883
EXPOSE 8883

ENTRYPOINT tail -f /dev/null
