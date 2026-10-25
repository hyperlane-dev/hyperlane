use super::*;

fn temporary_directory(name: &str) -> PathBuf {
    let nonce: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos(),
        Err(error) => panic!("{error}"),
    };
    let directory: PathBuf =
        env::temp_dir().join(format!("hyperlane-performance-{name}-{}-{nonce}", id()));
    match fs::create_dir_all(&directory) {
        Ok(()) => directory,
        Err(error) => panic!("{error}"),
    }
}

fn row(variant: &str, repeat: u32, rps: f64) -> String {
    format!(
        "{{\"variant\":\"{variant}\",\"protocol\":\"h1\",\"mode\":\"keepalive\",\"connections\":64,\"repeat\":{repeat},\"result\":{{\"errors\":0,\"rps\":{rps},\"p50_us\":100,\"p99_us\":200}}}}"
    )
}

fn report_for(name: &str, rows: &[String]) -> Result<String, String> {
    let directory: PathBuf = temporary_directory(name);
    let path: PathBuf = directory.join("raw.jsonl");
    let contents: String = format!("{}\n", rows.join("\n"));
    match fs::write(&path, contents) {
        Ok(()) => {}
        Err(error) => return Err(format!("{error}")),
    }
    let result: Result<String, String> = render_report(&path);
    match fs::remove_dir_all(&directory) {
        Ok(()) => result,
        Err(error) => Err(format!("{error}")),
    }
}

#[test]
fn reports_an_improvement() {
    let report: String = match report_for(
        "improvement",
        &[
            row("baseline", 1, 100.0),
            row("candidate", 1, 110.0),
            row("baseline", 2, 100.0),
            row("candidate", 2, 111.0),
            row("baseline", 3, 100.0),
            row("candidate", 3, 112.0),
        ],
    ) {
        Ok(report) => report,
        Err(error) => panic!("{error}"),
    };
    assert!(report.contains("=> improvement: median paired difference +11.00%"));
}

#[test]
fn reports_a_regression() {
    let report: String = match report_for(
        "regression",
        &[
            row("baseline", 1, 100.0),
            row("candidate", 1, 80.0),
            row("baseline", 2, 100.0),
            row("candidate", 2, 79.0),
            row("baseline", 3, 100.0),
            row("candidate", 3, 78.0),
        ],
    ) {
        Ok(report) => report,
        Err(error) => panic!("{error}"),
    };
    assert!(report.contains("=> regression: median paired difference -21.00%"));
}

#[test]
fn reports_changes_within_noise() {
    let report: String = match report_for(
        "noise",
        &[
            row("baseline", 1, 100.0),
            row("candidate", 1, 106.0),
            row("baseline", 2, 100.0),
            row("candidate", 2, 100.0),
            row("baseline", 3, 100.0),
            row("candidate", 3, 94.0),
        ],
    ) {
        Ok(report) => report,
        Err(error) => panic!("{error}"),
    };
    assert!(report.contains("=> within noise: median paired difference +0.00%"));
}

#[test]
fn reports_an_inconclusive_moving_control() {
    let report: String = match report_for(
        "control",
        &[
            row("baseline", 1, 100.0),
            row("candidate", 1, 120.0),
            row("control", 1, 100.0),
            row("baseline", 2, 100.0),
            row("candidate", 2, 120.0),
            row("control", 2, 200.0),
            row("baseline", 3, 100.0),
            row("candidate", 3, 120.0),
            row("control", 3, 300.0),
        ],
    ) {
        Ok(report) => report,
        Err(error) => panic!("{error}"),
    };
    assert!(report.contains("=> inconclusive: the control moved 66.7%"));
}

#[test]
fn reports_an_incomplete_cell() {
    let report: String = match report_for("incomplete", &[row("baseline", 1, 100.0)]) {
        Ok(report) => report,
        Err(error) => panic!("{error}"),
    };
    assert!(report.contains("incomplete: the two variants were not measured together"));
}

#[test]
fn rejects_missing_measurements() {
    let directory: PathBuf = temporary_directory("missing");
    let path: PathBuf = directory.join("absent.jsonl");
    let result: Result<String, String> = render_report(&path);
    match fs::remove_dir_all(&directory) {
        Ok(()) => {}
        Err(error) => panic!("{error}"),
    }
    assert_eq!(result, Err("no measurements were found".to_owned()));
}

#[test]
fn rejects_a_broken_measurement() {
    let result: Result<String, String> = report_for("broken", &["not json".to_owned()]);
    assert!(result.is_err());
}
