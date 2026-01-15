# Mega-Sena Random Match Finder

Rust CLI that calculates the average attempts needed to randomly match historical Mega-Sena results.

## Build & Run

```bash
cargo run --release
```

Press `Ctrl+C` to stop and see statistics.

## How It Works

1. Fetches ~2,800 historical draws from [loteria.json](https://github.com/guilhermeasn/loteria.json)
2. Iterates from last to first draw
3. For each draw, generates random 6-number combinations (1-60) until matching
4. Displays progress: `Draw #N: found in X attempts (avg: Y)`
5. On exit, shows final statistics: average, median, min, max

## Expected Results

Theoretical average: ~50 million attempts per draw (C(60,6) = 50,063,860 combinations)

## Dependencies

- `rand` - random number generation
- `reqwest` - HTTP client
- `serde` / `serde_json` - JSON parsing
- `ctrlc` - graceful shutdown
