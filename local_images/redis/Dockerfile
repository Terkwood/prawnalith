FROM prawnalith/local/base

RUN apt-get install -y tcl

RUN git clone https://github.com/antirez/redis.git

WORKDIR "/redis"

RUN git checkout 5.0

# use 4 cores
RUN make -j4
# Error when running tests on pi 3 B+:
# *** [err]: Server is able to generate a stack trace on selected systems in tests/integration/logging.tcl
#RUN make test   
RUN make install

EXPOSE 6379

ENTRYPOINT src/redis-server
