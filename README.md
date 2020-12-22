# sensitive-server
base on async-std framework tide

> `multi crate`

The business code layer adopts `DDD` idea

- web layer
- application layer
- domain layer
- infrastructure layer

## debug-env for dcoker

```shell
docker run -i --rm postgres cat /usr/share/postgresql/postgresql.conf.sample > my-postgres.conf;
docker run \
-d \
--name sensitive_server \
-v "$PWD/my-postgres.conf":/etc/postgresql/postgresql.conf \
-e POSTGRES_USER=postgres \
-e POSTGRES_PASSWORD=password \
-e POSTGRES_DB=sensitive_server \
-p 5434:5432 postgres -c 'config_file=/etc/postgresql/postgresql.conf'
```

## Unit test

### application layer

### domain layer

### infrastructure layer

```shell
RUST_BACKTRACE=1 cargo test --package sensitive-server-infrastructure
   Compiling sensitive-server-web v0.1.0 (/Users/caiwenhui/www/local/rust/sensitive-server/src/web)
   Compiling sensitive-server-infrastructure v0.1.0 (/Users/caiwenhui/www/local/rust/sensitive-server/src/infrastructure)
    Finished test [unoptimized + debuginfo] target(s) in 2.36s
     Running target/debug/deps/sensitive_server_infrastructure-292965eebfa551cd

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/conduit_regular-e3759796e037a400

running 3 tests
test test_create_regular ... ok
test lists_regulars ... ok
test delete_regular ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/conduit_word-da76817d28748c59

running 3 tests
test test_create_word ... ok
test lists_words ... ok
test delete_words ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests sensitive_server_infrastructure

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


```