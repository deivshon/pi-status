
ARG PLATFORM

FROM --platform=${PLATFORM} rust:slim-bookworm AS BACK_BUILDER
ARG CARGO_TARGET

WORKDIR /pi-status
COPY ./back/ ./back/
WORKDIR /pi-status/back

RUN RUSTFLAGS='-C target-feature=+crt-static' cargo install --path . --target ${CARGO_TARGET}

FROM --platform=${PLATFORM} node:latest AS FRONT_BUILDER

WORKDIR /pi-status
COPY ./front ./front
WORKDIR /pi-status/front/pi-status-front

RUN npm i
RUN npm run build

FROM --platform=${PLATFORM} alpine:latest
ARG CARGO_TARGET

WORKDIR /pi-status

COPY --from=BACK_BUILDER /pi-status/back/target/${CARGO_TARGET}/release/pi-status .
COPY --from=FRONT_BUILDER /pi-status/front/pi-status-front/dist ./front/pi-status-front/dist

CMD /pi-status/pi-status ${ARGS}
