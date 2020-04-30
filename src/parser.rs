//! For parsing command line arguments.

use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;
use std::str::FromStr;

/// Checks if help is requested, meaning argument list is empty, contains `-h`,
/// or contains `--help`.
pub fn parse_help(args: &[String]) -> bool {
    args.len() <= 1 || args.iter().any(|x| (x == "-h" || x == "--help"))
}

// Converts arguments belonging to `opt` to decimals. Returns an error message
// on failure.
fn parse_group(args: &[String], opt: &str) -> Result<Vec<Decimal>, String> {
    let mut out = Vec::<Decimal>::new();

    // Find `opt`.
    let mut i = 0;
    loop {
        if i >= args.len() {
            return Ok(out);
        }
        if args[i] == opt {
            break;
        }
        i += 1;
    }

    // Parse subsequent arguments into decimals until the next option or the end
    // of the argument list.
    i += 1;
    while i < args.len()
        && (args[i].len() < 2
            || !args[i].starts_with('-')
            || args[i].chars().nth(1).unwrap().is_digit(10))
    {
        let arg = &args[i].replace(",", "");
        match Decimal::from_str(arg) {
            Ok(dec) => {
                if dec.abs() > Decimal::new(10_i64.pow(15), 0) {
                    return Err(format!(
                        "magnitude of '{}' exceeds 1,000,000,000,000,000",
                        arg
                    ));
                }
                out.push(dec.normalize());
            }
            Err(_) => {
                return Err(format!("unable to parse '{}' to a number", arg));
            }
        }
        i += 1;
    }

    Ok(out)
}

/// Parses arguments for the `-a`, `-c`, and `-d` options respectively. Returns
/// an error message on failure.
pub fn parse_groups(args: &[String]) -> Result<(Vec<Decimal>, Vec<Decimal>, Vec<Decimal>), String> {
    // If no args given for `-a`, read environment variable INVB_ALLOC.
    let mut alloc = parse_group(args, "-a")?;
    if alloc.is_empty() {
        if let Some(s) = std::option_env!("INVB_ALLOC") {
            let alloc_args = ("-a ".to_owned() + s)
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<String>>();
            alloc = parse_group(&alloc_args, "-a")?;
        }
    }
    let alloc = alloc;

    // If not args given for `-d`, use 0.
    let delta = parse_group(args, "-d")?;
    let delta = if delta.is_empty() {
        vec![Decimal::zero()]
    } else {
        delta
    };

    let current = parse_group(args, "-c")?;

    // Validations.
    if alloc.is_empty() {
        return Err("no values provided for <alloc>".to_owned());
    }
    if current.is_empty() {
        return Err("no values provided for <current>".to_owned());
    }
    if delta.is_empty() {
        return Err("too many values provided for <current>".to_owned());
    }
    if alloc.len() != current.len() {
        return Err("number of values provided for <alloc> and <current> differ".to_owned());
    }
    if alloc.iter().fold(Decimal::zero(), |acc, x| acc + x) != Decimal::new(100, 0) {
        return Err("values for <alloc> do not sum to 100".to_owned());
    }

    Ok((alloc, current, delta))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn split_args(args: &str) -> Vec<String> {
        args.split_whitespace().map(String::from).collect()
    }

    #[rstest(
        args,
        expected,
        case("prog -a 2 -d 100 -h", true),
        case("prog -a --help 2 -d 100", true),
        case("prog", true),
        case("prog -a 2 -d 100", false),
        case("prog -a", false)
    )]
    fn parse_help(args: &str, expected: bool) {
        let args = split_args(args);
        assert_eq!(super::parse_help(&args), expected);
    }

    #[rstest(args, opt, expected,
        case("-a 50 50 -c 9 101 -d -2,0.1", "-a", vec![Decimal::new(50, 0), Decimal::new(50, 0)]),
        case("-a 50 50 -c 9 101 -d -2,0.1", "-c", vec![Decimal::new(9, 0), Decimal::new(101, 0)]),
        case("-a 50 50 -c 9 101 -d -2,0.1", "-d", vec![Decimal::new(-201, 1)]),
        case("-a 50 50 -c 9 101 -d -2,0.1", "-e", vec![]),
    )]
    fn parse_group_success(args: &str, opt: &str, expected: Vec<Decimal>) {
        let args = split_args(args);
        let result = super::parse_group(&args, opt);
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest(
        args,
        opt,
        case("-a 5x0 50 -c 9 101 -d -2,0.1", "-a"),
        case("-a 5x0 50 -c 1,000,000,000,000,000.01 101 -d -2,0.1", "-c")
    )]
    fn parse_group_fail(args: &str, opt: &str) {
        let args = split_args(args);
        let result = super::parse_group(&args, opt);
        assert!(result.is_err());
    }
}
