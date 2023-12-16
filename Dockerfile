FROM rust

WORKDIR /app

RUN cargo install sqlx-cli

COPY Cargo.toml Cargo.toml

COPY Cargo.lock Cargo.lock

RUN mkdir src && echo 'fn main() {panic!("not ready");}' > src/main.rs

RUN cargo build --release --locked

RUN rm -rf src

COPY src src

RUN touch src/main.rs && cargo build --release --locked

COPY migrations migrations

CMD cargo run --release --locked
