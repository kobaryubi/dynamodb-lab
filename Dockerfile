FROM rust

RUN rustup component add rustfmt

WORKDIR /usr/src/app
