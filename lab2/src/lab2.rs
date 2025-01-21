///P1///
fn add_chars_n(mut st: String, ch: char, nr: i32) -> String {
    let mut i = 0;
    while i < nr {
        st.push(ch);
        i += 1;
    }
    st
}

fn main_p1() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}

/////////////////////////////////////////////////////////

///P2///

fn add_chars_n2(st: &mut String, ch: char, nr: i32) {
    let mut i = 0;
    while i < nr {
        st.push(ch);
        i += 1;
    }
}

fn main_p2() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        add_chars_n2(&mut s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}

/////////////////////////////////////////////////////////

///P3///

fn add_space(st: &mut String, nr: i32) {
    let mut i: i32 = 0;
    while i < nr {
        st.push(' ');
        i += 1;
    }
}

fn add_str(st: &mut String, st1: &str) {
    st.push_str(st1);
}

fn nr_to_string(mut nr: i32) -> String {
    let mut st: String = String::from("");
    let mut p: i32 = 1;
    let mut numar_invers: i32 = 0;
    while nr != 0 {
        let cif: i32 = nr % 10;
        numar_invers = numar_invers * 10 + cif;
        p = p * 10;
        nr /= 10;
    }
    while numar_invers > 0 {
        let ch: char = ((numar_invers % 10) as u8 + '0' as u8) as char;
        st.push(ch);
        numar_invers /= 10;
    }
    return st;
}

fn add_integer(st: &mut String, mut nr: i32) {
    let mut new_st: String = String::new();

    while nr > 1000 {
        let cif = nr % 1000;
        let mut aux_st: String = String::new();
        aux_st.push('_');
        aux_st.push_str(&nr_to_string(cif));
        aux_st.push_str(&new_st);
        new_st = aux_st.clone();
        nr /= 1000;
    }

    if nr > 0 {
        let mut aux_st: String = String::new();
        let mut aux_nr = nr_to_string(nr);
        aux_nr.push_str(&new_st);
        aux_st.push_str(&aux_nr);
        st.push_str(&aux_st);
    }
}

fn add_float(st: &mut String, nr: f32) {
    let parte_intreaga: i32 = nr as i32; 
    let mut parte_fractionala: f32 = nr - parte_intreaga as f32;
    let mut rest: f32 = parte_fractionala - (parte_fractionala as i32) as f32; 

    let mut string_fractional: String = String::new();

    while (0.0 < rest) && (rest < 1.0) {
        parte_fractionala = rest * 10.0;
        let cif: char = ((parte_fractionala as i32) as u8 + '0' as u8) as char; 
        string_fractional.push(cif);
        rest = parte_fractionala - (parte_fractionala as i32) as f32;
    }

    while string_fractional.len() > 3 {
        string_fractional.pop();
    }

    add_integer(st, parte_intreaga);
    st.push('.');
    st.push_str(&string_fractional);

}

fn main_p3(){
    let mut st: &mut String = &mut String::new();
    add_space(&mut st, 43);
    add_str(&mut st, "I 💚\n");
    add_space(&mut st, 43);
    add_str(&mut st, "RUST.\n");
    println!("{}",st);
    let mut st: &mut String = &mut String::new();
    add_space(&mut st, 4);
    add_str(&mut st, "Most");
    add_space(&mut st, 15);
    add_str(&mut st, "crate");
    add_space(&mut st, 6);
    //print!("{}",st);
    add_integer(&mut st, 306437968);
    add_space(&mut st, 11);
    add_str(&mut st, "and");
    add_space(&mut st, 5);
    add_str(&mut st, "lastest");
    add_space(&mut st, 9);
    add_str(&mut st, "is\n");
  //  println!("{}",st);
  // let mut st: &mut String = &mut String::new();
    add_space(&mut st, 9);
    add_str(&mut st, "downloaded");
    add_space(&mut st, 10);
    add_str(&mut st, "has");
    add_space(&mut st, 14);
    add_str(&mut st, "downloads");
    add_space(&mut st, 5);
    add_str(&mut st, "the");
    add_space(&mut st, 9);
    add_str(&mut st, "version");
    add_space(&mut st, 4);
    add_float(&mut st, 2.038);
    add_str(&mut st, ".");
    add_space(&mut st, 3);
    print!("{}",st);
}

fn main() {
    main_p1();
    println!("");
    println!("");
    main_p2();
    println!("");
    println!("");
    main_p3();

    
}
