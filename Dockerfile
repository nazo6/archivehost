# build frontend
FROM node:20-slim AS frontend
WORKDIR /app
RUN npm i -g pnpm
COPY ./frontend ./frontend
COPY ./package.json ./package.json
COPY ./pnpm-lock.yaml ./pnpm-lock.yaml
COPY ./pnpm-workspace.yaml ./pnpm-workspace.yaml
RUN ls -la
RUN pnpm install --frozen-lockfile
RUN pnpm build:front

# build rust binary
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY ./archivehost .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY ./archivehost .
COPY --from=frontend /app/frontend/dist ../frontend/dist
RUN cargo build --release

# runtime
FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/archivehost /usr/local/bin
ENTRYPOINT ["/usr/local/bin/archivehost", "serve"]
