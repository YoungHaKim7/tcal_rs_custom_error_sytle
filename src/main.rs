use std::{error::Error, path::Path};

use tcal_rs_custom_error_sytle::calculator::engine::Engine;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    println!("Qalculate CLI - Interactive Calculator");
    println!("Type 'help' or 'exit' (or press Ctrl+C/Ctrl+D to quit)\n");

    let history_path = Path::new("history.txt");
    let mut history = load_history(history_path)?;
    let mut engine = Engine::new();

    while let Some(input) = readline_with_history("> ", &history)? {
        if input.is_empty() {
            continue;
        }

        if input == "exit" || input == "quit" {
            break;
        }

        if input == "help" {
            println!(
                r#"Commands:
- math: 2+3*4
- power: 2^10
- bitwise: 5 & 3, 1 << 4
- hex/bin/oct: 0xFF, 0b1010
- convert: 255 to hex bin oct
- unicode: "안녕" to unicode
- variables: x = 10
- res: reuse last result
- Arrow keys: navigate history
"#
            );
            continue;
        }

        match engine.full_eval(&input) {
            Ok(out) => println!("{}", out),
            Err(e) => println!("Error: {}", e),
        }

        history.push(input);
    }

    save_history(history_path, &history)?;
    Ok(())
}
