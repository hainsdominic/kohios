FROM rust:1.63

WORKDIR /usr/src/kohios
COPY . .

RUN cargo install --path .

CMD bash -c "kohios"