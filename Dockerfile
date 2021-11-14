# Build container
FROM rust:alpine3.14 as build

# We are indirectly depending on libbrotli.
RUN apk update && apk add libc-dev

WORKDIR /usr/src/helium-blockchain-http

COPY . .

ENV RUSTFLAGS -Ctarget-feature=-crt-static
RUN cargo build --release
RUN strip target/release/helium-blockchain-http

# Slim output image not containing any build tools / artefacts
FROM alpine:latest

RUN apk add libgcc

COPY --from=build /usr/src/helium-blockchain-http/target/release/helium-blockchain-http /usr/bin/helium-blockchain-http

CMD ["helium-blockchain-http"]
