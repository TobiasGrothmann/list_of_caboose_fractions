use primal::Sieve;
use rayon::prelude::*;
use simple_tqdm::ParTqdm;

fn caboose_fraction(c: u64, sieve: &Sieve) -> f64 {
    let mut count = 0_u64;
    for n in 1..(c - 1) {
        let number: u64 = n.pow(2) - n + c;
        let prime = sieve.is_prime(number as usize);
        if prime {
            count += 1;
        }
    }
    count as f64 / (c - 2) as f64
}

fn main() {
    let target: u64 = 1_000_000;

    println!("generating sieve");
    let sieve = Sieve::new(target.pow(2) as usize);

    println!("checking caboose numbers");
    let caboose_fractions: Vec<(usize, f64)> = (3..target as usize)
        .into_par_iter()
        .step_by(2)
        .tqdm()
        .map(|c| (c, caboose_fraction(c as u64, &sieve)))
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
