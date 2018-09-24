FROM prawnalith/local/base

# provides mosquitto
RUN echo "deb http://archive.raspbian.org/raspbian jessie main contrib non-free" >> /etc/apt/sources.list
RUN apt-get update
# should be able to improve this
RUN wget http://repo.mosquitto.org/debian/mosquitto-repo.gpg.key 
RUN apt-key add mosquitto-repo.gpg.key
RUN wget -P /etc/apt/sources.list.d/ http://repo.mosquitto.org/debian/mosquitto-jessie.list
# Required for old libssl which mosquitto depends on
# see https://github.com/eclipse/mosquitto/issues/529#issuecomment-356787605
RUN echo "deb http://archive.raspbian.org/raspbian jessie main contrib non-free" >> /etc/apt/sources.list
RUN apt-get install -y mosquitto --allow-unauthenticated
RUN apt-get install -y mosquitto-clients --allow-unauthenticated

# Expose for mosquitto
EXPOSE 1883
EXPOSE 8883

ENTRYPOINT /usr/sbin/mosquitto
