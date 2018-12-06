FROM arm32v7/debian:buster-slim

RUN apt-get update -y
RUN apt-get install -y libgstreamer1.0-0 gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly gstreamer1.0-libav gstreamer1.0-doc gstreamer1.0-tools gstreamer1.0-x gstreamer1.0-alsa gstreamer1.0-gl gstreamer1.0-gtk3  gstreamer1.0-pulseaudio
RUN apt-get install -y git vim
RUN git clone https://github.com/Terkwood/prawnalith
WORKDIR "/prawnalith"
RUN git checkout feature/video_init
WORKDIR "/prawnalith/local_images/gstreamer"
ENTRYPOINT ["sh", "recv_stream_to_hls.sh"]
