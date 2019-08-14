FROM rustlang/rust:nightly

WORKDIR /labhub

ENV SCCACHE_VER=0.2.10
ENV GITHUB_RELEASE_VER=v0.7.2

RUN wget -q https://github.com/mozilla/sccache/releases/download/${SCCACHE_VER}/sccache-${SCCACHE_VER}-x86_64-unknown-linux-musl.tar.gz -O sccache-${SCCACHE_VER}-x86_64-unknown-linux-musl.tar.gz \
  && tar xf sccache-${SCCACHE_VER}-x86_64-unknown-linux-musl.tar.gz \
  && cp sccache-${SCCACHE_VER}-x86_64-unknown-linux-musl/sccache /usr/bin \
  && apt-get update \
  && apt-get install -y cmake curl \
  && rm -rf /var/lib/apt/lists/*

RUN rustup component add clippy \
  && rustup component add rustfmt \
  && rustup target add i686-apple-darwin \
  && rustup target add i686-pc-windows-gnu \
  && rustup target add i686-pc-windows-msvc \
  && rustup target add i686-unknown-linux-gnu \
  && rustup target add x86_64-apple-darwin \
  && rustup target add x86_64-pc-windows-gnu \
  && rustup target add x86_64-pc-windows-msvc \
  && rustup target add x86_64-unknown-freebsd \
  && rustup target add x86_64-unknown-linux-gnu \
  && RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install cargo-tarpaulin \
  && curl -Lq https://github.com/aktau/github-release/releases/download/${GITHUB_RELEASE_VER}/linux-amd64-github-release.tar.bz2 -o linux-amd64-github-release.tar.bz2 \
  && tar xvf linux-amd64-github-release.tar.bz2 \
  && mv bin/linux/amd64/github-release /usr/bin \
  && rm -rf $CARGO_HOME/registry \
  && rm -rf $CARGO_HOME/git

ENV RUSTC_WRAPPER=sccache
