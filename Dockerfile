FROM rustlang/rust:nightly

COPY . /labhub/src
WORKDIR /labhub
RUN cd src \
    && cargo install --path . \
    && cd .. \
    && rm -rf src

ENTRYPOINT [ "labhub" ]
