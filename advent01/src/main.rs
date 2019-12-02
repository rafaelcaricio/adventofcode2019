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
        acc += calculate_fuel(mass) as u64;
    }
    println!("Result: {}", acc);
    Ok(())
}

fn calculate_fuel(mass: f64) -> f64 {
    let required_fuel = (mass / 3f64).floor() - 2f64;
    if required_fuel > 0f64 {
        required_fuel + calculate_fuel(required_fuel)
    } else {
        0f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(calculate_fuel(14f64), 2f64);
    }

    #[test]
    fn test_example2() {
        assert_eq!(calculate_fuel(1969f64), 966f64);
    }

    #[test]
    fn test_example3() {
        assert_eq!(calculate_fuel(100756f64), 50346f64);
    }
}
