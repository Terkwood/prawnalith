FROM prawnalith/local/base

RUN apt-get install -y tcl

RUN git clone https://github.com/antirez/redis.git

WORKDIR "/redis"

RUN git checkout 5.0

# use 4 cores
RUN make -j4
RUN make test
RUN make install

ENTRYPOINT tail -f /dev/null
