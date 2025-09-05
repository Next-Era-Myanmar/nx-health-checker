use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::Duration;
use tokio::{task::JoinHandle, time::sleep};
use sqlx::SqlitePool;
use crate::models::Service;
use once_cell::sync::Lazy;

pub static GLOBAL_COLLECTOR: Lazy<CollectorManager> = Lazy::new(|| CollectorManager::new());
use crate::metrics::{SERVICE_STATUS, SERVICE_LATENCY_SECONDS, SERVICE_CHECKS_TOTAL};

pub struct CollectorManager {
    running_flag: Arc<AtomicBool>,
    handles: parking_lot::Mutex<Vec<JoinHandle<()>>>,
}

impl CollectorManager {
    pub fn new() -> Self {
        Self {
            running_flag: Arc::new(AtomicBool::new(false)),
            handles: parking_lot::Mutex::new(Vec::new()),
        }
    }

    pub async fn start(&self, pool: SqlitePool) {
        self.stop().await;
        self.running_flag.store(true, Ordering::SeqCst);

        // Load services
        let services: Vec<Service> = match sqlx::query_as::<_, Service>("SELECT * FROM services")
            .fetch_all(&pool)
            .await
        {
            Ok(s) => s,
            Err(_) => Vec::new(),
        };

        let mut new_handles = Vec::new();
        for service in services {
            let pool_clone = pool.clone();
            let running = self.running_flag.clone();
            let handle = tokio::spawn(async move {
                let interval_secs = if service.healthcheck_duration_seconds <= 0 { 30 } else { service.healthcheck_duration_seconds } as u64;
                loop {
                    if !running.load(Ordering::SeqCst) { break; }
                    let start = std::time::Instant::now();
                    let status = match perform_check(&service.healthcheck_url).await {
                        Ok(up) => if up { "UP" } else { "DOWN" },
                        Err(_) => "DOWN",
                    };
                    let elapsed = start.elapsed().as_secs_f64();
                    let url_label = service.healthcheck_url.clone();
                    let name_label = service.service_name.clone();
                    let labels = [&url_label[..], &name_label[..]];
                    SERVICE_LATENCY_SECONDS.with_label_values(&labels).set(elapsed);
                    SERVICE_STATUS.with_label_values(&labels).set(if status == "UP" { 1.0 } else { 0.0 });
                    SERVICE_CHECKS_TOTAL.with_label_values(&[labels[0], labels[1], status]).inc();

                    // Sleep for configured duration or until stop
                    let mut slept = 0u64;
                    while slept < interval_secs {
                        if !running.load(Ordering::SeqCst) { break; }
                        sleep(Duration::from_secs(1)).await;
                        slept += 1;
                    }
                    if !running.load(Ordering::SeqCst) { break; }
                }
                let _ = pool_clone; // keep pool in scope if needed later
            });
            new_handles.push(handle);
        }

        *self.handles.lock() = new_handles;
    }

    pub async fn stop(&self) {
        self.running_flag.store(false, Ordering::SeqCst);
        let mut handles = Vec::new();
        std::mem::swap(&mut *self.handles.lock(), &mut handles);
        for h in handles { let _ = h.abort(); }
    }

    pub async fn restart(&self, pool: SqlitePool) {
        self.start(pool).await;
    }
}

async fn perform_check(url: &str) -> Result<bool, reqwest::Error> {
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .timeout(Duration::from_secs(10))
        .send()
        .await?;
    Ok(resp.status().is_success())
}


