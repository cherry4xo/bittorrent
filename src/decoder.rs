use crate::errors::Errors;

use serde_json::{self, json};

fn string_to_i64(input: &str) -> Result<i64, Errors> {
    let res: Result<i64, _> = input.parse();
    match res {
        Ok(num) => Ok(num),
        Err(_) => {
            Err(Errors::InvalidArgument(String::from("incorrect string to convert")))
        }
    }
}

fn decode_bencoded_string(encoded_value: &str) -> Result<serde_json::Value, Errors> {
    if encoded_value.chars().all(|c| c.is_ascii_digit()) {
        Err(Errors::InvalidArgument(String::from("incorrect string to convert")))
    } else {
        match encoded_value.find(':') {
            Some(index) => {
                match string_to_i64(&encoded_value[..index]) {
                    Ok(number) => {
                        let extracted_string: &str = &encoded_value[(index + 1)..number.try_into().unwrap()];
                        Ok(serde_json::Value::String(String::from(extracted_string)))
                    },
                    Err(_) => Err(Errors::InvalidArgument(String::from("incorrect string to convert"))),
                }
            },
            None => Err(Errors::InvalidArgument(String::from("incorrect string to convert")))
        }
    }
}

fn decode_bencoded_integer(encoded_value: &str) -> Result<serde_json::Value, Errors> {
    if encoded_value.chars().next() != Some('i') || encoded_value.chars().rev().next() != Some('e') {
        Err(Errors::InvalidArgument(String::from("incorrect string to convert")))
    } else {
        match string_to_i64(&encoded_value[1..encoded_value.len() - 2]) {
            Ok(number) => {
                Ok(serde_json::Value::String(String::from(number.to_string())))
            },
            Err(_) => Err(Errors::InvalidArgument(String::from("incorrect string to convert"))),
        }
    }
}