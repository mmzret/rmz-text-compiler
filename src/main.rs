mod constants;
mod test;

use std::{
    fs::{read_to_string, File},
    io::Write,
    path::PathBuf,
};

use clap::Parser;
use constants::{
    ANSWER, BOTTOM, CHARMAP, FACTORY, INSERT, LF, MUGSHOT, MUGSHOTS, NEXT, OCCASION, RED, RETURN,
    TOP, WHITE,
};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// File path for zero text file
    #[clap(short, long, value_parser)]
    file: Option<PathBuf>,

    /// Output path as bin
    #[clap(short, long, value_parser)]
    output: Option<PathBuf>,

    /// Is chat
    #[clap(short, long)]
    chat: bool,

    text: Option<String>,

    #[clap(long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let mut buffer: Vec<u8> = Vec::new();
    let mut c = Compiler::new(args.chat);

    let input = match args.file {
        Some(file) => read_to_string(file).unwrap(),
        None => args.text.unwrap(),
    };

    c.compile(&input, &mut buffer);
    if args.verbose {
        eprintln!("size is {}", buffer.len());
    }

    match args.output {
        Some(output) => {
            let mut file = File::create(output).unwrap();
            file.write_all(&buffer).unwrap();
            file.flush().unwrap();
        }
        _ => {
            println!("{}", to_string(&buffer));
        }
    }
}

struct Compiler {
    answer: u8,
    _indent: i32,
    chat: bool,
    insert: bool,
}

impl Compiler {
    fn new(chat: bool) -> Compiler {
        return Compiler {
            answer: 0,
            _indent: 2,
            chat: chat,
            insert: false,
        };
    }

    fn reset(&mut self) {
        self.answer = 0;
        self._indent = 2;
    }

    fn indent(&self) -> i32 {
        if !self.chat {
            return 0;
        }
        return self._indent;
    }

    fn compile(&mut self, input: &str, buffer: &mut Vec<u8>) {
        self.reset();

        let mut specials = "".to_string();
        let mut skip = 0;

        let size = input.chars().count();
        for i in 0..(size) {
            let c = input.chars().nth(i).unwrap();
            match c {
                '<' => skip = 0,
                '}' => {
                    self.insert = false;
                    continue;
                }
                _ => {}
            }

            if self.insert {
                continue;
            }

            if skip > 0 {
                skip -= 1;
                continue;
            }

            (|| {
                match c {
                    '<' => {
                        specials = "<".to_string();
                    }

                    '>' => {
                        self.process_special(&specials[1..], buffer);
                        specials = "".to_string();
                        if check(input, i + 1, '\n') && self.chat {
                            skip = self.indent() + 1;
                        }
                    }

                    '{' => {
                        self.insert = true;
                        buffer.push(INSERT);
                        return;
                    }

                    '\n' => {
                        let is_next = check(input, i - 1, '▼');
                        if !is_next {
                            buffer.push(LF);
                        }

                        skip = self.indent();
                        if self.chat && is_next {
                            skip += 1;
                        }

                        // is End of character comment?
                        if check(input, i + 1, '<') || check(input, i + 1, '#') {
                            skip = 0;
                        }
                    }

                    '#' => {
                        buffer.remove(buffer.len() - 1); // remove LF
                        buffer.push(0xFF);
                        skip = 1; // skip \n
                    }

                    _ => {
                        if specials.len() > 0 {
                            specials.push_str(&c.to_string());
                            return;
                        }

                        // char is '..'
                        if c == '.' && check(input, i + 1, '.') {
                            skip = 1;
                            buffer.push(0xe4);
                            return;
                        }

                        // char is '工場'
                        if c == '工' && check(input, i + 1, '場') {
                            skip = 1;
                            buffer.push(FACTORY[0]);
                            buffer.push(FACTORY[1]);
                            buffer.push(FACTORY[2]);
                            return;
                        }

                        // char is '場合'
                        if c == '場' && check(input, i + 1, '合') {
                            skip = 1;
                            buffer.push(OCCASION[0]);
                            buffer.push(OCCASION[1]);
                            buffer.push(OCCASION[2]);
                            return;
                        }

                        // normal charcode
                        let charcode = CHARMAP.get(&c.to_string());
                        match charcode {
                            Some(charcode) => {
                                if charcode.clone() > 0xFF {
                                    buffer.push((charcode >> 8) as u8);
                                }
                                buffer.push(charcode.clone() as u8);
                            }
                            None => {
                                eprintln!("{}: -1,", c);
                                return;
                            }
                        }
                    }
                }
            })();
        }

        buffer.push(0xff);
    }

    fn process_special(&mut self, input: &str, buffer: &mut Vec<u8>) {
        // remove all space chars before the special char('<')
        loop {
            if buffer.len() == 0 {
                break;
            }
            let idx = buffer.len() - 1;
            if buffer[idx] != 0x00 {
                break;
            }
            buffer.remove(idx);
        }

        match input {
            "OPTION" => {
                self.answer = 0;
                buffer.push(0xf6);
                buffer.push(0x02);
                return;
            }

            "RED" => {
                buffer.push(RED);
                return;
            }
            "/RED" => {
                buffer.push(WHITE);
                return;
            }

            "ANSWER" => {
                let old = self.answer;
                self.answer += 1;

                match old {
                    // first answer
                    0 => {
                        self._indent += 4;
                        buffer.remove(buffer.len() - 1); // remove LF
                        buffer.push(NEXT);
                        return;
                    }

                    _ => {
                        buffer.remove(buffer.len() - 1); // remove LF
                        buffer.push(RETURN);
                    }
                }

                buffer.push(ANSWER);
                buffer.push(old);
                return;
            }

            _ => {
                // mugshot
                let mut name = input.to_string();

                let mut prefix = "";
                let s: Vec<&str> = input.split(":").collect();
                if s.len() == 2 {
                    prefix = s[0];
                    name = s[1].to_string();
                }

                // is right mugshot?
                let mut val = 0 as u8;
                if prefix.contains("r") {
                    val = 1;
                }

                // is top mugshot?
                let mut top = false;
                if prefix.contains("t") {
                    top = true;
                }

                // is bottom mugshot?
                let mut bottom = false;
                if prefix.contains("b") {
                    bottom = true;
                }

                let mugshot = MUGSHOTS.get(&name);
                match mugshot {
                    Some(mugshot) => {
                        val |= (mugshot * 2) as u8;
                    }
                    None => {}
                }

                buffer.push(MUGSHOT);
                buffer.push(val);
                if top {
                    buffer.push(TOP);
                }
                if bottom {
                    buffer.push(BOTTOM);
                }
                return;
            }
        }
    }
}

fn check(input: &str, ofs: usize, c: char) -> bool {
    let val = input.chars().nth(ofs);
    match val {
        Some(val) => {
            return val == c;
        }
        None => {
            return false;
        }
    }
}

pub fn to_hex(val: u8) -> String {
    return format!("{:#04X}", val).to_string();
}

pub fn to_string(val: &Vec<u8>) -> String {
    let mut result = "".to_string();
    for n in val {
        result.push_str(&to_hex(n.clone()));
        if *n != 0xff {
            result.push_str(", ");
        }
    }
    return result;
}
