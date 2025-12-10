use std::time::Duration;

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use store::StoreRef;
use tokio::{
    sync::mpsc::{Receiver, Sender},
    task::JoinHandle,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncCommand {
    ForceSync,
    SyncItem { id: i64 },
    ScheduleOne { delay: Duration },
    Stop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncEvent {
    Progress { percent: u8 },
    ItemUpdated { id: i64 },
    Completed,
    Error { message: String },
}

#[derive(Clone)]
pub struct SyncHandle {
    pub cmd_tx: Sender<SyncCommand>,
}

impl SyncHandle {
    pub async fn force_sync(&self) -> Result<()> {
        self.cmd_tx
            .send(SyncCommand::ForceSync)
            .await
            .map_err(|e| anyhow!(e.to_string()))
    }
}

pub struct SyncService {
    store: StoreRef,
}

impl SyncService {
    pub fn new(store: StoreRef) -> Self {
        Self { store }
    }

    pub fn start(&self) -> (SyncHandle, JoinHandle<Result<()>>, Receiver<SyncEvent>) {
        let (cmd_tx, mut cmd_rx) = tokio::sync::mpsc::channel::<SyncCommand>(32);
        let (ev_tx, ev_rx) = tokio::sync::mpsc::channel::<SyncEvent>(64);

        let store = self.store.clone();

        let join = tokio::spawn(async move {
            tracing::info!("svc_sync: worker started");
            let mut ticker = tokio::time::interval(Duration::from_secs(60));

            loop {
                tokio::select! {
                    _ = ticker.tick() => {
                        if let Err(e) = do_sync_cycle(&*store, &ev_tx).await {
                            tracing::error!("periodic sync failed: {}", e.to_string());
                            let _ = ev_tx.send(SyncEvent::Error { message: e.to_string() }).await;
                        }
                    }

                    maybe_cmd = cmd_rx.recv() => {
                        match maybe_cmd {
                            Some(cmd) => match cmd {
                                SyncCommand::ForceSync => {
                                    tracing::info!("svc_sync: ForceSync received");
                                    if let Err(e) = do_sync_cycle(&*store, &ev_tx).await {
                                        let _ = ev_tx.send(SyncEvent::Error { message: e.to_string() }).await;
                                    } else {
                                        let _ = ev_tx.send(SyncEvent::Completed).await;
                                    }
                                }
                                SyncCommand::SyncItem {id } => {
                                    tracing::info!("svc_sync: SyncItem {id}");
                                    if let Err(e) = store.save_item(id, "synced").await {
                                        let _ = ev_tx.send(SyncEvent::Error { message: e.to_string() }).await;
                                    } else {
                                        let _ = ev_tx.send(SyncEvent::ItemUpdated { id }).await;
                                    }
                                }
                                SyncCommand::ScheduleOne { delay } => {
                                    tracing::info!("svc_sync: ScheduleOne (dela: {:?})", delay);
                                    let ev_tx_clone = ev_tx.clone();
                                    let store_clone = store.clone();
                                    tokio::spawn(async move {
                                        tokio::time::sleep(delay).await;
                                        if let Err(e) = do_sync_cycle(&*store_clone, &ev_tx_clone).await {
                                            let _ = ev_tx_clone.send(SyncEvent::Error { message: e.to_string() }).await;
                                        } else {
                                            let _ = ev_tx_clone.send(SyncEvent::Completed).await;
                                        }
                                    });
                                }
                                SyncCommand::Stop => {
                                    tracing::info!("svc_sync: Stop requested, exiting worker loop");
                                    break;
                                }
                            },
                            None => {
                                tracing::info!("svc_sync: command channel closed, shutting down");
                                break;
                            }
                        }
                    }
                }
            }

            tracing::info!("svc_sync: worker exiting");
            Ok(())
        });

        (SyncHandle { cmd_tx }, join, ev_rx)
    }
}

async fn do_sync_cycle(
    store: &dyn store::Store,
    ev_tx: &tokio::sync::mpsc::Sender<SyncEvent>,
) -> Result<()> {
    let pending = store.load_pending_changes().await?;
    let total = pending.len().max(1) as u64;
    for (i, id) in pending.into_iter().enumerate() {
        store.save_item(id, "synced").await?;
        let percent = (((i as u64 + 1) * 100) / total) as u8;
        let _ = ev_tx.send(SyncEvent::Progress { percent }).await;
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    Ok(())
}
