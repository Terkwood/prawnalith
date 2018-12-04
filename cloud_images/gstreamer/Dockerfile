FROM maxmcd/gstreamer

RUN apt-get update
RUN apt-get install -y gstreamer1.0-plugins-base
RUN apt-get install -y gstreamer1.0-plugins-good
RUN apt-get install -y gstreamer1.0-plugins-bad
RUN apt-get install -y gstreamer1.0-plugins-ugly
RUN apt-get install -y git vim
RUN echo Charlie 311
RUN git clone https://github.com/Terkwood/prawnalith
WORKDIR "/prawnalith"
RUN git checkout EXPERIMENTAL_MAGIC
WORKDIR "/prawnalith/cloud_images/gstreamer"
ENTRYPOINT ["sh", "recv.sh"]
