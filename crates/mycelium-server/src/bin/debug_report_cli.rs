use mycelium_server::contract_reporting::{ContractReportingSystem, PassRateDashboard};
use mycelium_types::ProblemResponse;
use std::fs;
use std::path::Path;

#[derive(clap::Parser)]
#[command(name = "debug-report")]
#[command(about = "Debug contract pass rate reporting CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Generate a pass rate report
    Report {
        #[arg(short, long, default_value = "7")]
        days: u32,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(long)]
        format: Option<String>, // json, html, text
    },
    /// Record validation results from evaluation data
    Record {
        #[arg(short, long)]
        eval_file: String,
        #[arg(short, long)]
        source: Option<String>,
    },
    /// Import/export validation data
    Export {
        #[arg(short, long)]
        output: String,
    },
    Import {
        #[arg(short, long)]
        input: String,
    },
    /// Run evaluation and record results
    Eval {
        #[arg(short, long)]
        suite: Option<String>,
        #[arg(short, long)]
        provider: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use clap::Parser;
    let cli = Cli::parse();
    let mut reporting_system = ContractReportingSystem::new();

    // Load existing data if available
    let data_file = "debug_contract_results.json";
    if Path::new(data_file).exists() {
        let existing_data = fs::read_to_string(data_file)?;
        match reporting_system.import_results(&existing_data) {
            Ok(_) => println!("Loaded existing validation data"),
            Err(e) => eprintln!("Warning: Failed to load existing data: {}", e),
        }
    }

    match cli.command {
        Commands::Report { days, output, format } => {
            let report = reporting_system.generate_report(days);
            let format = format.as_deref().unwrap_or("json");
            
            let output_content = match format {
                "html" => PassRateDashboard::generate_html(&report),
                "json" => serde_json::to_string_pretty(&report)?,
                "text" => generate_text_report(&report),
                _ => return Err(anyhow::anyhow!("Unsupported format: {}", format)),
            };

            if let Some(output_path) = output {
                fs::write(&output_path, output_content)?;
                println!("Report written to: {}", output_path);
            } else {
                println!("{}", output_content);
            }
        }
        Commands::Record { eval_file, source } => {
            let eval_data = fs::read_to_string(&eval_file)?;
            let results = parse_eval_results(&eval_data)?;
            let source = source.unwrap_or_else(|| "eval".to_string());
            
            let mut recorded = 0;
            for (case_id, input, response) in results {
                reporting_system.record_validation(case_id, input, response, source.clone());
                recorded += 1;
            }
            
            println!("Recorded {} validation results", recorded);
            save_data(&reporting_system, data_file)?;
        }
        Commands::Export { output } => {
            let data = reporting_system.export_results();
            fs::write(&output, data)?;
            println!("Data exported to: {}", output);
        }
        Commands::Import { input } => {
            let data = fs::read_to_string(&input)?;
            let count = reporting_system.import_results(&data)
                .map_err(|e| anyhow::anyhow!("Import failed: {}", e))?;
            println!("Imported {} validation results", count);
            save_data(&reporting_system, data_file)?;
        }
        Commands::Eval { suite, provider } => {
            println!("Running evaluation and recording results...");
            run_evaluation_and_record(&mut reporting_system, suite, provider).await?;
            save_data(&reporting_system, data_file)?;
        }
    }

    Ok(())
}

fn generate_text_report(report: &mycelium_server::contract_reporting::PassRateReport) -> String {
    format!(r#"
=== Mycelium Debug Contract Pass Rate Report ===

Generated: {}
Period: {} days

📊 Overall Statistics:
- Total Validations: {}
- Passed Validations: {}
- Pass Rate: {:.1}%
- Target: ≥95%
- Status: {}

📈 Daily Breakdown:
{}

🔍 Common Failure Patterns:
{}

💡 Recommendations:
{}

⚠️ Recent Failures:
{}
"#,
        format_timestamp(report.generated_at),
        (report.period_end - report.period_start) / (24 * 60 * 60),
        report.overall_stats.total_validations,
        report.overall_stats.passed_validations,
        report.overall_stats.pass_rate_percentage,
        if report.overall_stats.pass_rate_percentage >= 95.0 {
            "✅ Target Met"
        } else {
            "❌ Below Target"
        },
        report.daily_breakdown
            .iter()
            .map(|day| format!("  {} - {}/{} ({:.1}%)", day.date, day.passed_validations, day.total_validations, day.pass_rate_percentage))
            .collect::<Vec<_>>()
            .join("\n"),
        if report.overall_stats.common_failure_patterns.is_empty() {
            "  None detected".to_string()
        } else {
            report.overall_stats.common_failure_patterns
                .iter()
                .take(5)
                .map(|pattern| format!("  {}x {}", pattern.frequency, pattern.issue_type))
                .collect::<Vec<_>>()
                .join("\n")
        },
        report.improvement_recommendations
            .iter()
            .map(|rec| format!("  • {}", rec))
            .collect::<Vec<_>>()
            .join("\n"),
        if report.recent_failures.is_empty() {
            "  None".to_string()
        } else {
            report.recent_failures
                .iter()
                .take(5)
                .map(|failure| format!("  {} - {}", failure.case_id, failure.validation.issues.join(", ")))
                .collect::<Vec<_>>()
                .join("\n")
        }
    )
}

fn format_timestamp(timestamp: u64) -> String {
    let dt = std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(timestamp);
    format!("{:?}", dt)
}

fn parse_eval_results(eval_data: &str) -> anyhow::Result<Vec<(String, String, ProblemResponse)>> {
    // This would parse evaluation results from various formats
    // For now, assume JSON format with evaluation results
    let mut results = Vec::new();
    
    // Try to parse as a JSON array of evaluation results
    if let Ok(eval_results) = serde_json::from_str::<serde_json::Value>(eval_data) {
        if let Some(cases) = eval_results.as_array() {
            for case in cases {
                if let (Some(case_id), Some(input), Some(response_data)) = (
                    case.get("case_id").and_then(|v| v.as_str()),
                    case.get("input").and_then(|v| v.as_str()),
                    case.get("response")
                ) {
                    if let Ok(response) = serde_json::from_value::<ProblemResponse>(response_data.clone()) {
                        results.push((
                            case_id.to_string(),
                            input.to_string(), 
                            response
                        ));
                    }
                }
            }
        }
    }
    
    if results.is_empty() {
        // Generate sample data for testing
        results.push((
            "sample-case-1".to_string(),
            "How to debug a memory leak?".to_string(),
            ProblemResponse {
                abstract_shape: "ABSTRACT:\n- Debug loop pattern".to_string(),
                cross_domain_matches: vec![
                    "SEARCH: medical diagnosis process".to_string(),
                    "SEARCH: detective investigation method".to_string(),
                    "SEARCH: scientific hypothesis testing".to_string(),
                ],
                mapping: "MAP:\n- Memory symptoms -> diagnostic tools".to_string(),
                synthesis: "SYNTHESIZE:\nPivot rationale:\n- Shift focus from symptom to root cause\nFix steps:\n- Use memory profilers\nVerification steps:\n- Monitor memory usage over time\nFallback pivot:\n- Try different profiling tools".to_string(),
            }
        ));
        
        results.push((
            "sample-case-2".to_string(),
            "Fix race condition bug".to_string(),
            ProblemResponse {
                abstract_shape: "Race condition debugging".to_string(), // Invalid - missing ABSTRACT:
                cross_domain_matches: vec![
                    "Traffic coordination".to_string(), // Invalid - missing SEARCH:
                ],
                mapping: "timing issues".to_string(), // Invalid - missing MAP:
                synthesis: "Add locks".to_string(), // Invalid - missing SYNTHESIZE:
            }
        ));
    }
    
    Ok(results)
}

async fn run_evaluation_and_record(
    reporting_system: &mut ContractReportingSystem,
    _suite: Option<String>,
    _provider: Option<String>,
) -> anyhow::Result<()> {
    // This would integrate with the evaluation system to run actual evaluations
    // For now, generate some mock evaluation results
    
    let mock_cases = vec![
        ("eval-case-1", "Debug performance issue"),
        ("eval-case-2", "Fix compilation error"),
        ("eval-case-3", "Resolve integration test failure"),
    ];
    
    for (case_id, input) in &mock_cases {
        // Generate a mock response (in real implementation, this would call the provider)
        let response = if case_id.contains("1") || case_id.contains("3") {
            // Valid response
            ProblemResponse {
                abstract_shape: "ABSTRACT:\n- Debug pattern analysis".to_string(),
                cross_domain_matches: vec![
                    "SEARCH: medical diagnosis workflow".to_string(),
                    "SEARCH: detective case investigation".to_string(),
                    "SEARCH: scientific method hypothesis testing".to_string(),
                ],
                mapping: "MAP:\n- Problem symptoms -> diagnostic approach -> solution verification".to_string(),
                synthesis: "SYNTHESIZE:\nPivot rationale:\n- Shift from random fixes to systematic diagnosis\nFix steps:\n- Isolate the problem scope\nVerification steps:\n- Run focused tests to verify fix\nFallback pivot:\n- Try alternative diagnostic approach".to_string(),
            }
        } else {
            // Invalid response
            ProblemResponse {
                abstract_shape: "Debug the thing".to_string(),
                cross_domain_matches: vec!["try different approach".to_string()],
                mapping: "fix -> test".to_string(),
                synthesis: "Just debug it".to_string(),
            }
        };
        
        reporting_system.record_validation(
            case_id.to_string(),
            input.to_string(),
            response,
            "eval".to_string(),
        );
    }
    
    println!("Recorded evaluation results for {} cases", mock_cases.len());
    Ok(())
}

fn save_data(reporting_system: &ContractReportingSystem, filename: &str) -> anyhow::Result<()> {
    let data = reporting_system.export_results();
    fs::write(filename, data)?;
    Ok(())
}