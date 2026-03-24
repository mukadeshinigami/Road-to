/// Task 2: CLI command parser — `Option<Command>`.

enum Command {
    Exit,
    Help,
    Run { name: String },
    Status,
}

fn parse(input: &str) -> Option<Command> {
    let input = input.trim();
    if input.is_empty() {
        return None;
    }

    match input {
        "exit" => Some(Command::Exit),
        "help" => Some(Command::Help),
        "status" => Some(Command::Status),
        _ => {
            let mut parts = input.split_whitespace();
            match parts.next() {
                Some("run") => {
                    let name: String = parts.collect::<Vec<_>>().join(" ");
                    if name.is_empty() {
                        None
                    } else {
                        Some(Command::Run { name })
                    }
                }
                _ => None,
            }
        }
    }
}

fn main() {
    let samples = [
        "exit",
        "help",
        "run myapp",
        "run my multi word app",
        "status",
        "unknown",
        "  help  ",
    ];

    for line in samples {
        match parse(line) {
            Some(Command::Exit) => println!("{line:?} -> Exit"),
            Some(Command::Help) => println!("{line:?} -> Help"),
            Some(Command::Status) => println!("{line:?} -> Status"),
            Some(Command::Run { name }) => println!("{line:?} -> Run({name:?})"),
            None => println!("{line:?} -> None"),
        }
    }
}
