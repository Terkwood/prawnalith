FROM prawnalith/local/base

RUN apt-get install -y cmake libssl-dev libwebsockets-dev uuid-dev libc-ares-dev

WORKDIR "/tmp__build"

ENV C_ARES_VERSION=1.14.0

RUN wget http://c-ares.haxx.se/download/c-ares-$C_ARES_VERSION.tar.gz
RUN tar -xvf c-ares-$C_ARES_VERSION.tar.gz

WORKDIR "/tmp__build/c-ares-$C_ARES_VERSION"

RUN ./configure
# -j4 switch allows building on all 4 cores
RUN make -j4
RUN make install

WORKDIR "/tmp__build"

ENV MOSQUITTO_VERSION=1.5.3

RUN wget http://mosquitto.org/files/source/mosquitto-$MOSQUITTO_VERSION.tar.gz
RUN tar xavf mosquitto-$MOSQUITTO_VERSION.tar.gz

WORKDIR "/tmp__build/mosquitto-$MOSQUITTO_VERSION"

RUN apt-get install -y g++
RUN cmake -DWITH_WEBSOCKETS=ON .
RUN make -j4
RUN make install

WORKDIR "/"

RUN rm -rf /tmp__build

RUN useradd -r -m -d /var/lib/mosquitto -s /usr/sbin/nologin -g nogroup mosquitto

# Expose for mosquitto
EXPOSE 1883
EXPOSE 8883

ENTRYPOINT /usr/local/sbin/mosquitto
