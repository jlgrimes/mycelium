use mycelium_eval::{
    actionability_delta, load_snapshot, write_snapshot, ActionabilitySnapshot, BenchmarkSuite,
    EvalConfig, EvalRunner, RunMode, ScoreReport, DEBUGGING_V1_CASES, ISOMORPHIC_TRANSFER_CASES, SEED_CASES,
};
use mycelium_providers::StubProvider;
use std::path::Path;
use std::sync::Arc;

fn print_report(label: &str, report: &ScoreReport) {
    println!("\n=== {label} ===");
    println!(
        "Score: {:.1}% | Actionability: {:.2}/5 | Verification: {:.1}% | Passed: {}/{}\n",
        report.mean_score * 100.0,
        report.mean_actionability,
        report.verification_rate * 100.0,
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
            "  [{status}] {:<25} {:.0}% act={}/5 verify={}",
            result.case_id,
            result.overall * 100.0,
            result.actionability_score,
            if result.verification_presence {
                "yes"
            } else {
                "no"
            }
        );
        if let Some(err) = &result.error {
            print!("  ERROR: {err}");
        }
        println!();
    }
}

fn print_comparison(baseline: &ScoreReport, staged: &ScoreReport) {
    println!("\n=== Comparison: Baseline vs Staged ===");
    println!(
        "Baseline: {:.1}% (act {:.2}/5, verify {:.1}%)",
        baseline.mean_score * 100.0,
        baseline.mean_actionability,
        baseline.verification_rate * 100.0,
    );
    println!(
        "Staged:   {:.1}% (act {:.2}/5, verify {:.1}%)",
        staged.mean_score * 100.0,
        staged.mean_actionability,
        staged.verification_rate * 100.0,
    );
    let delta = (staged.mean_score - baseline.mean_score) * 100.0;
    let arrow = if delta > 0.0 { "+" } else { "" };
    println!("Delta:    {arrow}{delta:.1}pp");
}

fn parse_suite(args: &[String]) -> BenchmarkSuite {
    if let Some(value) = args.windows(2).find(|w| w[0] == "--suite").map(|w| &w[1]) {
        match value.as_str() {
            "debugging-v1" => return BenchmarkSuite::DebuggingV1,
            "isomorphic-transfer" => return BenchmarkSuite::IsomorphicTransfer,
            _ => {}
        }
    }
    BenchmarkSuite::Seed
}

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|arg| arg == flag)
}

fn parse_value(args: &[String], flag: &str) -> Option<String> {
    args.windows(2)
        .find(|w| w[0] == flag)
        .map(|w| w[1].clone())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let filter = args
        .windows(2)
        .find(|w| w[0] == "--filter")
        .map(|w| w[1].split(',').map(String::from).collect::<Vec<_>>());

    let suite = parse_suite(&args);
    let list_mode = has_flag(&args, "--list");
    if list_mode {
        let cases = match suite {
            BenchmarkSuite::Seed => SEED_CASES,
            BenchmarkSuite::DebuggingV1 => DEBUGGING_V1_CASES,
            BenchmarkSuite::IsomorphicTransfer => ISOMORPHIC_TRANSFER_CASES,
        };
        println!(
            "Available benchmark cases for suite `{suite}` ({}):",
            cases.len()
        );
        for case in cases {
            println!("  {:<25} {}", case.id, case.input);
        }
        return Ok(());
    }

    let snapshot_enabled = has_flag(&args, "--snapshot");
    let snapshot_dir = parse_value(&args, "--snapshot-dir")
        .unwrap_or_else(|| "reports/actionability".to_string());
    let delta_path = parse_value(&args, "--delta");

    let config = EvalConfig { filter, suite };

    let provider: Arc<dyn mycelium_core::ReasoningProvider> = Arc::new(StubProvider);

    let baseline_runner = EvalRunner::new(Arc::clone(&provider), RunMode::Baseline);
    let staged_runner = EvalRunner::new(provider, RunMode::Staged);

    let baseline_report = baseline_runner.run(&config).await;
    let staged_report = staged_runner.run(&config).await;

    print_report("Baseline (single-pass)", &baseline_report);
    print_report("Staged (pipeline)", &staged_report);
    print_comparison(&baseline_report, &staged_report);

    let snapshot = ActionabilitySnapshot::new(suite, baseline_report, staged_report);

    if snapshot_enabled {
        let path = write_snapshot(&snapshot, Path::new(&snapshot_dir))?;
        println!("\nSnapshot written to {}", path.display());
    }

    if let Some(path) = delta_path {
        let previous = load_snapshot(Path::new(&path))?;
        if previous.suite_label() != snapshot.suite_label() {
            println!(
                "\nWarning: comparing suite '{}' against '{}'",
                previous.suite_label(),
                snapshot.suite_label()
            );
        }
        let diff = actionability_delta(&snapshot, &previous);
        println!("\n=== Actionability Delta vs {} ===", path);
        println!(
            "Baseline: score {:+.1}pp | actionability {:+.2} | verification {:+.1}pp",
            diff.baseline_score_delta * 100.0,
            diff.baseline_actionability_delta,
            diff.baseline_verification_delta * 100.0
        );
        println!(
            "Staged:   score {:+.1}pp | actionability {:+.2} | verification {:+.1}pp",
            diff.staged_score_delta * 100.0,
            diff.staged_actionability_delta,
            diff.staged_verification_delta * 100.0
        );
    }

    Ok(())
}
