
fn collatz_step(n: u64) -> u64 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

// calculate steps
fn collatz_length(mut n: u64) -> u64 {
    let mut steps = 0;
    while n != 1 {
        n = collatz_step(n);
        steps += 1;
    }
    steps
}

// calculate and saves complete sequence
fn collatz_sequence(mut n: u64) -> Vec<u64> {
    let mut seq = Vec::new();
    seq.push(n);
    while n != 1 {
        n = collatz_step(n);
        seq.push(n);
    }
    seq
}

fn main() {
    println!("Enter a starting number:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let start: u64 = input.trim().parse().expect("Please enter a valid number.");

    // Länge berechnen (effizient)
    let steps = collatz_length(start);
    println!("Schritte bis 1: {}", steps);

    // Ganze Sequenz speichern und ausgeben (optional)
    if start < 100_000 { // z.B. nur für kleinere Startzahlen
        let seq = collatz_sequence(start);
        println!("Sequenz: {:?}", seq);
    }
}