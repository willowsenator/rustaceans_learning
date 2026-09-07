mod sealed {
    pub trait Sealed {}
}

trait Frobnicate: sealed::Sealed {
    fn frobnicate(&self) -> String;
}

struct Widget;

impl sealed::Sealed for Widget {}
impl Frobnicate for Widget {
    fn frobnicate(&self) -> String {
        "frobnicate widget".to_string()
    }
}

fn main() {
    println!("{}", Widget::frobnicate(&Widget));
}
