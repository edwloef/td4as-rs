use clap::Parser;
use std::{fs::read_to_string, path::PathBuf};

#[derive(Parser)]
struct Args {
    input: PathBuf,
}

fn main() {
    read_to_string(Args::parse().input)
        .unwrap()
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .enumerate()
        .for_each(|(i, line)| {
            if i % 4 == 0 {
                print!("\n[{i:x}]: ");
            }

            match line.split_once(' ').unwrap() {
                ("ADD", data) => match data.split_once(", ").unwrap() {
                    ("A", im) => print!("0000{:04b}", im_to_u4(im)),
                    ("B", im) => print!("0101{:04b}", im_to_u4(im)),
                    _ => panic!(),
                },
                ("MOV", data) => match data.split_once(", ").unwrap() {
                    ("A", "B") => print!("00010000"),
                    ("B", "A") => print!("01000000"),
                    ("A", im) => {
                        print!("0011{:04b}", im_to_u4(im));
                    }
                    ("B", im) => {
                        print!("0111{:04b}", im_to_u4(im));
                    }
                    _ => panic!(),
                },
                ("IN", im) => match im {
                    "A" => print!("00100000"),
                    "B" => print!("01100000"),
                    _ => panic!(),
                },
                ("OUT", im) => {
                    if im == "B" {
                        print!("10010000");
                    } else {
                        print!("1011{:04b}", im_to_u4(im));
                    }
                }
                ("JZ", im) => print!("1110{:04b}", im_to_u4(im)),
                ("JMP", im) => print!("1111{:04b}", im_to_u4(im)),
                _ => panic!(),
            }

            print!(" ");
        });
}

fn im_to_u4(im: &str) -> u8 {
    let im = im.strip_prefix("0x").unwrap();
    let im = u8::from_str_radix(im, 16).unwrap();
    assert!(im < 16);
    im
}
