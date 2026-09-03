trait Consume {
    fn consume(self: Box<Self>) -> i32;
}

struct Thing(i32);

impl Consume for Thing {
    fn consume(self: Box<Self>) -> i32 {
        self.0
    }
}

// Works: consume() takes ownership through a Box, so calling it only
// ever moves the fat pointer (data ptr + vtable ptr), never the unsized
// `dyn Consume` value itself. That's why `self: Box<Self>` methods are
// dyn-compatible even though plain `self` methods aren't.
fn use_dyn(c: Box<dyn Consume>) -> i32 {
    c.consume()
}

fn main() {
    let thing: Box<dyn Consume> = Box::new(Thing(42));
    println!("Consumed: {}", use_dyn(thing));
}
