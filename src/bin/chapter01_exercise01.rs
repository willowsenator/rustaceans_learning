fn main() {
    let mut z = Box::new(0);

    for i in 0..5 {
        *z = i;
    }

    println!("Last z: {}", z);
}
