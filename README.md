# AXUM COURSE
Course for Axum: An Awesome Web Framework for Rust. 


## debugging

TERMINAL 1: to watch the changes in the source folder
```sh
cargo watch --quiet --clear --watch src/ --exec run
```

TERMINAL 2: to watch the changes and execute the tests within quick_dev
```sh
cargo watch --quiet --clear --watch tests/ --exec "test --quiet quick_dev -- --nocapture"
```
