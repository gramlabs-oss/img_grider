test:
    just native-test
    mix test

native-test:
    just native-cargo test

native-cargo +args='':
     (cd native/imggrider && cargo {{args}})
