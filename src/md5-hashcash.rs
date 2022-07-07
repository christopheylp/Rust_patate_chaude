use std::ptr::hash;
use std::u64;
use md5;

fn solve(input: MD5HashCashInput) -> String {
    let mut compteur: u64 = 1;
    let mut is_seed_found = false;
    let mut hash_code: String = "".to_string();
    let complexity = input.complexity;
    let message = input.message;
    while is_seed_found != true {
        let seed_value = to_seed(compteur);
        hash_code = get_hash_code(seed_value, &message).to_uppercase();
        is_seed_found = checkSeed(hash_code.clone(), complexity);
        compteur += 1;
    }
    hash_code

}
fn checkSeed(hash: String, complexity: u32) -> bool {
    let mut compteur = 0;
    let bin  = hexToBinary(&*hash);
    for character in str::chars(&bin) {
        if character == '0' {
            compteur += 1;
        }
        else {
            break;
        }
    }
    return complexity <= compteur;
}

fn to_seed(number: u64) -> String {
    let complexity_bin = decimal_to_binary(number);
    let str = String::from(complexity_bin.to_string());
    String::from(binaryToHex(str, 8).to_string()).to_uppercase()
}


fn get_hash_code(seed: String, message: &String) -> String {
    let mut seed_value = seed.to_owned();
    let message_value = message.to_owned();
    seed_value.push_str(&message_value);
    let digest = md5::compute(seed_value);
    format!("{:?}", digest)
}


pub struct MD5HashCashInput {
    complexity: u32,
    message: String,
}


pub struct MD5HashCashOutput {
    seed: u64,
    hashcode: String,
}

fn hex_to_decimal(val: String) -> u64  {
    let z = u64::from_str_radix(&*val, 16).expect("Not a binary hex!");
    z
}



fn decimal_to_binary(mut decimal: u64) -> u64 {
    if decimal == 0 {
        decimal
    } else {
        let mut bits = String::new();

        while decimal > 0 {
            if decimal % 2 == 0 {
                bits.push_str("0");
            } else {
                bits.push_str("1");
            }

            decimal /= 2;
        }

        match bits.chars().rev().collect::<String>().parse() {
            Ok(num) => num,
            Err(_e) => panic!("Something went wrong"),
        }
    }
}

fn binaryToHex(val: String, len: usize) -> String {
    let n: u64 = u64::from_str_radix(&*val, 2).unwrap();
    format!("{:01$x}", n, len * 2)
}

fn hexToBinary(hex: &str) -> String {
    hex[0..].chars().map(to_binary).collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn main(){}

#[cfg(test)]
mod test{
    use crate::{solve, MD5HashCashInput};

    #[test]
    fn sample_test(){

        let input = MD5HashCashInput {
            complexity: 9,
            message: String::from("hello"),
        };

        assert_eq!(solve(input), "00441745D9BDF8E5D3C7872AC9DBB2C3");
    }

}
