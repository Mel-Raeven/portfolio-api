FROM rust:latest

WORKDIR /usr/src/portfolioCMS
COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install --path .

CMD ["portfolioCMS"]
