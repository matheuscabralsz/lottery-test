# Mega-Sena Random Match Finder

Rust CLI that generates random Mega-Sena combinations until matching a historical winning result.

## Build & Run

```bash
cargo run --release
```

## How It Works

1. Fetches ~2,800 historical draws from [loteria.json](https://github.com/guilhermeasn/loteria.json)
2. Generates random 6-number combinations (1-60)
3. Checks against all historical results using O(1) HashSet lookup
4. Stops on first match, displaying attempts, time, combination, and contest number

## Dependencies

- `rand` - random number generation
- `reqwest` - HTTP client (with rustls-tls)
- `serde` / `serde_json` - JSON parsing
