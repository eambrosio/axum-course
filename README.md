# Web App based on Axum with Auth support

## Test env

1. Open a terminal and run this to watch the changes under the `/src` folder
```bash
cargo watch --quiet --clear --watch src/ --exec run
```

2. Open a 2nd terminal and run this to watch the changes under the /tests folder and execute the sample request:
```bash
cargo watch --quiet --clear --watch tests/ --exec "test --quiet quick_dev -- --nocapture" 
```
and the result will look similar to this:
```
running 1 test

=== Response for GET http://localhost:8080/hello
=> Status         : 200 OK OK
=> Headers        :
   content-type: text/html; charset=utf-8
   content-length: 28
   date: Sat, 04 Nov 2023 15:24:40 GMT
=> Response Body  :
Hello <strong>world</strong>
===

.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```