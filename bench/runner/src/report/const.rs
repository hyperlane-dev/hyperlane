/// The improvement threshold in percent.
pub(crate) const IMPROVEMENT_THRESHOLD: f64 = 5.0;
/// The regression threshold in percent.
pub(crate) const REGRESSION_THRESHOLD: f64 = -10.0;
/// The maximum stable control spread in percent.
pub(crate) const CONTROL_TOLERANCE: f64 = 10.0;
/// The baseline variant name.
pub(crate) const BASELINE_VARIANT: &str = "baseline";
/// The candidate variant name.
pub(crate) const CANDIDATE_VARIANT: &str = "candidate";
/// The control variant name.
pub(crate) const CONTROL_VARIANT: &str = "control";
/// The improvement verdict.
pub(crate) const IMPROVEMENT_VERDICT: &str = "improvement";
/// The regression verdict.
pub(crate) const REGRESSION_VERDICT: &str = "regression";
/// The inconclusive verdict.
pub(crate) const INCONCLUSIVE_VERDICT: &str = "inconclusive";
/// The within-noise verdict.
pub(crate) const WITHIN_NOISE_VERDICT: &str = "within noise";
/// The report heading.
pub(crate) const REPORT_HEADER: &str =
    "hyperlane performance report\n===========================\n";
/// The report footer.
pub(crate) const REPORT_FOOTER: &str = "\nThe verdict is a report. It never blocks a build.\n";
/// The error returned when no rows exist.
pub(crate) const NO_MEASUREMENTS_ERROR: &str = "no measurements were found";
/// The reason two variants were not paired.
pub(crate) const NOT_PAIRED_REASON: &str = "the two variants were not measured together";
/// The reason no usable pair exists.
pub(crate) const NO_USABLE_PAIR_REASON: &str = "no repeat produced a usable pair";
