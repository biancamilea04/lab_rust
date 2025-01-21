use std::{fs, io};

///////////////////////// P1 ///////////////////////

fn p1() -> Result<String, io::Error> {
    let continut_fisier = fs::read_to_string("p1.txt")?;

    let mut longest_line_bytes: String = String::from("");
    let mut longest_line_characters: String = String::from("");

    for rand in continut_fisier.lines() {
        if rand.len() > longest_line_bytes.len() {
            longest_line_bytes = rand.to_string().clone();
        }
        if rand.chars().count() > longest_line_characters.chars().count() {
            longest_line_characters = rand.to_string().clone();
        }
    }

    let mut rezultat = longest_line_bytes;
    rezultat.push('\n');
    rezultat.push_str(&longest_line_characters);

    Ok(rezultat)
}

fn main_p1() {
    let rez = p1();
    if rez.is_ok() {
        println!("Problema 1 rezultat: {}", rez.unwrap());
    } else {
        println!("Problema 1 Eroare:\n{}", rez.unwrap_err());
    }
}

///////////////////////// P2 ///////////////////////

fn p2(input: String) -> Result<String, String> {
    let rot13_prima_jumatate = String::from("ABCDEFGHIJKLMabcdefghijklm");
    let mut rez = String::from("");

    for c in input.chars() {
        if c.is_ascii() {
            if c.is_alphabetic() {
                if rot13_prima_jumatate.find(c).is_some() {
                    rez.push((c as u8 + 13 as u8) as char);
                } else {
                    rez.push((c as u8 - 13 as u8) as char);
                }
            } else {
                rez.push(c);
            }
        } else {
            return Err(String::from(
                "Sirul dat contine caractere care nu sunt ASCII",
            ));
        }
    }

    Ok(rez)
}

fn main_p2() {
    let rezultat1 = p2(String::from("HELLO"));
    if rezultat1.is_ok() {
        println!("Problema 2 rezultat: {}", rezultat1.unwrap());
    } else {
        println!("Problema 2 eroare: {}", rezultat1.unwrap_err())
    }

    let rezultat2 = p2(String::from("hello"));
    if rezultat2.is_ok() {
        println!("Problema 2 rezultat: {}", rezultat2.unwrap());
    } else {
        println!("Problema 2 eroare: {}", rezultat2.unwrap_err())
    }

    let rezultat3 = p2(String::from("hello!"));
    if rezultat3.is_ok() {
        println!("Problema 2 rezultat: {}", rezultat3.unwrap());
    } else {
        println!("Problema 2 eroare: {}", rezultat3.unwrap_err())
    }

    let rezultat4 = p2(String::from("hello!👀"));
    if rezultat4.is_ok() {
        println!("Problema 2 rezultat: {}", rezultat4.unwrap());
    } else {
        println!("Problema 2 eroare: {}", rezultat4.unwrap_err())
    }
}

///////////////////////// P3 ///////////////////////

fn comparam(cuvant: &str) -> &str {
    if cuvant == "pt" || cuvant == "ptr" {
        return "pentru";
    } else if cuvant == "dl" {
        return "domnul";
    } else if cuvant == String::from("dna") {
        return "doamna";
    } else {
        return cuvant;
    }
}
fn p3() -> Result<String, io::Error> {
    let continut_fisier: String = fs::read_to_string("p3.txt")?;

    let mut rezultat: String = String::from("");

    for cuvant in continut_fisier.split(" ") {
        rezultat.push_str(comparam(cuvant));
        rezultat.push(' ');
    }

    Ok(String::from(rezultat))
}

fn check() -> Result<(), io::Error> {
    let rez = p3();

    if rez.is_ok() {
        fs::write("p3.txt", rez.unwrap())?;
    } else {
        println!("Problema 3 rezultat: {:?}", rez.unwrap_err());
    }

    Ok(())
}

fn main_p3() {
    let rez_check = check();
    println!("Problema 3 rezultat: {:?}", rez_check);
}

///////////////////////// P4 ///////////////////////

fn p4() -> Result<(), io::Error> {
    let continut_fisier: String = fs::read_to_string("C:/Windows/System32/drivers/etc/hosts")?;

    for linie in continut_fisier.lines() {
        let mut id: String = String::from("");
        let mut host: String = String::from("");
        let copie_linie = linie;
        let mut count = 0;

        for cuvant in copie_linie.split(" ") {
            if linie.find("#").is_some() || count == 2 {
                break;
            }

            if id == "" {
                id = cuvant.to_string();
            } else {
                host = cuvant.to_string();
            }

            count += 1;
        }

        if id != "" && host != "" {
            println!("Functia P4:  {} => {}", host, id);
        }
    }

    return Ok(());
}

fn main_p4() {
    let rez = p4();
    println!("{:?}", rez);
}

///////////////////////// MAIN ///////////////////////

fn main() {
    main_p1();
    println!("\n");
    main_p2();
    println!("\n");
    main_p3();
    println!("\n");
    main_p4();
}
