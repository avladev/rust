use std::io::Error;
use std::fs;

fn main() -> Result<(), Error> {
    let mut errors = vec![];

    match fs::read_to_string("logs.txt") {
        Ok(data) => {
            println!("Logs 2 {} bytes", data.len());

            for line in data.split("\n") {
                if line.starts_with("ERROR") {
                    errors.push(line.to_string());
                }
            }
        }
        Err(error) => {
            println!("Error reading logs.txt Error: {}", error);
        }
    }

    match fs::write("errors.txt", errors.join("\n")) {
        Ok(_) => {
            println!("Successfully wrote errors to errors.txt");
        }
        Err(error) => {
            println!("Error writing errors.txt Errors: {}", error);
        }
    }

    // Expect impl
    let text = fs::read_to_string("logs.txt")
        .expect("Error reading logs.txt");

    fs::write(
        "errors.txt",
        text.split("\n")
            .filter(|line| line.starts_with("ERROR"))
            .collect::<Vec<&str>>().join("\n")
    ).expect("Error writing errors.txt");

    println!("{:#?}", errors);

    // Impl with try operator
    let text = fs::read_to_string("logs2.txt")?;

    fs::write(
        "errors.txt",
        text.split("\n")
            .filter(|line| line.starts_with("ERROR"))
            .collect::<Vec<&str>>().join("\n")
    )?;

    //
    Ok(())
}