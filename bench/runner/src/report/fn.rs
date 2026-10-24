use super::*;

/// Reads raw measurement rows from a JSON Lines file.
///
/// # Arguments
///
/// - `&Path` - The raw measurement file.
///
/// # Returns
///
/// - `Result<Vec<MeasurementRow>, String>` - The rows, or the read or parse error.
fn load_rows(path: &Path) -> Result<Vec<MeasurementRow>, String> {
    let contents: String = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(format!("could not read {}: {error}", path.display())),
    };
    let mut rows: Vec<MeasurementRow> = Vec::new();
    for (index, line) in contents.lines().enumerate() {
        let trimmed: &str = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        match from_str::<MeasurementRow>(trimmed) {
            Ok(row) => rows.push(row),
            Err(error) => {
                return Err(format!(
                    "invalid measurement on line {}: {error}",
                    index + 1
                ));
            }
        }
    }
    Ok(rows)
}

/// Groups measurement rows by workload cell, variant and repeat.
///
/// # Arguments
///
/// - `Vec<MeasurementRow>` - The rows to group.
///
/// # Returns
///
/// - `Cells` - The grouped measurements.
fn group_rows(rows: Vec<MeasurementRow>) -> Cells {
    let mut grouped: Cells = BTreeMap::new();
    for row in rows {
        let MeasurementRow {
            variant,
            protocol,
            mode,
            connections,
            repeat,
            result,
        } = row;
        let cell: Cell = Cell {
            protocol,
            mode,
            connections,
        };
        grouped
            .entry(cell)
            .or_default()
            .entry(variant)
            .or_default()
            .insert(repeat, result);
    }
    grouped
}

/// Computes the median of a slice of floating-point values.
///
/// # Arguments
///
/// - `&[f64]` - The values.
///
/// # Returns
///
/// - `f64` - The median, or zero for an empty slice.
fn median(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mut sorted: Vec<f64> = values.to_vec();
    sorted.sort_by(|first: &f64, second: &f64| first.total_cmp(second));
    let middle: usize = sorted.len() / 2;
    if sorted.len() % 2 == 1 {
        return sorted.get(middle).copied().unwrap_or(0.0);
    }
    let lower: f64 = sorted.get(middle - 1).copied().unwrap_or(0.0);
    let upper: f64 = sorted.get(middle).copied().unwrap_or(0.0);
    (lower + upper) / 2.0
}

/// Finds the smallest and largest values in a nonempty slice.
///
/// # Arguments
///
/// - `&[f64]` - The values.
///
/// # Returns
///
/// - `(f64, f64)` - The minimum and maximum, or zeroes for an empty slice.
fn extrema(values: &[f64]) -> (f64, f64) {
    let Some(first) = values.first() else {
        return (0.0, 0.0);
    };
    let mut minimum: f64 = *first;
    let mut maximum: f64 = *first;
    for value in values.iter().copied() {
        minimum = minimum.min(value);
        maximum = maximum.max(value);
    }
    (minimum, maximum)
}

/// Formats one variant's absolute measurements.
///
/// # Arguments
///
/// - `&str` - The variant label.
/// - `&Repeats` - The variant results keyed by repeat.
///
/// # Returns
///
/// - `String` - The formatted report line.
fn describe(label: &str, results: &Repeats) -> String {
    let mut rps: Vec<f64> = Vec::new();
    let mut p50: Vec<f64> = Vec::new();
    let mut p99: Vec<f64> = Vec::new();
    let mut errors: u64 = 0;
    for result in results.values() {
        rps.push(result.rps);
        p50.push(result.p50_us as f64);
        p99.push(result.p99_us as f64);
        errors += result.errors;
    }
    let (worst, best) = extrema(&rps);
    format!(
        "  {label:>10}  rps med={:>10.1}  best={best:>10.1}  worst={worst:>10.1}  p50={:>7.0}us  p99={:>8.0}us  errors={errors}\n",
        median(&rps),
        median(&p50),
        median(&p99),
    )
}

/// Computes the verdict for one paired workload cell.
///
/// # Arguments
///
/// - `&Repeats` - The baseline results keyed by repeat.
/// - `&Repeats` - The candidate results keyed by repeat.
/// - `&Repeats` - The control results keyed by repeat.
///
/// # Returns
///
/// - `(&'static str, String)` - The verdict and its reason.
fn verdict_for(
    baseline: &Repeats,
    candidate: &Repeats,
    control: &Repeats,
) -> (&'static str, String) {
    let mut deltas: Vec<f64> = Vec::new();
    let mut shared: usize = 0;
    for (repeat, base) in baseline {
        if let Some(cand) = candidate.get(repeat) {
            shared += 1;
            if base.rps > 0.0 {
                deltas.push((cand.rps - base.rps) / base.rps * 100.0);
            }
        }
    }
    if shared == 0 {
        return (INCONCLUSIVE_VERDICT, NOT_PAIRED_REASON.to_owned());
    }
    if deltas.is_empty() {
        return (INCONCLUSIVE_VERDICT, NO_USABLE_PAIR_REASON.to_owned());
    }
    let median_delta: f64 = median(&deltas);
    let (minimum_delta, maximum_delta) = extrema(&deltas);
    let spread: f64 = maximum_delta - minimum_delta;
    let detail: String =
        format!("median paired difference {median_delta:+.2}%, spread {spread:.1} points");
    if control.len() > 1 {
        let control_rps: Vec<f64> = control
            .values()
            .map(|result: &MeasurementResult| result.rps)
            .collect();
        let (minimum_control, maximum_control) = extrema(&control_rps);
        if maximum_control > 0.0 {
            let control_spread: f64 = (maximum_control - minimum_control) / maximum_control * 100.0;
            if control_spread > CONTROL_TOLERANCE {
                return (
                    INCONCLUSIVE_VERDICT,
                    format!("the control moved {control_spread:.1}%, the machine changed"),
                );
            }
        }
    }
    if median_delta.abs() <= spread {
        return (WITHIN_NOISE_VERDICT, detail);
    }
    if median_delta >= IMPROVEMENT_THRESHOLD {
        return (IMPROVEMENT_VERDICT, detail);
    }
    if median_delta <= REGRESSION_THRESHOLD {
        return (REGRESSION_VERDICT, detail);
    }
    (WITHIN_NOISE_VERDICT, detail)
}

/// Renders a raw JSON Lines measurement file as a performance report.
///
/// # Arguments
///
/// - `&Path` - The raw measurement file.
///
/// # Returns
///
/// - `Result<String, String>` - The complete report, or a broken measurement error.
pub fn render_report(path: &Path) -> Result<String, String> {
    let rows: Vec<MeasurementRow> = load_rows(path)?;
    if rows.is_empty() {
        return Err(NO_MEASUREMENTS_ERROR.to_owned());
    }
    let grouped: Cells = group_rows(rows);
    let empty: Repeats = BTreeMap::new();
    let mut report: String = String::new();
    report.push_str(REPORT_HEADER);
    for (cell, variants) in grouped {
        report.push_str(&format!(
            "\n{} / {} / concurrency {}\n",
            cell.protocol, cell.mode, cell.connections
        ));
        for variant in [BASELINE_VARIANT, CANDIDATE_VARIANT, CONTROL_VARIANT] {
            if let Some(results) = variants.get(variant)
                && !results.is_empty()
            {
                report.push_str(&describe(variant, results));
            }
        }
        let baseline: &Repeats = variants.get(BASELINE_VARIANT).unwrap_or(&empty);
        let candidate: &Repeats = variants.get(CANDIDATE_VARIANT).unwrap_or(&empty);
        let control: &Repeats = variants.get(CONTROL_VARIANT).unwrap_or(&empty);
        if baseline.is_empty() || candidate.is_empty() {
            report.push_str(&format!("  incomplete: {NOT_PAIRED_REASON}\n"));
            continue;
        }
        let (verdict, reason) = verdict_for(baseline, candidate, control);
        report.push_str(&format!("  => {verdict}: {reason}\n"));
    }
    report.push_str(REPORT_FOOTER);
    Ok(report)
}
