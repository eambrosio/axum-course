# AXUM COURSE
Course for Axum: An Awesome Web Framework for Rust. 


## debugging

TERMINAL 1: 
```sh
cargo watch --quiet --clear --watch src/ --exec run
```

TERMINAL 2: 
```sh
cargo watch --quiet --clear --watch tests/ --exec "test --quiet quick_dev -- --nocapture"
```
