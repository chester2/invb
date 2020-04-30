use rust_decimal::Decimal;

mod parser;
mod table;

use table::{Column, Table};

const HELP: &str = concat!(
    "View transactions needed to rebalance an investment portfolio.\n",
    "Usage: invb [-a <alloc>...] -c <current>... [-d <delta>]\n",
    "\n",
    "Arguments:\n",
    "  -a <alloc>...    Ideal component allocations in percentages.\n",
    "  -c <current>...  Current market value of components.\n",
    "  -d <delta>       Net change of portfolio value after transactions.\n",
    "\n",
    "Notes:\n",
    "  * Numbers may be provided with comma thousands separators.\n",
    "  * <alloc> and <current> must have an equal number of components.\n",
    "  * <alloc> and <current> components must be specified in the same order.\n",
    "  * <alloc> components must sum to 100.\n",
    "  * <alloc> defaults to what's stored in the environment variable 'INVB_ALLOC'.\n",
    "  * <delta> defaults to 0.\n",
);

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if parser::parse_help(&args) {
        print!("{}", HELP);
        return;
    }

    let (alloc, current, delta) = parser::parse_groups(&args).unwrap_or_else(|msg| {
        println!("error:\n  {}", msg);
        std::process::exit(1);
    });

    let total = (delta[0] + current.iter().fold(Decimal::new(0, 0), |acc, x| acc + x)).normalize();
    let future: Vec<Decimal> = alloc
        .iter()
        .map(|x| (x / Decimal::new(100, 0) * total).normalize())
        .collect();
    let change: Vec<Decimal> = future
        .iter()
        .zip(current.iter())
        .map(|(f, c)| (f - c).normalize())
        .collect();

    let table = Table::new(vec![
        Column::from_decimals("Original", &current),
        Column::from_decimals("Final", &future),
        Column::from_decimals("Change", &change),
    ]);
    print!("{}", table.draw());
}
