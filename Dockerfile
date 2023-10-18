FROM rust:1.73

WORKDIR /usr/src/automeme
COPY . .

RUN cargo install --locked --path .

CMD ["automeme"]