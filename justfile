setup:
    mix deps.get
    mix compile
    just native-cargo build

format:
    mix format
    just native-cargo fmt

test:
    just native-cargo test
    mix test

clean:
    rm -rf _build/ deps/ native/imggrider/target/

native-cargo +args='':
     (cd native/imggrider && cargo {{args}})
