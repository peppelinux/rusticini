#[derive(Debug)]
struct Struct{
    a:String,
    b:i32
}

fn main() {
    let c = Struct{a : "giuseppe".to_string(), b : 67};
    println!("{:?}", c);
}
