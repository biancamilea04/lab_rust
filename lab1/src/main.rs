fn numar() -> u32 {
    3
}

fn o_suma() {
    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }
    let x: i32 = sum(21, 23);
    print!("Suma mea este {}", x);
}

fn numele_meu() {
    let x = numar();
    let y = {
        let temp: u32 = x;
        let a: u32 = x + x;
        a + temp * 2
    };
    println!("Am {} beri", y);
}

fn main() {
    println!("Hello, world!");
    let rabbits: &str = "cinci";
    println!("Hello world! I have {} rabbits.", rabbits);
    numele_meu();
    o_suma();
}
