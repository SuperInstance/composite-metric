/// composite-metric: Computes composite scores from multiple weighted metrics.
use std::collections::HashMap;

struct MetricConfig {
    weight: f64,
    invert: bool,
}

fn compute_composite(
    values: &HashMap<String, f64>,
    config: &HashMap<String, MetricConfig>,
) -> f64 {
    let total_weight: f64 = config.values().map(|c| c.weight).sum();
    let score: f64 = values
        .iter()
        .map(|(name, &val)| {
            let cfg = config.get(name).expect("missing metric config");
            let normalized = if cfg.invert { 1.0 - val } else { val };
            normalized * cfg.weight
        })
        .sum();

    score / total_weight
}

fn main() {
    let mut config = HashMap::new();
    config.insert("latency".into(), MetricConfig { weight: 0.3, invert: true });
    config.insert("throughput".into(), MetricConfig { weight: 0.4, invert: false });
    config.insert("error_rate".into(), MetricConfig { weight: 0.3, invert: true });

    let mut values = HashMap::new();
    values.insert("latency".into(), 0.25);
    values.insert("throughput".into(), 0.82);
    values.insert("error_rate".into(), 0.03);

    let composite = compute_composite(&values, &config);
    println!("Metrics: {values:?}");
    println!("Composite score: {composite:.4}");
    println!("Grade: {}", if composite > 0.8 { "A" } else if composite > 0.6 { "B" } else if composite > 0.4 { "C" } else { "D" });
}
