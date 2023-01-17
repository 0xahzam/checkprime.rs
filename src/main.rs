use std::io;

fn prime(x: f64) {
    let sqr = x.sqrt().floor();

    let num: i32 = sqr as i32 + 1;

    let mut flag = false;

    for i in 2..num {
        let n: i32 = num % i;

        if n == 0 {
            flag = true
        }
    }

    if flag == false {
        println!("{} is a prime number", x);
    } else {
        println!("{} is not a prime number", x)
    }
}

fn main() {

    println!("Enter a number: ");

    let mut input_text = String::new();
    
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();

    match trimmed.parse::<f64>() {
        Ok(i) => prime(i),
        Err(..) => println!("Err"),
    };
}
