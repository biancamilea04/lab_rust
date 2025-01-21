/////////////////////////////////// P1 ///////////////////////////////////

use rusqlite::Connection;
use rusqlite::Result;
use std::fs;
use std::io::{self, Write};

trait Commands {
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: Vec<String>);
}

struct PingCommand {}

impl Commands for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }
    fn exec(&mut self, args: Vec<String>) {
        if args.is_empty() {
            println!("pong");
        } else {
            println!("Comanda ping nu are nevoie de argumente!");
        }
    }
}

struct CountCommand {}

impl Commands for CountCommand {
    fn get_name(&self) -> &str {
        "count"
    }
    fn exec(&mut self, args: Vec<String>) {
        println!("count {} args", args.len());
    }
}

struct TimesCommand {
    count: i32,
}
impl Commands for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }
    fn exec(&mut self, args: Vec<String>) {
        if args.is_empty() {
            println!("Times a fost apelata de {} ori", self.count);
            self.count += 1;
        } else {
            println!("Comanda times nu are nevoie de argumente");
        }
    }
}
struct Terminal {
    comms: Vec<Box<dyn Commands>>,
}

impl Terminal {
    fn new() -> Self {
        Terminal { comms: Vec::new() }
    }
    fn register(&mut self, command: Box<dyn Commands>) {
        self.comms.push(command);
    }
    fn run(&mut self) -> Result<String, io::Error> {
        let continut_fisier: String = fs::read_to_string("src/p1.txt")?;

        for line in continut_fisier.lines() {
            let mut sw = 0;
            if !line.is_empty() {
                let comenzi_str: Vec<&str> = line.split_whitespace().collect();

                let mut comenzi: Vec<String>;
                comenzi = Vec::new();
                for c in comenzi_str {
                    comenzi.push(String::from(c));
                }

                for comanda in self.comms.iter_mut() {
                    if comanda.get_name() == comenzi[0] {
                        comanda.exec(comenzi[1..].to_vec());
                        sw = 1;
                    }
                    if comanda.get_name() == comenzi[0].to_lowercase()
                        && comanda.get_name() != comenzi[0]
                    {
                        println!(
                            "Comanda {} este scrisa gresit, incearca {}!",
                            comenzi[0],
                            comanda.get_name()
                        );
                        sw = 1;
                    }
                }

                if comenzi[0] == "stop" {
                    println!("stop!");
                    break;
                }

                if sw == 0 {
                    println!("Comanda {} nu exista", comenzi[0]);
                }
            } else {
                println!("Linia nu a primti comenzi!");
            }
        }

        Ok(String::from(""))
    }
}

fn main1() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));

    let rez = terminal.run();
    if rez.is_err() {
        println!("{}", rez.unwrap_err());
    }
}
/////////////////////////////////// P2 ///////////////////////////////////

#[derive(Default)]
struct BazaDeDate {
    name: String,
    url: String,
}

impl BazaDeDate {
    fn exec_add(&mut self, args: Vec<String>) -> Result<String, rusqlite::Error> {
        self.name = args[0].clone();
        self.url = args[1].clone();

        let conn = Connection::open("lab6p2.db")?;
        let create = r"
create table if not exists lab6p2 (
    name text    not null,
    url  txt not null
);
";
        conn.execute(create, ())?;

        self.name = args[0].clone();
        self.url = args[1].clone();
        conn.execute(
            "insert into lab6p2 (name, url) values (?1, ?2);",
            (args[0].clone(), args[1].clone()),
        )?;
        println!("Add> Am facut add");
        Ok(String::from(""))
    }
    fn exec_search(&mut self, args: Vec<String>) -> Result<String, rusqlite::Error> {
        println!("Add> Am facut search");

        let conn = Connection::open("lab6p2.db")?;
        let mut stmt = conn.prepare("select * from lab6p2")?;

        let iter = stmt.query_map([], |row| {
            Ok(BazaDeDate {
                name: row.get("name")?,
                url: row.get("url")?,
            })
        })?;

        let mut flag = 0;
        for i in iter {
            let i = i?;
            if args[0].clone().contains(i.name.chars().nth(0).unwrap()) {
                println!("name={}, url={}", i.name, i.url);
                flag = 1;
            }
        }

        if flag == 0 {
            println!("Numele {} nu se gaseste in baza de date!", args[0]);
        }

        Ok(String::from(""))
    }
}

fn p2() {
    let mut input = String::new();

    println!("Pentru a iesi din program introdu comanda bk exit");

    while input.trim() != "bk exit" {
        print!("Introdu comanda>");
        io::stdout().flush().unwrap();

        input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let comenzi_linie_str: Vec<&str> = input.split_whitespace().collect();

        let mut comenzi_linie: Vec<String>;
        comenzi_linie = Vec::new();
        for c in comenzi_linie_str {
            comenzi_linie.push(String::from(c));
        }

        if !comenzi_linie.is_empty() {
            if comenzi_linie[0] == "bk" {
                println!("comanda intodusa:{}", input.trim());
                if comenzi_linie[1] == "add" {
                    if comenzi_linie.len() == 4 {
                        let mut add: BazaDeDate = Default::default();

                        let add_var = add.exec_add(comenzi_linie[2..].to_vec());
                        if add_var.is_err() {
                            println!("{}", add_var.unwrap_err());
                        }
                    } else {
                        println!("Comanda add nu are argumente suficiente");
                    }
                } else if comenzi_linie[1] == "search" {
                    if comenzi_linie.len() == 3 {
                        let mut search: BazaDeDate = Default::default();

                        let search_var = search.exec_search(comenzi_linie[2..].to_vec());
                        if search_var.is_err() {
                            println!("{}", search_var.unwrap_err());
                        }
                    } else {
                        println!("Comanda search nu are suficiente argumente");
                    }
                }
            } else {
                println!("Comanda introdusa gresit. Comanda trebuie sa inceapa cu bk");
            }
        } else {
            println!("Nu s-au introdus comenzi");
        }
    }
}

///////////////////////////////////  M A I N  ///////////////////////////////////

fn main() {
    main1();
    println!("\n");
    p2();
}
