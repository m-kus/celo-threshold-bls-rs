FROM rust:1.76
WORKDIR /root
COPY . /root/dkg-tooling/
#RUN git clone https://github.com/trilitech/dkg-tooling
RUN cd $HOME/dkg-tooling/crates/dkg-cli \
    && RUSTFLAGS="-C target-feature=-crt-static" NO_SOLC_BUILD=1 cargo build --release

FROM ubuntu:22.04
COPY --from=0 /root/dkg-tooling/target/release/dkg-cli /usr/bin/
ENTRYPOINT [ "dkg-cli" ]
