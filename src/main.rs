use std::env;

fn main() {
    let args :Vec<String> = env::args().collect();
    match args.len() {
        4 => match (args[1].parse::<f64>(), args[2].parse::<f64>(), args[3].parse::<f64>()) {
            (Ok(a), Ok(b), Ok(c)) => quadratic_eq(a, b, c),
            _ => println!("One or more arguments are not valid."),
        },
        _ => {
            println!("Enter three decimal numbers as arguments. Eg.:");
            println!("$ quadratic_formula 1 3 -4");
            quadratic_eq(1.0, 3.0, -4.0);
        },
    };
}

fn quadratic_eq(a: f64, b: f64, c: f64) {
    if a == 0.0 {
        println!("'a' is equal to zero, this is not a quadratic function.");
    } else {
        let delta = b.powi(2) -4.0*a*c;
        if delta < 0.0 {
            println!("Delta is smaller than 0, the root doesn't exist.");
        } else if delta == 0.0 {
            let x = (-b+ delta.sqrt())/(2.0*a);
            println!("Delta equals 0, the root is {}", x);
        } else {
            let x1 = (-b+ delta.sqrt())/(2.0*a);
            let x2 = (-b- delta.sqrt())/(2.0*a);
            println!("Delta is bigger than 0, the roots are {} and {}", x1, x2);
        }
    }
}

