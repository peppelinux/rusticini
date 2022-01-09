Setup
-----

````
cargo new infinite-loop
````

Cargo.toml
----------

````
[package]
name = "infinite-loop"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.4"
````

source
------

````
use rand::prelude::*;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    println!("Starting infinite test loop");
    let mut random_int: u8;
    loop {
        random_int = random();
        println!("Hello World! Value Is :{}", random_int);
        sleep(Duration::from_millis(2000));
    }
}
````
