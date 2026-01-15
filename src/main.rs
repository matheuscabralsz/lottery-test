use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::collections::BTreeSet;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

const DATA_URL: &str = "https://raw.githubusercontent.com/guilhermeasn/loteria.json/master/data/megasena.json";

type HistoricalData = std::collections::HashMap<String, Vec<String>>;

fn fetch_historical_data() -> Result<HistoricalData, Box<dyn std::error::Error>> {
    println!("Fetching historical Mega-Sena results...");
    let response = reqwest::blocking::get(DATA_URL)?;
    let data: HistoricalData = response.json()?;
    println!("Loaded {} historical draws.\n", data.len());
    Ok(data)
}

fn parse_draw(numbers: &[String]) -> Option<BTreeSet<u8>> {
    let set: BTreeSet<u8> = numbers.iter().filter_map(|n| n.parse().ok()).collect();
    if set.len() == 6 {
        Some(set)
    } else {
        None
    }
}

fn generate_combination(rng: &mut impl rand::Rng) -> BTreeSet<u8> {
    let mut numbers: Vec<u8> = (1..=60).collect();
    numbers.shuffle(rng);
    numbers.into_iter().take(6).collect()
}

fn find_match(target: &BTreeSet<u8>, running: &AtomicBool) -> Option<u64> {
    let mut rng = thread_rng();
    let mut attempts: u64 = 0;

    loop {
        if !running.load(Ordering::Relaxed) {
            return None;
        }

        attempts += 1;
        let combo = generate_combination(&mut rng);

        if &combo == target {
            return Some(attempts);
        }
    }
}

fn print_statistics(results: &[u64], elapsed: std::time::Duration) {
    if results.is_empty() {
        println!("\nNo draws completed.");
        return;
    }

    let mut sorted = results.to_vec();
    sorted.sort();

    let sum: u64 = sorted.iter().sum();
    let count = sorted.len();
    let average = sum as f64 / count as f64;
    let min = sorted[0];
    let max = sorted[count - 1];
    let median = if count % 2 == 0 {
        (sorted[count / 2 - 1] + sorted[count / 2]) as f64 / 2.0
    } else {
        sorted[count / 2] as f64
    };

    println!("\n========== FINAL STATISTICS ==========");
    println!("Draws completed: {}", count);
    println!("Total time: {:.2?}", elapsed);
    println!("---------------------------------------");
    println!("Average attempts: {:.2}", average);
    println!("Median attempts:  {:.2}", median);
    println!("Min attempts:     {}", min);
    println!("Max attempts:     {}", max);
    println!("=======================================");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Mega-Sena Average Attempts Calculator ===\n");
    println!("Using {} threads\n", rayon::current_num_threads());

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("\n\nReceived Ctrl+C, finishing current draws...");
        r.store(false, Ordering::SeqCst);
    })?;

    let historical_data = fetch_historical_data()?;

    // Sort draws by number (descending: last to first)
    let mut draws: Vec<(u32, BTreeSet<u8>)> = historical_data
        .iter()
        .filter_map(|(num, numbers)| {
            let draw_num = num.parse::<u32>().ok()?;
            let set = parse_draw(numbers)?;
            Some((draw_num, set))
        })
        .collect();
    draws.sort_by(|a, b| b.0.cmp(&a.0));

    println!("Processing {} draws (last to first)...\n", draws.len());

    let start = Instant::now();
    let results: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(Vec::new()));
    let completed = AtomicUsize::new(0);

    draws.par_iter().for_each(|(draw_num, target)| {
        if !running.load(Ordering::Relaxed) {
            return;
        }

        if let Some(attempts) = find_match(target, &running) {
            let mut res = results.lock().unwrap();
            res.push(attempts);
            let count = res.len();
            let avg = res.iter().sum::<u64>() as f64 / count as f64;
            let done = completed.fetch_add(1, Ordering::Relaxed) + 1;

            println!(
                "Draw #{}: found in {} attempts (avg: {:.2}, completed: {})",
                draw_num, attempts, avg, done
            );
        }
    });

    let final_results = results.lock().unwrap();
    print_statistics(&final_results, start.elapsed());

    Ok(())
}
