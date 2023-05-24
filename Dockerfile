FROM rust:1.69-bookworm AS build
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN apt update &&  apt-get -y install make cmake libprotobuf-dev protobuf-compiler libhdf5-dev libzstd-dev git
WORKDIR /opt
RUN git clone https://github.com/wkusmirek/Icarust
WORKDIR /opt/Icarust
RUN cargo build --release
RUN mkdir -p /opt/ont/minknow/conf
COPY cert /opt/ont/minknow/conf/rpc-certs
