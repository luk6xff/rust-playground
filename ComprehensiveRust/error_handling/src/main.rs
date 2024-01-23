use std::num::ParseIntError;


fn multiply1(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => {
            match second_number_str.parse::<i32>() {
                Ok(second_number) => {
                    Ok(first_number * second_number)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

fn multiply2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

fn multiply3(first_number_str: &str, second_number_str: &str) ->Result<i32, ParseIntError> {
    Ok(first_number_str.parse::<i32>()? * second_number_str.parse::<i32>()?)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("Result: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let first_num = "20z";
    let second_num = "40";
    print(multiply1(first_num, second_num));
    print(multiply2(first_num, second_num));
    print(multiply3(first_num, second_num));
}
