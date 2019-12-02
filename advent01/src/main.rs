use std::io;

fn main() -> io::Result<()> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut acc = 0u64;
    while stdin.read_line(&mut buf).unwrap_or(0) > 0 {
        let trimmed_line = buf.trim();
        let mass = match String::from(trimmed_line).parse::<f64>() {
            Ok(mass) => mass,
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break
            }
        };
        buf.clear();
        let part = (mass / 3f64).floor() as u64;
        acc += part - 2;
    }
    println!("Result: {}", acc);
    Ok(())
}
