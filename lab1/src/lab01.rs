fn prim(x: i32) -> i32 {
    if x < 2 || x % 2 == 0 {
        return 0;
    } else {
        let mut d = 3;
        while d <= x / 2 {
            if x % d == 0 {
                return 0;
            }
            d += 2;
        }
    }
    return 1;
}

fn prime_100() {
    let mut i = 0;
    while i < 101 {
        if prim(i) != 0 {
            println!("Numarul {} este prim", i);
        }
        i += 1;
    }
}

fn coprime(a: i32, b: i32) {
    let mut x = a;
    let mut y = b;
    while x != y && a != 0 && b != 0 {
        if x > y {
            x -= y;
        } else {
            y -= x;
        }
    }
    if x == 1 && a != b {
        println!("Numerele {} si {} sunt coprime", a, b);
    } else {
        println!("Numerele {} si {} nu sunt coprime", a, b);
    }
}

fn perechi_coprime() {
    let mut i = 0;
    while i < 101 {
        let mut j = 0;
        while j < 101 {
            coprime(i, j);
            j += 1;
        }
        i += 1;
    }
}
fn trei() -> i32 {
    3
}

fn beri() {
    let mut i = 99;
    while i > 1 {
        println!("{} bottles of beer on the wall,", i);
        println!("{} bottles of beer,", i);
        println!("Take one down, pass it around,");
        println!("{} bottles of beer on the wall,", i - 1);
        println!("");
        i -= 1;
    }

    println!("{} bottle of beer on the wall,", i);
    println!("{} bottle of beer,", i);
    println!("Take one down, pass it around,");
    println!("No bottles of beer on the wall.");
    println!("No bottles of beer.");
    println!("Go to the store, buy some more,");
    println!("99 bottles of beer on the wall.");
}

fn main() {
    if prim(2) == 0 {
        println!("Numarul nu este prim");
    } else {
        println!("Numarul este prim");
    }
    println!("");
    prime_100();
    println!("");
    coprime(7, 12);
    println!("");
    perechi_coprime();
    println!("");
    beri();
    println!("{}", trei());
}
