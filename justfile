
default: wbd

wbd:
    cargo run --bin wbd

wbcli:
    cargo run --bin wbcli

devsetup:
    cp dev/hooks/* .git/hooks

fmt:
    cargo +nightly fmt --all

lint:
    cargo clippy -- -W clippy::unwrap_used -W clippy::cargo
