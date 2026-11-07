pub(crate) const TARGET_ROUTE_PATH: &str = "/target";
pub(crate) const TARGET_ECHO_HEADER: &str = "x-request-target";
pub(crate) const TARGET_SPOOF_HEADER: &str = "hyperlane.request.target";
pub(crate) const TARGET_SPOOF_VALUE: &str = "/untrusted?fake=1";
pub(crate) const TARGET_REWRITTEN_PATH: &str = "/rewritten";
pub(crate) const TRAILING_QUESTION_TARGET: &str = "/target?";

pub(crate) const TARGET_CASES: [(&str, &str); 8] = [
    ("duplicate keys", "/target?a=1&b=2&a=3"),
    ("bare flag", "/target?flag"),
    ("empty value", "/target?k="),
    ("plus sign", "/target?q=a+b"),
    ("encoded space", "/target?q=a%20b"),
    ("nested percent", "/target?q=%2520"),
    ("encoded unicode", "/target?q=%E4%BD%A0%E5%A5%BD"),
    (
        "mixed query",
        "/target?x=1&x&y=&z=a+b&w=%20&v=%2520&u=%E4%BD%A0%E5%A5%BD",
    ),
];
