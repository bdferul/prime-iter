pub const GEN_START: u64 = 2365;
pub const PRINT_START: u64 = 2000;
pub const PRINT_LEN: u64 = i64::MAX as u64;
pub const GEN_RANGE: u64 = 200;

use zetik_prime::PrimeIter;

fn main() {
    let print_fmt = |id, next: u64| println!("{id}> +{} {next}", next.saturating_sub(GEN_START));

    let mut primes = PrimeIter::default();
    primes.last_where(|x| x < PRINT_START || x <= GEN_START);
    for i in 1.. {
        let next = primes.next().unwrap();

        if next - GEN_START > GEN_RANGE {
            break;
        }

        print_fmt(i, next);
    }
}

fn _main() {
    let n = 1_000_000;
    let primes = PrimeIter::default().take(n);

    let mut counter = [0; 10];

    let start = std::time::Instant::now();
    for p in primes {
        let ndx = (p % 10) as usize;
        counter[ndx] += 1;
    }
    println!("Generated in {} milliseconds", start.elapsed().as_millis());

    for (i, c) in counter.into_iter().enumerate() {
        let ratio = c as f64 / n as f64;
        println!("{i}: {ratio:.2} {c}");
    }
}
