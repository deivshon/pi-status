FROM rust:latest AS BACK_BUILDER
WORKDIR /pi-status

COPY ./back/src ./back/src
COPY ./back/Cargo.toml ./back/Cargo.lock ./back/

WORKDIR /pi-status/back

RUN rustup target add aarch64-unknown-linux-gnu
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo install --path . --target aarch64-unknown-linux-gnu

FROM node:latest AS FRONT_BUILDER
WORKDIR /pi-status

COPY ./front ./front

WORKDIR /pi-status/front/pi-status-front

RUN npm i
RUN npm run build

FROM alpine:latest
WORKDIR /pi-status

COPY --from=BACK_BUILDER /pi-status/back/target/aarch64-unknown-linux-gnu/release/pi-status .
COPY --from=FRONT_BUILDER /pi-status/front/pi-status-front/dist ./front/pi-status-front/dist

CMD /pi-status/pi-status ${ARGS}
