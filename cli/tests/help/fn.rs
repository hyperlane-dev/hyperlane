use super::*;

#[test]
fn test_print_help_runs_without_logger() {
    print_help();
}

#[test]
fn test_print_help_is_idempotent() {
    print_help();
    print_help();
}
