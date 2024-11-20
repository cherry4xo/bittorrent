use crate::errors::Errors;

use serde_json;

fn string_to_i64(input: &String) -> Result<i64, Errors> {
    let res: Result<i64, _> = input.parse();
    match res {
        Ok(num) => Ok(num),
        Err(_) => {
            Err(Errors::InvalidArgument(String::from("incorrect string to convert")))
        }
    }
}