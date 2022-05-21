# Apache Ignite Test in Rust

This repo proves we can talk to Apache Ignite from Rust

## Running

```shell
docker-compose up -d
cargo run
```

## Results

Results should look something like:

```shell
/snap/bin/cargo run --color=always --package igniterstest --bin igniterstest
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/igniterstest`
ALL caches: []
Some(MyOtherType { list: [Some(FooBar)], arr: [-23423423, -2342343242315] })

Process finished with exit code 0
```