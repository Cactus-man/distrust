extern crate tokio;
extern crate websockets;

use crate::error::GatewayError;
use crate::payloads::GatewayPayload;
use crate::{payloads::DispatchType, payloads::PayloadData};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time;
use websockets::{Frame, WebSocket};

pub struct Shard<'l> {
    url: String,
    serial: Option<u32>,
    listeners: HashMap<DispatchType, &'l dyn Fn(GatewayPayload) -> ()>,
}

impl<'l> Shard<'l> {
    pub fn new(url: &str) -> Shard {
        Shard {
            url: String::from(url),
            serial: None,
            listeners: HashMap::new(),
        }
    }

    pub fn on_event<F>(&mut self, t: DispatchType, listener: &'l F)
    where
        F: Fn(GatewayPayload) -> (),
    {
        self.listeners.insert(t, listener);
    }

    pub async fn listen<'a>(&'a mut self) -> Result<(), GatewayError> {
        let mut conn = WebSocket::builder();
        // configure websocket
        let mut conn = conn.connect(&self.url).await?;

        if let Frame::Text {
            payload,
            continuation: _,
            fin: _,
        } = conn.receive().await?
        {
            let payload: GatewayPayload = serde_json::from_str(&payload)?;
            if let PayloadData::Hello { heartbeat_interval } = payload.d {
                let mut ticker = time::interval(Duration::from_millis(heartbeat_interval));

                self.heartbeat().await;
                self.identify().await;

                'action: loop {
                    let action = self.next(&mut ticker, &mut conn).await?;
                    match action {
                        ShardAction::Disconnect => {
                            return Ok(());
                        }
                        ShardAction::Dispatch(_) => {
                            todo!()
                        }
                        ShardAction::Heartbeat => self.heartbeat().await,
                        ShardAction::None => continue,
                        ShardAction::Resume => {
                            for _ in 0..3 {
                                if let Ok(_) = self.resume(&mut conn).await {
                                    continue 'action;
                                }
                            }
                        }
                    };
                }
            } else {
                Err(GatewayError::InitializationError)
            }
        } else {
            Err(GatewayError::InitializationError)
        }
    }

    async fn identify(&mut self) {
        // TODO (gateway identification) send identify payload
    }

    async fn heartbeat(&mut self) {
        // TODO (send single beat) send single heartbeat payload via websocket
    }

    async fn resume(&mut self, conn: &mut WebSocket) -> Result<(), ()> {
        if let Some(serial) = self.serial {
            // TODO (try resuming)
            println!("resuming from last serial, {}", serial);
            Ok(())
        } else {
            Err(())
        }
    }

    async fn next(
        &mut self,
        ticker: &mut time::Interval,
        conn: &mut WebSocket,
    ) -> Result<ShardAction, GatewayError> {
        tokio::select! {
            _ = ticker.tick() => { Result::<ShardAction, GatewayError>::Ok(ShardAction::Heartbeat) },
            frame = conn.receive() => {
                match frame? {
                    Frame::Text { payload, continuation: _, fin: _ } => {
                        Ok(ShardAction::Dispatch(serde_json::from_str::<GatewayPayload>(&payload)?))
                    },
                    Frame::Close { payload } => {
                        match payload {
                            Some((code, _)) if (code == 4004 || 4009 < code && code < 4015) => {
                                Ok(ShardAction::Disconnect)
                            },
                            Some(_) => Ok(ShardAction::Resume),
                            _ => Ok(ShardAction::Disconnect)
                        }
                    },
                    _ => Ok(ShardAction::None),
                }
            }
        }
    }
}

impl std::fmt::Debug for Shard<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Shard")
            .field("url", &self.url)
            .field("serial", &self.serial)
            .finish()
    }
}

enum ShardAction {
    Disconnect,
    Dispatch(GatewayPayload),
    Heartbeat,
    None,
    Resume,
}
