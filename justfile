_:
    just --list

test:
    cargo test --all-features

bench:
    cargo bench --all-features

format:
    cargo fmt --all

lint:
    cargo clippy --all-features --all-targets -- -D warnings
