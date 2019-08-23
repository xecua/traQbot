FROM rustlang/rust:nightly

RUN cargo install diesel_cli --no-default-features --features mysql
