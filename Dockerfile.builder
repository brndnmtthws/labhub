FROM rustlang/rust:nightly

WORKDIR /labhub

ENV SCCACHE_VER=0.2.8
RUN wget -q https://github.com/mozilla/sccache/releases/download/${SCCACHE_VER}/sccache-${SCCACHE_VER}-x86_64-unknown-linux-musl.tar.gz -O sccache-${SCCACHE_VER}-x86_64-unknown-linux-musl.tar.gz \
  && tar xf sccache-${SCCACHE_VER}-x86_64-unknown-linux-musl.tar.gz \
  && cp sccache-${SCCACHE_VER}-x86_64-unknown-linux-musl/sccache /usr/bin
ENV RUSTC_WRAPPER=sccache
RUN rustup default nightly-2019-02-25 \
  && rustup self update \
  && rustup component add clippy \
  && rustup component add rustfmt

