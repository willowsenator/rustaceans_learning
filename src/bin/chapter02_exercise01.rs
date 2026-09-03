// C sized
#[repr(C)]
struct CSeveralTypes {
    tiny: bool,
    normal: u32,
    small: u8,
    long: u64,
    short: u16,
}

impl CSeveralTypes {
    fn new(tiny: bool, normal: u32, small: u8, long: u64, short: u16) -> Self {
        Self {
            tiny,
            normal,
            small,
            long,
            short,
        }
    }
}

// Rust sized
// Fields are never read individually — this struct exists only to compare its size_of_val against CSeveralTypes
#[allow(dead_code)]
struct SeveralTypes {
    tiny: bool,
    normal: u32,
    small: u8,
    long: u64,
    short: u16,
}

impl SeveralTypes {
    fn new(tiny: bool, normal: u32, small: u8, long: u64, short: u16) -> Self {
        Self {
            tiny,
            normal,
            small,
            long,
            short,
        }
    }
}

fn main() {
    let c_several_types = CSeveralTypes::new(false, 2, 5, 7, 8);
    let rust_several_types = SeveralTypes::new(false, 2, 5, 7, 8);
    println!("C sized : {}", size_of_val(&c_several_types));
    println!("Rust sized : {}", size_of_val(&rust_several_types));
}
