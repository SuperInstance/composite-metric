# Composite Metric — Weighted Multi-Metric Scoring

**Composite metric computation** combines multiple individual metrics — each measuring a different aspect of system health — into a single normalized score via configurable weights and inversion rules. This crate computes weighted averages where "lower is better" metrics (latency, error rate) are inverted before scoring.

## Why It Matters

SREs and platform engineers rarely care about a single metric in isolation. "Is the service healthy?" depends on latency, throughput, error rate, saturation, and uptime simultaneously. A composite score — a single number from 0 to 1 — provides an at-a-glance health indicator that can drive alerting, autoscaling, and capacity decisions. Google's SRE book formalizes this as Service Level Objectives (SLOs) with multi-window multi-burn-rate alerting. Netflix's Atlas, Datadog's composite monitors, and PagerDuty's service health scores all use this pattern. The key design decision is the weight allocation: how much does latency matter relative to error rate? This crate makes that explicit and tunable.

## How It Works

Given `n` metrics with values `vᵢ ∈ [0, 1]`, weights `wᵢ > 0`, and inversion flags `invᵢ`:

**Step 1 — Normalize** (invert if needed):

```
v'ᵢ = invᵢ ? (1 - vᵢ) : vᵢ
```

For "lower is better" metrics (latency, error rate), inversion transforms a low value (good) into a high value (good score).

**Step 2 — Weighted average**:

```
S = Σ(v'ᵢ × wᵢ) / Σ(wᵢ)
```

The denominator normalizes weights so they sum to 1, allowing arbitrary weight magnitudes. A score of `1.0` means all metrics are perfect; `0.0` means all are at their worst.

### Example Computation

| Metric | Raw Value | Weight | Invert | Normalized | Weighted |
|---|---|---|---|---|---|
| Latency (p99) | 0.25 | 0.3 | yes | 0.75 | 0.225 |
| Throughput | 0.82 | 0.4 | no | 0.82 | 0.328 |
| Error rate | 0.03 | 0.3 | yes | 0.97 | 0.291 |
| **Total** | | **1.0** | | | **0.844** |

Score `0.844` → Grade B. The service is healthy but has room for improvement in throughput.

### Linear Weighting vs. Alternatives

This crate uses **linear weighting** — the simplest aggregation. Alternatives include:

- **Geometric mean**: `S = (Π v'ᵢ)^{1/n}` — penalizes any single low metric more heavily.
- ** harmonic mean**: `S = n / Σ(1/v'ᵢ)` — even more sensitive to worst-case.
- **Min** (T-norm): `S = min(v'ᵢ)` — the weakest link determines the score.

Linear weighting is appropriate when metrics are independent and commensurate.

## Quick Start

```rust
use std::collections::HashMap;

struct MetricConfig { weight: f64, invert: bool }

fn compute_composite(
    values: &HashMap<String, f64>,
    config: &HashMap<String, MetricConfig>,
) -> f64 {
    let total_weight: f64 = config.values().map(|c| c.weight).sum();
    let score: f64 = values.iter().map(|(name, &val)| {
        let cfg = config.get(name).expect("missing metric config");
        let normalized = if cfg.invert { 1.0 - val } else { val };
        normalized * cfg.weight
    }).sum();
    score / total_weight
}

let mut config = HashMap::new();
config.insert("latency".into(), MetricConfig { weight: 0.3, invert: true });
config.insert("throughput".into(), MetricConfig { weight: 0.4, invert: false });
config.insert("error_rate".into(), MetricConfig { weight: 0.3, invert: true });

let mut values = HashMap::new();
values.insert("latency".into(), 0.25);
values.insert("throughput".into(), 0.82);
values.insert("error_rate".into(), 0.03);

let score = compute_composite(&values, &config);
println!("Composite score: {score:.4}");  // ~0.844
```

## API

| Function | Description |
|---|---|
| `compute_composite(values, config)` | Weighted average with per-metric inversion. `O(n)` where n = metric count. |

| Type | Description |
|---|---|
| `MetricConfig` | `{ weight: f64, invert: bool }` — per-metric configuration. |

## Architecture Notes

Composite metrics serve the η (evaluation) side of γ + η = C in SuperInstance. They aggregate fleet telemetry into actionable health scores that drive alerting, autoscaling, and SLO compliance tracking. See [SuperInstance Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

1. Beyer, B. et al. (2016). *Site Reliability Engineering*, Ch. 6: Monitoring. Google. — SLOs and multi-window alerting.
2. Netflix Atlas. <https://github.com/Netflix/atlas> — Composite metric computation at scale.
3. Bouwman, M. et al. (2020). *Implementing Service Level Objectives*. O'Reilly.

## License

MIT
