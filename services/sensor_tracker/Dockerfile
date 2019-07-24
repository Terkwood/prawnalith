FROM arm32v7/rust

RUN apt-get update

# specify a known, working version of nightly 
# to avoid Signal 11 mem alloc failures
RUN rustup default nightly-2019-05-25

RUN apt-get install -y g++ cmake

COPY . .

#RUN cargo install --path .

CMD ["tail","-f","/dev/null"]
#CMD ["sensor_tracker"]

