use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use rusqlite::Connection;
use tokio::sync::Notify;

use crate::ai::embedding::{resolve_embedding_config, EmbeddingConfig};
use crate::db;

pub struct PipelineSchedulerState {
    interval_min: Arc<AtomicU64>,
    notify: Arc<Notify>,
    next_run_ms: Arc<AtomicU64>,
    running: Arc<AtomicBool>,
}

impl PipelineSchedulerState {
    pub fn new(interval_min: u64) -> Self {
        Self {
            interval_min: Arc::new(AtomicU64::new(interval_min)),
            notify: Arc::new(Notify::new()),
            next_run_ms: Arc::new(AtomicU64::new(0)),
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn update_interval(&self, minutes: u64) {
        self.interval_min.store(minutes, Ordering::SeqCst);
        self.notify.notify_one();
    }

    pub async fn trigger_now(
        &self,
        conn: &Arc<Mutex<Connection>>,
        config: &EmbeddingConfig,
    ) -> Result<(), String> {
        if self.running.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            return Err("Pipeline is already running".to_string());
        }
        let result = run_pipeline(conn, config).await;
        self.running.store(false, Ordering::SeqCst);
        result
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    pub fn next_run(&self) -> Option<i64> {
        let val = self.next_run_ms.load(Ordering::SeqCst);
        if val == 0 {
            None
        } else {
            Some(val as i64)
        }
    }

    pub fn spawn(&self, conn: Arc<Mutex<Connection>>) {
        let interval_min = self.interval_min.clone();
        let notify = self.notify.clone();
        let next_run_ms = self.next_run_ms.clone();
        let running = self.running.clone();

        tauri::async_runtime::spawn(async move {
            let mut run_count = 0u64;

            loop {
                let minutes = interval_min.load(Ordering::SeqCst);
                let duration = std::time::Duration::from_secs(minutes * 60);
                let now_ms = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;
                next_run_ms.store(now_ms + (minutes * 60 * 1000), Ordering::SeqCst);

                tokio::select! {
                    _ = tokio::time::sleep(duration) => {}
                    _ = notify.notified() => {
                        continue;
                    }
                }

                // Resolve embedding config
                let config = {
                    let c = match conn.lock() {
                        Ok(c) => c,
                        Err(e) => {
                            log::error!("Pipeline: failed to lock DB: {}", e);
                            continue;
                        }
                    };
                    match resolve_embedding_config(&c) {
                        Ok(config) => config,
                        Err(e) => {
                            log::info!("Pipeline: no API key configured, skipping: {}", e);
                            continue;
                        }
                    }
                };

                // Run pipeline (skip if manual trigger is in progress)
                if running.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
                    log::info!("Pipeline: skipping scheduled run (already running)");
                    continue;
                }
                if let Err(e) = run_pipeline(&conn, &config).await {
                    log::error!("Pipeline run failed: {}", e);
                }
                running.store(false, Ordering::SeqCst);

                // Record last run time
                {
                    if let Ok(c) = conn.lock() {
                        let now = db::now_ms();
                        let _ = db::settings::set_setting(&c, "pipeline_last_run", &now.to_string());
                    }
                }

                run_count += 1;

                // Backfill every 5 runs
                if run_count % 5 == 0 {
                    if let Err(e) =
                        crate::pipeline::backfill::backfill_isolated_nodes(&conn, &config).await
                    {
                        log::error!("Backfill failed: {}", e);
                    }
                }
            }
        });
    }
}

async fn run_pipeline(
    conn: &Arc<Mutex<Connection>>,
    config: &EmbeddingConfig,
) -> Result<(), String> {
    log::info!("Pipeline: starting run");

    let distilled = crate::pipeline::distill::distill_mashes(conn, config).await?;
    log::info!("Pipeline: distilled {} mashes", distilled);

    let jarred = crate::pipeline::jar::jar_mashes(conn, config).await?;
    log::info!("Pipeline: jarred {} mashes", jarred);

    Ok(())
}
