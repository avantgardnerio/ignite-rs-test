use std::any::Any;
use std::borrow::Borrow;
use ignite_rs::{ClientConfig, Ignite};
use ignite_rs::cache::Cache;

use ignite_rs_derive::IgniteObj;

fn main() {
    let client_config = ClientConfig::new("localhost:10800");
    let mut ignite = ignite_rs::new_client(client_config).unwrap();
    let names = ignite.get_cache_names().unwrap();
    for name in names.iter() {
        println!("Got: {}", name);
        let cfg = ignite.get_cache_config(name);
        let cfg = cfg.unwrap();
        let entities = cfg.query_entities.unwrap();
        for entity in entities.iter() {
            println!("entity {:?}", entity.table);
            for field in entity.query_fields.iter() {
                println!("   field {} {}", field.name, field.type_name)
            }
        }
    }

    // let region: Cache<i64, BinaryObject>  = ignite.get_or_create_cache("SQL_PUBLIC_REGION").unwrap();
    // let res = region.get(&0i64).unwrap();
    // ignite.get_binary_type();

    // Create a typed cache named "test"
    let hello_cache: Cache<MyType, MyOtherType> = ignite
        .get_or_create_cache::<MyType, MyOtherType>("test")
        .unwrap();

    let key = MyType {
        bar: "AAAAA".into(),
        foo: 999,
    };
    let val = MyOtherType {
        list: vec![Some(FooBar {})],
        arr: vec![-23423423i64, -2342343242315i64],
    };

    // Put value
    hello_cache.put(&key, &val).unwrap();

    let records = hello_cache.query_scan(1024).unwrap();
    for record in records {
        println!("{:?}={:?}", record.0, record.1);
    } 

    // Retrieve value
    println!("{:?}", hello_cache.get(&key).unwrap());
}

// Define your structs, that could be used as keys or values
#[derive(IgniteObj, Clone, Debug)]
struct MyType {
    bar: String,
    foo: i32,
}

#[derive(IgniteObj, Clone, Debug)]
struct MyOtherType {
    list: Vec<Option<FooBar>>,
    arr: Vec<i64>,
}

#[derive(IgniteObj, Clone, Debug)]
struct FooBar {}
