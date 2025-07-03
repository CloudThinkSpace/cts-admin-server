# Compile
FROM    rust:1.80.1-alpine3.20 AS compiler

RUN     apk add -q --update-cache --no-cache build-base openssl-dev cmake
WORKDIR /
ENV     RUSTFLAGS="-C target-feature=-crt-static"
COPY    . .
RUN     cargo build --bin web --release
RUN     cargo install sea-orm-cli


# Run
FROM    alpine:3.20

RUN     apk update --quiet \
        && apk add -q --no-cache libgcc tini curl openssl

COPY    --from=compiler /target/release/web /app/web
COPY    --from=compiler /config.toml /app/config.toml
COPY    --from=compiler /.env /app/.env

EXPOSE  3000

ENTRYPOINT ["tini", "--"]
WORKDIR /app
CMD   ["./web"]