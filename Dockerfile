# マルチステージビルドを使用し、Rustのプログラムをビルドする
FROM rust:1.78-slim-bookworm AS builder
WORKDIR /app

ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

# ユーザー作成
RUN adduser book && chown -R book /app
USER book
COPY --from=builder ./app/target/release/app ./target/release/app

# ポートを解放し、アプリケーションを起動
ENV PORT 8080
EXPOSE $PORT
ENTRYPOINT ["./target/release/app"]
