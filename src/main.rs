use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{BTreeSet, HashMap};
use std::time::Instant;

const DATA_URL: &str = "https://raw.githubusercontent.com/guilhermeasn/loteria.json/master/data/megasena.json";

type DrawNumber = String;
type DrawNumbers = Vec<String>;
type HistoricalData = HashMap<DrawNumber, DrawNumbers>;

fn fetch_historical_data() -> Result<HistoricalData, Box<dyn std::error::Error>> {
    println!("Fetching historical Mega-Sena results...");
    let response = reqwest::blocking::get(DATA_URL)?;
    let data: HistoricalData = response.json()?;
    println!("Loaded {} historical draws.\n", data.len());
    Ok(data)
}

fn build_lookup_map(
    data: &HistoricalData,
) -> HashMap<BTreeSet<u8>, String> {
    let mut lookup = HashMap::new();

    for (concurso, numbers) in data {
        let set: BTreeSet<u8> = numbers
            .iter()
            .filter_map(|n| n.parse::<u8>().ok())
            .collect();

        if set.len() == 6 {
            lookup.insert(set, concurso.clone());
        }
    }

    lookup
}

fn generate_combination() -> BTreeSet<u8> {
    let mut rng = thread_rng();
    let mut numbers: Vec<u8> = (1..=60).collect();
    numbers.shuffle(&mut rng);
    numbers.into_iter().take(6).collect()
}

fn format_combination(combo: &BTreeSet<u8>) -> String {
    let nums: Vec<String> = combo.iter().map(|n| format!("{:02}", n)).collect();
    nums.join(" - ")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Mega-Sena Random Match Finder ===\n");

    let historical_data = fetch_historical_data()?;
    let lookup = build_lookup_map(&historical_data);

    println!("Starting random generation...\n");

    let start = Instant::now();
    let mut attempts: u64 = 0;

    loop {
        attempts += 1;

        if attempts % 1_000_000 == 0 {
            println!("Progress: {} million attempts...", attempts / 1_000_000);
        }

        let combo = generate_combination();

        if let Some(concurso) = lookup.get(&combo) {
            let elapsed = start.elapsed();

            println!("\n========== MATCH FOUND! ==========");
            println!("Total attempts: {}", attempts);
            println!("Time elapsed: {:.2?}", elapsed);
            println!("Matching combination: {}", format_combination(&combo));
            println!("Matched contest (concurso): #{}", concurso);
            println!("===================================");

            break;
        }
    }

    Ok(())
}
