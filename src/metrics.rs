use once_cell::sync::Lazy;
use prometheus::{register_counter_vec, register_gauge_vec, Encoder, GaugeVec, CounterVec, TextEncoder};

pub static SERVICE_STATUS: Lazy<GaugeVec> = Lazy::new(|| {
    register_gauge_vec!(
        "service_status",
        "Service health status (1=UP, 0=DOWN)",
        &["service_health_checkurl", "service_name"]
    ).expect("register service_status")
});

pub static SERVICE_LATENCY_SECONDS: Lazy<GaugeVec> = Lazy::new(|| {
    register_gauge_vec!(
        "service_latency_seconds",
        "Last health check latency in seconds",
        &["service_health_checkurl", "service_name"]
    ).expect("register service_latency_seconds")
});

pub static SERVICE_CHECKS_TOTAL: Lazy<CounterVec> = Lazy::new(|| {
    register_counter_vec!(
        "service_checks_total",
        "Total number of health checks",
        &["service_health_checkurl", "service_name", "result"]
    ).expect("register service_checks_total")
});

pub fn gather_metrics() -> Vec<u8> {
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    encoder.encode(&metric_families, &mut buffer).ok();
    buffer
}


