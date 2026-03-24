/// Task 3: Calculator with `Result`.

fn calculate(op: char, a: f64, b: f64) -> Result<f64, String> {
    match op {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => {
            if b == 0.0 {
                Err(String::from("division by zero"))
            } else {
                Ok(a / b)
            }
        }
        _ => Err(String::from("unknown operator")),
    }
}

fn main() {
    let cases = [
        ('+', 1.0, 2.0),
        ('-', 5.0, 3.0),
        ('*', 2.0, 4.0),
        ('/', 10.0, 2.0),
        ('/', 1.0, 0.0),
        ('%', 1.0, 1.0),
    ];

    for (op, a, b) in cases {
        match calculate(op, a, b) {
            Ok(result) => println!("{a} {op} {b} = {result}"),
            Err(e) => println!("{a} {op} {b} -> Err({e:?})"),
        }
    }

    if let Ok(result) = calculate('*', 3.0, 7.0) {
        println!("if let Ok: 3 * 7 = {result}");
    }
}
