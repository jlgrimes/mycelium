use mycelium_eval::{EvalConfig, EvalRunner, RunMode, ScoreReport, SEED_CASES};
use mycelium_providers::StubProvider;
use std::sync::Arc;

fn print_report(label: &str, report: &ScoreReport) {
    println!("\n=== {label} ===");
    println!(
        "Score: {:.1}% | Passed: {}/{}\n",
        report.mean_score * 100.0,
        report.cases_passed,
        report.cases_total,
    );
    for result in &report.results {
        let status = if result.overall >= 0.3 {
            "PASS"
        } else {
            "FAIL"
        };
        print!(
            "  [{status}] {:<25} {:.0}%",
            result.case_id,
            result.overall * 100.0
        );
        if let Some(err) = &result.error {
            print!("  ERROR: {err}");
        }
        println!();
        for fs in &result.field_scores {
            if !fs.missed.is_empty() {
                println!("         {} missed: {}", fs.field, fs.missed.join(", "));
            }
        }
    }
}

fn print_comparison(baseline: &ScoreReport, staged: &ScoreReport) {
    println!("\n=== Comparison: Baseline vs Staged ===");
    println!(
        "Baseline: {:.1}% ({}/{} passed)",
        baseline.mean_score * 100.0,
        baseline.cases_passed,
        baseline.cases_total,
    );
    println!(
        "Staged:   {:.1}% ({}/{} passed)",
        staged.mean_score * 100.0,
        staged.cases_passed,
        staged.cases_total,
    );
    let delta = (staged.mean_score - baseline.mean_score) * 100.0;
    let arrow = if delta > 0.0 { "+" } else { "" };
    println!("Delta:    {arrow}{delta:.1}pp");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // Parse optional --filter flag
    let filter = args
        .windows(2)
        .find(|w| w[0] == "--filter")
        .map(|w| w[1].split(',').map(String::from).collect::<Vec<_>>());

    let list_mode = args.iter().any(|a| a == "--list");
    if list_mode {
        println!("Available benchmark cases ({}):", SEED_CASES.len());
        for case in SEED_CASES {
            println!("  {:<25} {}", case.id, case.input);
        }
        return Ok(());
    }

    let config = EvalConfig { filter };

    // Both modes use StubProvider for now; swap in real providers as they mature.
    let provider: Arc<dyn mycelium_core::ReasoningProvider> = Arc::new(StubProvider);

    let baseline_runner = EvalRunner::new(Arc::clone(&provider), RunMode::Baseline);
    let staged_runner = EvalRunner::new(provider, RunMode::Staged);

    let baseline_report = baseline_runner.run(&config).await;
    let staged_report = staged_runner.run(&config).await;

    print_report("Baseline (single-pass)", &baseline_report);
    print_report("Staged (pipeline)", &staged_report);
    print_comparison(&baseline_report, &staged_report);

    Ok(())
}
