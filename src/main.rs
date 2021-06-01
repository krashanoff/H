#![allow(non_snake_case)]
use std::{
    env::args,
    io::{stdin, Read},
    process::exit,
};

//
// H
//
// For the dead week enjoyer.
//
// WTFPL
//

/// Encodes a single four-bit value to `baseH`.
fn encode_u4(c: &u8) -> char {
    match c {
        0b0000 => 'H',   // Latin Capital Letter H
        0b0001 => 'h',   // Latin Small Letter H
        0b0010 => 'Ĥ',  // Latin Capital Letter H with circumflex
        0b0011 => 'ĥ',  // Latin Small Letter H with circumflex
        0b0100 => 'Ħ',  // Latin Capital Letter H with stroke
        0b0101 => 'ħ',  // Latin Small Letter H with stroke
        0b0110 => 'Ƕ',  // Latin Capital Letter Hwair
        0b0111 => 'Ȟ',  // Latin Capital Letter H with caron
        0b1000 => 'ȟ',  // Latin Small Letter H with caron
        0b1001 => 'ℋ', // Script Capital H
        0b1010 => 'ℌ', // Black-Letter Capital H
        0b1011 => 'ℍ', // Double-Struck Capital H
        0b1100 => 'ℎ', // Planck Constant
        0b1101 => 'ℏ', // Planck Constant with strike
        0b1110 => 'ḩ', // The letter h with a cedilla.
        0b1111 => 'Ⱨ', // The letter H with a descender.
        _ => 0.into(),
    }
}

/// Decodes a given `char` from `baseH` to a four-bit value.
fn decode_u4(c: &char) -> Result<u8, &'static str> {
    match c {
        'H' => Ok(0b0000),
        'h' => Ok(0b0001),
        'Ĥ' => Ok(0b0010),
        'ĥ' => Ok(0b0011),
        'Ħ' => Ok(0b0100),
        'ħ' => Ok(0b0101),
        'Ƕ' => Ok(0b0110),
        'Ȟ' => Ok(0b0111),
        'ȟ' => Ok(0b1000),
        'ℋ' => Ok(0b1001),
        'ℌ' => Ok(0b1010),
        'ℍ' => Ok(0b1011),
        'ℎ' => Ok(0b1100),
        'ℏ' => Ok(0b1101),
        'ḩ' => Ok(0b1110),
        'Ⱨ' => Ok(0b1111),
        _ => Err("input is not H-encoded"),
    }
}

/// Kill the program with some error message.
fn die<T: ToString>(msg: T) -> ! {
    eprintln!("{}", msg.to_string());
    exit(1)
}

fn main() {
    let vargs: Vec<String> = args().collect();
    if vargs.len() < 3 {
        die("Usage: H [edED] (-|.*)")
    }

    let buf = match vargs[2].as_str() {
        "-" => {
            let mut buffer = String::new();
            stdin()
                .read_to_string(&mut buffer)
                .unwrap_or_else(|e| die(format!("Failed to read from stdin: {}", e)));
            buffer
        }
        _ => vargs[2..].join(" "),
    };

    match vargs[1].to_uppercase().as_str() {
        "E" => {
            buf.bytes().for_each(|b| {
                print!("{}", encode_u4(&(b >> 4)));
                print!("{}", encode_u4(&(b & 0x0f)));
            });
        }
        "D" => {
            buf.chars()
                .collect::<Vec<char>>()
                .chunks(2)
                .for_each(|chunk| match chunk {
                    &[c1, c2] => {
                        let mut b = decode_u4(&c1).unwrap_or_else(|e| die(e)) << 4;
                        b |= decode_u4(&c2).unwrap_or_else(|e| die(e));
                        print!("{}", b as char);
                    }
                    &_ => die("Input is not valid H encoding."),
                });
        }
        &_ => die("Invalid mode."),
    }
    exit(0)
}
