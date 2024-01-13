setup:
    mix deps.get
    mix compile
    just cargo build

format:
    mix format
    just cargo fmt

test:
    just cargo test
    mix test

clean:
    rm -rf _build/ deps/ native/imggrider/target/
    just clean-assets

clean-assets:
    rm -rf test/assets/output/*

cargo +args='':
     (cd native/imggrider && cargo {{args}})
