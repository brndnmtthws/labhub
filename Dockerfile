FROM rustlang/rust:nightly

COPY . /labhub/src
WORKDIR /labhub
RUN cd src \
    && cargo install --path . \
    && cd .. \
    && cp src/Rocket.toml . \
    && rm -rf src \
    && rm -rf $HOME/.cargo/registry \
    && rm -rf $HOME/.cargo/git

ENV ROCKET_ENV=production

ENTRYPOINT [ "labhub" ]
