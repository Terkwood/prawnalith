FROM prawnalith/local/rust
RUN git clone https://github.com/Terkwood/prawnalith.git
WORKDIR "/prawnalith/services/led_status_helper"
RUN cargo build --release
ENTRYPOINT tail -f /dev/null
