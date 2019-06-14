fn main() {
    let fizz = "fizz".to_string();
    let buzz = "buzz".to_string();

    for number in 0..100 {
        let mut output = "".to_string();
        if number % 3 == 0 {
            output.push_str(&fizz)
        }
        if number % 5 == 0 {
            output.push_str(&buzz);
        }
        if output.len() > 0 {
            println!("{}", &output);
        } else {
            println!("{}", number);
        }
    }

}
