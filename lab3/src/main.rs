///////////////////////// P1 /////////////////////////

fn prime(nr: u16) -> u16 {
    if nr < 2 || (nr % 2 == 0 && nr > 2) {
        return 0;
    };
    let mut d = 3;
    while d * d <= nr {
        if nr % d == 0 {
            return 0;
        };
        d += 2;
    }
    return 1;
}

fn next_prime1_1(x: u16) -> Option<u16> {
    let mut var = x + 1;
    while var > x && var <= u16::MAX {
        if prime(var) == 1 {
            return Some(var);
        }
        var += 1;
    }
    return None;
}

fn next_prime1_2(x: u16) -> Result<u16, &'static str> {
    let mut var = x + 1;
    while var > x && var <= u16::MAX {
        if prime(var) == 1 {
            return Ok(var);
        }
        var += 1;
    }
    if var >= u16::MAX {
        return Err("Valoarea maxima este depasita");
    }
    return Err("nimic");
}

fn main_ex1_1() {
    let r = next_prime1_1(5);
    if r.is_some() {
        println!("Exista {}", r.unwrap());
    } else {
        println!("None");
    }
}

fn main_ex1_2() {
    let r = next_prime1_2(5);
    if r.is_ok() {
        println!("Exista {}", r.unwrap());
    } else {
        println!("None");
    }

    println!("\n");
}

///////////////////////// P2 /////////////////////////

fn check_addition(a: u32, b: u32) -> u32 {
    if a + b >= u32::MAX {
        panic!(
            "Cele doua numere adunate {} si {} sunt prea mari pentru u32 ",
            a, b
        );
    } else {
        return a + b;
    }
}

fn check_multiplication(a: u32, b: u32) -> u32 {
    if a + b >= u32::MAX {
        panic!(
            "Cele doua numere inmultite {} si {} sunt prea mari pentru u32 ",
            a, b
        );
    } else {
        return a * b;
    }
}

fn main_ex2() {
    println!(
        "Adunarea lui 1024 cu 1024 este {}",
        check_addition(1024, 1024)
    );
    println!(
        "Inmultirea lui 1024 cu 1024 este {}",
        check_multiplication(124, 100024)
    );

    println!("\n");
}

///////////////////////// P3 /////////////////////////

fn check_addition_result(a: u32, b: u32) -> Result<u32, &'static str> {
    if a + b >= u32::MAX {
        return Err("Cele doua numere adunate sunt prea mari pentru u32 ");
    } else {
        return Ok(a + b);
    }
}

fn check_multiplication_result(a: u32, b: u32) -> Result<u32, &'static str> {
    if a + b >= u32::MAX {
        return Err("Cele doua numere inmultite sunt prea mari pentru u32 ");
    } else {
        return Ok(a * b);
    }
}

fn main_ex3() {

    let rez_add = check_addition_result(1024, 1024);
    if rez_add.is_ok() {
        println!("{}+{}={}", 1024, 1024, rez_add.unwrap())
    } else {
        println!("{}", rez_add.unwrap_err());
    }

    let rez_mull = check_multiplication_result(1024, 1024);
    if rez_mull.is_ok() {
        println!("{}+{}={}", 1024, 1024, rez_mull.unwrap())
    } else {
        println!("{}", rez_mull.unwrap_err());
    }

    println!("\n");
}

///////////////////////// P4 /////////////////////////

#[derive(Debug)]
enum ErrorType {
    NotAscii,
    NotDigit,
    NotBase16,
    NotLetter,
    NotPrint,
}

fn to_uppercase(mut input: char) -> Result<char, ErrorType> {
    if input.is_ascii_alphabetic() {
        input.make_ascii_uppercase();
        return Ok(input);
    } else {
        return Err(ErrorType::NotLetter);
    }
}

fn to_lowercase(mut input: char) -> Result<char, ErrorType> {
    if input.is_ascii_alphabetic() {
        input.make_ascii_lowercase();
        return Ok(input);
    } else {
        return Err(ErrorType::NotLetter);
    }
}

fn print_char(input: char) -> Result<char, ErrorType> {
    if input.is_ascii_graphic() || input.is_ascii_whitespace() {
        return Ok(input);
    } else {
        return Err(ErrorType::NotPrint);
    }
}

fn char_to_number(input: char) -> Result<i32, ErrorType> {
    if input.is_ascii() && input.is_ascii_digit() {
        return Ok((input as u8 - '0' as u8) as i32);
    } else if !input.is_ascii() {
        return Err(ErrorType::NotAscii);
    } else if !input.is_ascii_digit() {
        return Err(ErrorType::NotDigit);
    }
    return Err(ErrorType::NotAscii);
}

fn char_to_number_hex(input: char) -> Result<u32, ErrorType> {
    if input.is_ascii() && input.is_ascii_hexdigit() {
        return Ok(input.to_digit(16).unwrap());
    } else if !input.is_ascii_hexdigit() {
        return Err(ErrorType::NotBase16);
    } else {
        return Err(ErrorType::NotAscii);
    }
}

fn print_error(e: ErrorType) -> String {
    match e {
        ErrorType::NotAscii => String::from("Nu este ascii"),
        ErrorType::NotDigit => String::from("Nu este cifra"),
        ErrorType::NotBase16 => String::from("Nu este in baza 16"),
        ErrorType::NotLetter => String::from("Nu este litera"),
        ErrorType::NotPrint => String::from("Nu este printabila"),
    }
}

fn main_ex4() {
    let rez_1 = to_uppercase('b');
    if rez_1.is_ok() {
        println!(" to_uppercase('c')={}", rez_1.unwrap());
    } else {
        println!(" to_uppercase('c')={}", print_error(rez_1.unwrap_err()));
    }

    let rez_1 = to_uppercase('9');
    if rez_1.is_ok() {
        println!(" to_uppercase('')={}", rez_1.unwrap());
    } else {
        println!(" to_uppercase('')={}", print_error(rez_1.unwrap_err()));
    }

    let rez_1 = to_lowercase('U');
    if rez_1.is_ok() {
        println!(" to_lowercase('')={}", rez_1.unwrap());
    } else {
        println!(" to_lowercase('')={:?}", rez_1.unwrap_err());
    }

    let rez_2 = to_lowercase('?');
    if rez_2.is_ok() {
        println!(" to_lowercase('')={}", rez_2.unwrap());
    } else {
        println!(" to_lowercase('')={}", print_error(rez_2.unwrap_err()));
    }

    let rez_3 = print_char('☺');
    if rez_3.is_ok() {
        println!(" print_char={}", rez_3.unwrap());
    } else {
        println!(" print_char={}", print_error(rez_3.unwrap_err()))
    }

    let rez_3 = print_char('(');
    if rez_3.is_ok() {
        println!(" print_char={}", rez_3.unwrap());
    } else {
        println!(" print_char={}", print_error(rez_3.unwrap_err()))
    }

    let rez_4 = char_to_number('2');
    if rez_4.is_ok() {
        println!(" char_to_number={}", rez_4.unwrap());
    } else {
        println!(" char_to_number={}", print_error(rez_4.unwrap_err()))
    }

    let rez_4 = char_to_number('a');
    if rez_4.is_ok() {
        println!(" char_to_number={}", rez_4.unwrap());
    } else {
        println!(" char_to_number={}", print_error(rez_4.unwrap_err()))
    }

    let rez_5 = char_to_number_hex('A');
    if rez_5.is_ok() {
        println!(" char_to_number_hex={}", rez_5.unwrap());
    } else {
        println!(" char_to_number=_hex{}", print_error(rez_5.unwrap_err()))
    }

    let rez_5 = char_to_number_hex('z');
    if rez_5.is_ok() {
        println!(" char_to_number_hex={}", rez_5.unwrap());
    } else {
        println!(" char_to_number=_hex{}", print_error(rez_5.unwrap_err()))
    }

    println!("\n");
}

///////////////////////// P5 /////////////////////////

fn check_nume(nume: String) -> Option<String> {
    for c in nume.chars() {
        if !c.is_ascii_alphabetic() {
            return None;
        }
    }
    return Some(nume);
}

fn main_ex5() {
    let rez = check_nume(String::from("Maria"));
    if rez.is_some() {
        println!("Numele {} este valid", rez.unwrap());
    } else {
        println!("Numele nu contine caractere valide");
    }

    let rez2 = check_nume(String::from("M@ria"));
    if rez2.is_some() {
        println!("Numele {} este valid", rez2.unwrap());
    } else {
        println!("Numele nu contine caractere valide");
    }
}

///////////////////////// MAIN /////////////////////////

fn main() {
    main_ex1_1();
    main_ex1_2();
    main_ex2();
    main_ex3();
    main_ex4();
    main_ex5();
}
