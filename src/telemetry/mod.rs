use tracing_subscriber::{
    EnvFilter, Registry, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt,
};

pub fn init() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .flatten_event(true)
        .with_target(true)
        .with_span_events(FmtSpan::CLOSE);
    Registry::default().with(filter).with(fmt_layer).init();
}
