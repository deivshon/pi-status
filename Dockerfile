FROM rust:1.67 AS BACK

WORKDIR /pi-status
COPY ./back ./back

WORKDIR /pi-status/back

RUN cargo install --path .

FROM node:latest
WORKDIR /pi-status
COPY ./front ./front

WORKDIR /pi-status/front/pi-status-front

RUN npm i
RUN npm run build

RUN mkdir /pst
WORKDIR /pi-status

COPY --from=BACK /pi-status/back/target/release/pi-status .
CMD ./pi-status ${ARGS}
