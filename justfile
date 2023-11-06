test:
    just native-test
    mix test

native-test:
    (cd native && cargo test)
