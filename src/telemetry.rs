use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

pub fn init_tracing() {
    let mut fmt_layer = fmt::layer();
    if std::env::var("INCLUDE_SPAN_EVENTS").is_ok_and(|value| value.eq_ignore_ascii_case("true")) {
        fmt_layer = fmt_layer.with_span_events(FmtSpan::ENTER | FmtSpan::EXIT);
    }
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}
