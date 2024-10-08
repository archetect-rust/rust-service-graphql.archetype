alias ut := test-ut
alias it := test-it
alias all := test-all

test-ut:
    cargo test --lib --bins

test-it:
    cargo test --test '*'
                                                  
test-all:
    cargo test

test TEST:
    cargo test --test {{'{'}}{ TEST }}

install-defaults := ''

install install-options=install-defaults:
    cargo install --path crates/{{ project_name }}_bin/ {{'{'}}{ install-options }}

run-defaults := '{% if persistence != "None" %}--temp-db{% endif %}'

run run-options=run-defaults:
    cargo run -- {{'{'}}{ run-options }}

build-defaults := ''

build build-options=build-defaults:
    cargo build {{'{'}}{ build-options }}
