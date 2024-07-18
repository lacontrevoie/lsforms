FROM rust:1-bookworm AS builder

WORKDIR /run_dir

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt update \
    && apt -y install libpq5 ca-certificates \
    && apt clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /run_dir

# config.toml will be missing and needs to be mounted
COPY --from=builder /run_dir/target/release/constello ./

COPY --from=builder /run_dir/emails ./emails

COPY --from=builder /run_dir/static ./static

RUN adduser --disabled-password --gecos "" --no-create-home "unprivileged"

USER unprivileged

CMD ["/run_dir/constello"]
