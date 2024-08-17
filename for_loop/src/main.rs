use std::clone;

struct VecElement
{
    name: String,
    index: i32,
    inner_vector: Vec<Vec<i32>>,
}

fn main() {
    println!("Hello, world!");

    let vector = Vec::<VecElement>::new();

    for e in &vector {
        let e_ = e;
        for ee in &e_.inner_vector {
            let ee_ = ee;
        }
    }
}
