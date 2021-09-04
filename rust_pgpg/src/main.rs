//use std::vec;
// fn leibniz_common<F>(terms: u32, pow: &F) -> f64
//     where F: Fn(u32) -> f64
// {
//     let mut s = 0.0;
//     for n in 0..(terms + 1) {
//         s+= pow(n) / (2 * n + 1) as f64;
//     }
//     s * 4.0
// }
fn vec_add() {
    const SIZE: usize = 10;
    let mut vec1: Vec<u32> = Vec::with_capacity(SIZE);
    let mut vec2: Vec<u32> = Vec::with_capacity(SIZE);

    for n in 0..(SIZE - 1) {
        let ele: u32 = n as u32;
        vec1.push(ele);
        vec2.push(ele*2);
    }

    for n in 0..(SIZE - 1) {
        println!("{}", vec1[n].to_string());
    }
}

fn main() {
    println!("Hello, world!");
    let num = 212;
    println!("num: {}", num.to_string());
    vec_add();
}

