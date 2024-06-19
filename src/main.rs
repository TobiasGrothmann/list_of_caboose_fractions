use std::collections::HashSet;

use primal::Primes;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use thousands::Separable;

fn caboose_fraction(c: u64, primes: &HashSet<u64>) -> f64 {
    let mut count = 0_u64;
    for n in 1..(c - 1) {
        let number: u64 = n.pow(2) - n + c;
        let prime = primes.contains(&number);
        if prime {
            count += 1;
        }
    }
    count as f64 / (c - 2) as f64
}

fn main() {
    let target: u64 = 1_000_000;
    println!(
        "generating primes up to {}^2 = {}",
        target.separate_with_commas(),
        target.pow(2).separate_with_commas()
    );
    let primes = Primes::all()
        .take_while(|&p| p as u64 <= target.pow(2))
        .map(|p| p as u64)
        .collect::<HashSet<u64>>();

    println!("checking caboose numbers");
    let caboose_fractions: Vec<(u64, f64)> = (3..target)
        .into_par_iter()
        .map(|c| (c, caboose_fraction(c, &primes)))
        .collect();

    println!("writing to caboose_fractions.csv");
    let mut wtr = csv::Writer::from_path("caboose_fractions.csv").unwrap();
    wtr.write_record(&["c", "fraction"]).unwrap();
    for (c, percentage) in &caboose_fractions {
        wtr.write_record(&[c.to_string(), percentage.to_string()])
            .unwrap();
    }

    for (c, fraction) in caboose_fractions {
        if fraction > 0.5 {
            println!("c = {} => {:.2}%", c, fraction * 100.0);
        }
    }
}
