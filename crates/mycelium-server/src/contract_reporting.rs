use crate::debug_contract::{ContractValidation, DebugContractValidator};
use mycelium_types::ProblemResponse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// A single debug contract validation result with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub timestamp: u64,
    pub case_id: String,
    pub input: String,
    pub response: ProblemResponse,
    pub validation: ContractValidation,
    pub source: String, // e.g., "eval", "api", "test"
}

/// Aggregated pass rate statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassRateStats {
    pub total_validations: usize,
    pub passed_validations: usize,
    pub pass_rate_percentage: f64,
    pub common_failure_patterns: Vec<FailurePattern>,
    pub confidence_distribution: HashMap<String, usize>, // "high", "medium", "low" -> count
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailurePattern {
    pub issue_type: String,
    pub frequency: usize,
    pub examples: Vec<String>, // Sample case IDs that exhibit this pattern
}

/// Time-based pass rate tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassRateReport {
    pub generated_at: u64,
    pub period_start: u64,
    pub period_end: u64,
    pub overall_stats: PassRateStats,
    pub daily_breakdown: Vec<DailyStats>,
    pub recent_failures: Vec<ValidationResult>,
    pub improvement_recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyStats {
    pub date: String, // YYYY-MM-DD format
    pub total_validations: usize,
    pub passed_validations: usize,
    pub pass_rate_percentage: f64,
}

/// In-memory store for validation results (in production, this would use a database)
pub struct ContractReportingSystem {
    results: Vec<ValidationResult>,
}

impl Default for ContractReportingSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractReportingSystem {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// Record a validation result
    pub fn record_validation(
        &mut self,
        case_id: String,
        input: String,
        response: ProblemResponse,
        source: String,
    ) -> ValidationResult {
        let validation = DebugContractValidator::validate(&response);
        let result = ValidationResult {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            case_id,
            input,
            response,
            validation,
            source,
        };
        
        self.results.push(result.clone());
        result
    }

    /// Generate a pass rate report for the specified time period
    pub fn generate_report(&self, days_back: u32) -> PassRateReport {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let period_start = now - (days_back as u64 * 24 * 60 * 60);

        let period_results: Vec<&ValidationResult> = self.results
            .iter()
            .filter(|r| r.timestamp >= period_start)
            .collect();

        let overall_stats = self.calculate_stats(&period_results);
        let daily_breakdown = self.calculate_daily_breakdown(&period_results, period_start);
        let recent_failures = self.get_recent_failures(&period_results, 10);
        let improvement_recommendations = self.generate_recommendations(&overall_stats);

        PassRateReport {
            generated_at: now,
            period_start,
            period_end: now,
            overall_stats,
            daily_breakdown,
            recent_failures,
            improvement_recommendations,
        }
    }

    fn calculate_stats(&self, results: &[&ValidationResult]) -> PassRateStats {
        let total = results.len();
        let passed = results.iter().filter(|r| r.validation.valid).count();
        let pass_rate = if total > 0 {
            (passed as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        // Analyze common failure patterns
        let mut issue_counts: HashMap<String, Vec<String>> = HashMap::new();
        for result in results.iter().filter(|r| !r.validation.valid) {
            for issue in &result.validation.issues {
                issue_counts
                    .entry(issue.clone())
                    .or_default()
                    .push(result.case_id.clone());
            }
        }

        let common_failure_patterns: Vec<FailurePattern> = issue_counts
            .into_iter()
            .map(|(issue_type, examples)| FailurePattern {
                frequency: examples.len(),
                issue_type,
                examples: examples.into_iter().take(5).collect(), // Limit examples
            })
            .collect();

        // Analyze confidence distribution
        let mut confidence_distribution = HashMap::new();
        for result in results {
            *confidence_distribution
                .entry(result.validation.confidence.clone())
                .or_insert(0) += 1;
        }

        PassRateStats {
            total_validations: total,
            passed_validations: passed,
            pass_rate_percentage: pass_rate,
            common_failure_patterns,
            confidence_distribution,
        }
    }

    fn calculate_daily_breakdown(&self, results: &[&ValidationResult], _period_start: u64) -> Vec<DailyStats> {
        let mut daily_results: HashMap<String, Vec<&ValidationResult>> = HashMap::new();
        
        for result in results {
            let date = Self::timestamp_to_date(result.timestamp);
            daily_results.entry(date).or_default().push(result);
        }

        let mut daily_stats: Vec<DailyStats> = daily_results
            .into_iter()
            .map(|(date, day_results)| {
                let total = day_results.len();
                let passed = day_results.iter().filter(|r| r.validation.valid).count();
                let pass_rate = if total > 0 {
                    (passed as f64 / total as f64) * 100.0
                } else {
                    0.0
                };

                DailyStats {
                    date,
                    total_validations: total,
                    passed_validations: passed,
                    pass_rate_percentage: pass_rate,
                }
            })
            .collect();

        daily_stats.sort_by(|a, b| a.date.cmp(&b.date));
        daily_stats
    }

    fn get_recent_failures(&self, results: &[&ValidationResult], limit: usize) -> Vec<ValidationResult> {
        let mut failures: Vec<ValidationResult> = results
            .iter()
            .filter(|r| !r.validation.valid)
            .map(|r| (*r).clone())
            .collect();
        
        failures.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        failures.into_iter().take(limit).collect()
    }

    fn generate_recommendations(&self, stats: &PassRateStats) -> Vec<String> {
        let mut recommendations = Vec::new();

        if stats.pass_rate_percentage < 95.0 {
            recommendations.push(format!(
                "Current pass rate ({:.1}%) is below target (95%). Focus on improving validation compliance.",
                stats.pass_rate_percentage
            ));
        }

        // Analyze common failure patterns and suggest improvements
        let most_common_failures: Vec<_> = stats.common_failure_patterns
            .iter()
            .filter(|p| p.frequency > 1)
            .collect();

        for pattern in most_common_failures.iter().take(3) {
            if pattern.issue_type.contains("ABSTRACT:") {
                recommendations.push("High frequency of ABSTRACT: prefix violations. Consider improving prompt engineering to emphasize required format.".to_string());
            } else if pattern.issue_type.contains("SEARCH:") {
                recommendations.push("Common SEARCH: prefix violations detected. Review cross-domain match generation logic.".to_string());
            } else if pattern.issue_type.contains("verification") {
                recommendations.push("Verification presence issues detected. Enhance synthesis generation to include explicit verification steps.".to_string());
            } else if pattern.issue_type.contains("3 items") {
                recommendations.push("Cross-domain match count violations. Ensure minimum 3 matches are consistently generated.".to_string());
            }
        }

        if recommendations.is_empty() {
            recommendations.push("Pass rate meets or exceeds target. Continue monitoring for regressions.".to_string());
        }

        recommendations
    }

    fn timestamp_to_date(timestamp: u64) -> String {
        // Simple date formatting - in production, use chrono
        let days_since_epoch = timestamp / (24 * 60 * 60);
        let epoch_date = chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        let date = epoch_date + chrono::Duration::days(days_since_epoch as i64);
        date.format("%Y-%m-%d").to_string()
    }

    /// Export results as JSON for external analysis
    #[allow(dead_code)]
    pub fn export_results(&self) -> String {
        serde_json::to_string_pretty(&self.results).unwrap_or_default()
    }

    /// Load results from JSON (for persistence)
    #[allow(dead_code)]
    pub fn import_results(&mut self, json_data: &str) -> anyhow::Result<usize> {
        let results: Vec<ValidationResult> = serde_json::from_str(json_data)?;
        let count = results.len();
        self.results.extend(results);
        Ok(count)
    }

    #[cfg(test)]
    pub fn get_results_count(&self) -> usize {
        self.results.len()
    }
}

/// HTML dashboard generator
pub struct PassRateDashboard;

impl PassRateDashboard {
    pub fn generate_html(report: &PassRateReport) -> String {
        let pass_rate_color = if report.overall_stats.pass_rate_percentage >= 95.0 {
            "green"
        } else if report.overall_stats.pass_rate_percentage >= 90.0 {
            "orange" 
        } else {
            "red"
        };

        format!(r#"
<!DOCTYPE html>
<html>
<head>
    <title>Mycelium Debug Contract Pass Rate Dashboard</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; background-color: #f5f5f5; }}
        .header {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 20px; border-radius: 8px; margin-bottom: 20px; }}
        .metric-card {{ background: white; padding: 20px; margin: 10px 0; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        .pass-rate {{ font-size: 2.5em; font-weight: bold; color: {}; }}
        .target-indicator {{ font-size: 0.9em; color: #666; }}
        .failure-list {{ max-height: 300px; overflow-y: auto; }}
        .failure-item {{ border-left: 3px solid #ff6b6b; padding: 10px; margin: 5px 0; background: #fff5f5; }}
        .recommendation {{ background: #e3f2fd; border-left: 4px solid #2196f3; padding: 10px; margin: 5px 0; }}
        .daily-chart {{ display: flex; align-items: end; height: 100px; gap: 2px; }}
        .daily-bar {{ background: #4caf50; min-width: 20px; display: flex; align-items: end; justify-content: center; color: white; font-size: 0.8em; }}
        table {{ width: 100%; border-collapse: collapse; }}
        th, td {{ text-align: left; padding: 8px; border-bottom: 1px solid #ddd; }}
        th {{ background-color: #f2f2f2; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>🧪 Mycelium Debug Contract Pass Rate Dashboard</h1>
        <p>Generated: {} | Period: {} days</p>
    </div>

    <div class="metric-card">
        <h2>📊 Overall Pass Rate</h2>
        <div class="pass-rate">{:.1}%</div>
        <div class="target-indicator">Target: ≥95% | {}/{} validations passed</div>
    </div>

    <div class="metric-card">
        <h2>📈 Daily Breakdown</h2>
        <div class="daily-chart">
            {}
        </div>
        <table>
            <tr><th>Date</th><th>Validations</th><th>Passed</th><th>Pass Rate</th></tr>
            {}
        </table>
    </div>

    <div class="metric-card">
        <h2>🔍 Common Failure Patterns</h2>
        {}
    </div>

    <div class="metric-card">
        <h2>💡 Recommendations</h2>
        {}
    </div>

    <div class="metric-card">
        <h2>⚠️ Recent Failures</h2>
        <div class="failure-list">
            {}
        </div>
    </div>
</body>
</html>
        "#,
        pass_rate_color,
        Self::format_timestamp(report.generated_at),
        (report.period_end - report.period_start) / (24 * 60 * 60),
        report.overall_stats.pass_rate_percentage,
        report.overall_stats.passed_validations,
        report.overall_stats.total_validations,
        Self::generate_daily_chart(&report.daily_breakdown),
        Self::generate_daily_table(&report.daily_breakdown),
        Self::generate_failure_patterns(&report.overall_stats.common_failure_patterns),
        Self::generate_recommendations(&report.improvement_recommendations),
        Self::generate_recent_failures(&report.recent_failures)
        )
    }

    fn format_timestamp(timestamp: u64) -> String {
        let dt = std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(timestamp);
        format!("{:?}", dt)
    }

    fn generate_daily_chart(daily_stats: &[DailyStats]) -> String {
        daily_stats
            .iter()
            .map(|day| {
                let height = ((day.pass_rate_percentage / 100.0) * 80.0).max(5.0) as u32;
                let color = if day.pass_rate_percentage >= 95.0 {
                    "#4caf50"
                } else if day.pass_rate_percentage >= 90.0 {
                    "#ff9800"
                } else {
                    "#f44336"
                };
                format!(
                    r#"<div class="daily-bar" style="height: {}px; background: {};" title="{}: {:.1}%">{:.0}%</div>"#,
                    height, color, day.date, day.pass_rate_percentage, day.pass_rate_percentage
                )
            })
            .collect::<Vec<_>>()
            .join("")
    }

    fn generate_daily_table(daily_stats: &[DailyStats]) -> String {
        daily_stats
            .iter()
            .map(|day| {
                format!(
                    "<tr><td>{}</td><td>{}</td><td>{}</td><td>{:.1}%</td></tr>",
                    day.date, day.total_validations, day.passed_validations, day.pass_rate_percentage
                )
            })
            .collect::<Vec<_>>()
            .join("")
    }

    fn generate_failure_patterns(patterns: &[FailurePattern]) -> String {
        if patterns.is_empty() {
            return "<p>No common failure patterns detected.</p>".to_string();
        }

        patterns
            .iter()
            .take(5)
            .map(|pattern| {
                format!(
                    "<div class='failure-item'><strong>{}x</strong> {}<br><small>Examples: {}</small></div>",
                    pattern.frequency,
                    pattern.issue_type,
                    pattern.examples.join(", ")
                )
            })
            .collect::<Vec<_>>()
            .join("")
    }

    fn generate_recommendations(recommendations: &[String]) -> String {
        recommendations
            .iter()
            .map(|rec| format!("<div class='recommendation'>{}</div>", rec))
            .collect::<Vec<_>>()
            .join("")
    }

    fn generate_recent_failures(failures: &[ValidationResult]) -> String {
        if failures.is_empty() {
            return "<p>No recent failures to display.</p>".to_string();
        }

        failures
            .iter()
            .take(10)
            .map(|failure| {
                format!(
                    "<div class='failure-item'><strong>{}</strong> ({})<br><small>Issues: {}</small></div>",
                    failure.case_id,
                    Self::format_timestamp(failure.timestamp),
                    failure.validation.issues.join(", ")
                )
            })
            .collect::<Vec<_>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_response(valid: bool) -> ProblemResponse {
        if valid {
            ProblemResponse {
                abstract_shape: "ABSTRACT:\n- Debug loop pattern with systematic verification".to_string(),
                cross_domain_matches: vec![
                    "SEARCH: Medical diagnosis systematic approach".to_string(),
                    "SEARCH: Detective investigation methodology".to_string(),
                    "SEARCH: Scientific hypothesis testing framework".to_string(),
                ],
                mapping: "MAP:\n- Problem symptoms -> diagnostic tools -> solution verification\n- Mapping confidence: high".to_string(),
                synthesis: "SYNTHESIZE:\nPivot rationale:\n- Shift from random fixes to systematic approach\nFix steps:\n- Isolate problem scope and run diagnostics\nVerification steps:\n- Create test case and verify fix resolves issue\nFallback pivot:\n- Try alternative diagnostic method if verification fails".to_string(),
            }
        } else {
            ProblemResponse {
                abstract_shape: "Invalid pattern".to_string(),
                cross_domain_matches: vec!["match1".to_string()],
                mapping: "invalid".to_string(),
                synthesis: "invalid".to_string(),
            }
        }
    }

    #[test]
    fn reporting_system_records_validations() {
        let mut system = ContractReportingSystem::new();
        
        let result = system.record_validation(
            "test-case-1".to_string(),
            "test input".to_string(),
            sample_response(true),
            "test".to_string(),
        );
        
        if !result.validation.valid {
            println!("Validation failed with issues: {:?}", result.validation.issues);
        }
        
        assert!(result.validation.valid);
        assert_eq!(system.get_results_count(), 1);
    }

    #[test]
    fn report_calculates_pass_rates_correctly() {
        let mut system = ContractReportingSystem::new();
        
        // Add valid results
        for i in 0..8 {
            system.record_validation(
                format!("valid-case-{}", i),
                "test input".to_string(),
                sample_response(true),
                "test".to_string(),
            );
        }
        
        // Add invalid results  
        for i in 0..2 {
            system.record_validation(
                format!("invalid-case-{}", i),
                "test input".to_string(),
                sample_response(false),
                "test".to_string(),
            );
        }
        
        let report = system.generate_report(7);
        assert_eq!(report.overall_stats.total_validations, 10);
        assert_eq!(report.overall_stats.passed_validations, 8);
        assert_eq!(report.overall_stats.pass_rate_percentage, 80.0);
    }

    #[test]
    fn dashboard_generates_valid_html() {
        let mut system = ContractReportingSystem::new();
        system.record_validation(
            "test-case".to_string(),
            "test input".to_string(),
            sample_response(true),
            "test".to_string(),
        );
        
        let report = system.generate_report(7);
        let html = PassRateDashboard::generate_html(&report);
        
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Pass Rate Dashboard"));
        assert!(html.contains("100.0%")); // Should show 100% pass rate
    }
}