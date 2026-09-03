trait Consume {
    fn consume(self) -> i32;
}

struct Thing(i32);

impl Consume for Thing {
    fn consume(self) -> i32 {
        self.0
    }
}

// Works: a generic bound monomorphizes, so each caller gets a
// consume() specialized to a concrete, sized type.
fn use_generic<T: Consume>(c: T) -> i32 {
    c.consume()
}

fn main() {
    let thing = Thing(42);
    println!("Consumed: {}", use_generic(thing));
}
