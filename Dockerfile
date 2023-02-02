FROM rust:latest AS BACK
WORKDIR /pi-status

COPY ./back/src ./back/src
COPY ./back/Cargo.toml ./back/Cargo.lock ./back/

WORKDIR /pi-status/back

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM node:latest AS FRONT
WORKDIR /pi-status

COPY ./front ./front

WORKDIR /pi-status/front/pi-status-front

RUN npm i
RUN npm run build

FROM alpine:latest
WORKDIR /pi-status

RUN mkdir /pi-status/front/pi-status-front -p
COPY --from=BACK /pi-status/back/target/x86_64-unknown-linux-musl/release/pi-status .
COPY --from=FRONT /pi-status/front/pi-status-front/dist ./front/pi-status-front/dist

RUN mkdir /pst

CMD ./pi-status ${ARGS}
