#![allow(non_snake_case)]
use std::{env::args, process::exit};

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
fn decode_u4(c: &char) -> u8 {
    match c {
        'H' => 0b0000,
        'h' => 0b0001,
        'Ĥ' => 0b0010,
        'ĥ' => 0b0011,
        'Ħ' => 0b0100,
        'ħ' => 0b0101,
        'Ƕ' => 0b0110,
        'Ȟ' => 0b0111,
        'ȟ' => 0b1000,
        'ℋ' => 0b1001,
        'ℌ' => 0b1010,
        'ℍ' => 0b1011,
        'ℎ' => 0b1100,
        'ℏ' => 0b1101,
        'ḩ' => 0b1110,
        'Ⱨ' => 0b1111,
        _ => 0.into(),
    }
}

/// Encode a slice of bytes to H.
pub fn encode(input: &String) -> String {
    let s = input.as_bytes();
    let mut encoded = String::new();

    for byte in s.iter() {
        encoded.push(encode_u4(&(byte >> 4)));
        encoded.push(encode_u4(&(byte & 0x0f)));
    }

    encoded
}

/// Decode a slice of bytes to H.
pub fn decode(input: &String) -> String {
    let bytes: Vec<char> = input.char_indices().map(|(_, c)| c).collect();
    let mut decoded = String::new();

    for (idx, c) in bytes.iter().enumerate().step_by(2) {
        let mut b = decode_u4(c) << 4;
        if idx + 1 < bytes.len() {
            b |= decode_u4(&bytes[idx + 1]);
        }
        decoded.push(b.into());
    }

    decoded
}

/// Kill the program with some error message.
fn die(msg: &str) {
    println!("{}", msg);
    exit(1)
}

fn main() {
    let vargs: Vec<String> = args().collect();
    if vargs.len() < 3 {
        die("Usage: H [edED] .*")
    }

    let bytes = vargs[2..].join(" ");
    match vargs[1].to_uppercase().as_str() {
        "E" => print!("{}", encode(&bytes)),
        "D" => print!("{}", decode(&bytes)),
        &_ => die("Invalid mode."),
    }
    exit(0)
}
