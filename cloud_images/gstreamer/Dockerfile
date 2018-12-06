FROM maxmcd/gstreamer

RUN apt-get update
RUN apt-get install -y gstreamer1.0-plugins-base
RUN apt-get install -y gstreamer1.0-plugins-good
RUN apt-get install -y gstreamer1.0-plugins-bad
RUN apt-get install -y gstreamer1.0-plugins-ugly
RUN apt-get install -y git vim
RUN echo Peaceful Stream Vintage
RUN mkdir /stream
WORKDIR "/stream"
ADD . ./
ENTRYPOINT ["sh", "recv.sh"]
