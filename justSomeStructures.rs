struct triangle {
    base: f32,
    height: f32,
}

impl triangle {
    fn area(&self) -> f32 {
        return 0.5 * self.base * self.height;
    }
}

fn main() {
    let triangle_1 = triangle {
        base: 12.0,
        height: 40.0,
    };
    let testClosure = |test: u32| -> u32 { return test };
    println!("Area: {:.2}", triangle_1.area());
    println!("Test: {}", testClosure(31));
}
