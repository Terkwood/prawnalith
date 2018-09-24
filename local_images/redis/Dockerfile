FROM prawnalith/local/base

RUN git clone https://github.com/antirez/redis.git

WORKDIR "/redis"

RUN git checkout 5.0

# use 4 cores
RUN make -j4
RUN make install
RUN make test

ENTRYPOINT tail -f /dev/null
