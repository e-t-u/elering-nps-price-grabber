FROM rust:latest

WORKDIR /home/codespace

COPY . .

RUN bash ./setup.sh

ENV PATH="/root/.cargo/bin:$PATH"
