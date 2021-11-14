# Helium Blockchain Http Server

This is a Rust application to serve up the Helium blockchain as
stored by the
[blockchain-etl](https://github.com/helium/blockchain-etl) service and
schema. The two applications rely on the schema being compatible to
work

## Database

Copy `.env-example` to `.env` and properly configure the `DATABASE_URL`.

## Docker

Run `make docker-build` to build a docker image.
