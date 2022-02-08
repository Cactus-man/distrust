use super::{GatewayDispatch, GatewayPayload, PayloadData};

use crate::{error::Result, DiscordError};

use std::{collections::HashMap, time::Duration};
use tokio::time::{interval, MissedTickBehavior};
use tokio::join;
use websockets::{Frame, WebSocket, WebSocketReadHalf, WebSocketWriteHalf};

// TODO (implement rate limit) queue gateway commands which surpass 120 per minute and warn
// TODO (centalize listeners) move listeners to a location shared by shards
// IDEA (filtering listeners) listening on a specific server
pub struct Shard<'a> {
    url: &'a str,
    serial: Option<u64>,
    conn: Option<websockets::WebSocket>, // ? exchange for &'a mut ...
    _listeners: HashMap<GatewayDispatch, &'a dyn Fn(GatewayPayload)>,
}

// FIX (rate limits) can't identify more than `max_concurrency` shards per 5 seconds

impl<'a> Shard<'a> {
    pub fn new(url: &'a str) -> Shard {
        Shard {
            url,
            serial: None,
            conn: None,
            _listeners: HashMap::new(),
        }
    }

    pub async fn connect(&self) -> Result<()> {
        let mut ws = WebSocket::connect(self.url).await?;
        if let Frame::Text { payload, .. } = ws.receive().await? {
            // .. = continuation: bool, fin: bool
            let payload: GatewayPayload = serde_json::from_str(&payload)?;
            if let PayloadData::Hello { heartbeat_interval } = payload.d {
                let (mut read, mut write) = ws.split();
                // FIX identify payload is required
                loop {
                    join!(
                        self.listen(&mut read),
                        self.heartbeat(&mut write, heartbeat_interval)
                    );
                }
            };
        };
        Err(DiscordError::InternalError)
    }

    async fn identify(&self) {
        // TODO Allow library users to configure identify
    }

    async fn listen(&self, ws: &mut WebSocketReadHalf) -> Result<()> {
        loop {
            match ws.receive().await? {
                Frame::Text {
                    payload,
                    continuation: _,
                    fin: _,
                } => {
                    println!("Just imagine it was handled!");
                }
                Frame::Close { payload } => {
                    if let Some(payload) = payload {
                        println!("Gateway closed with code {}: {}", payload.0, payload.1);
                    };
                    return Ok(());
                }
                _ => (),
            };
        }
    }

    async fn heartbeat(&self, ws: &mut WebSocketWriteHalf, period: u64) -> Result<()> {
        // TODO (allow close) add capability of closing heartbeat loop
        // FIX (handle received op 1) Reply with heartbeat immediatly upon request
        let mut timer = interval(Duration::from_millis(period));
        timer.set_missed_tick_behavior(MissedTickBehavior::Delay);
        loop {
            timer.tick().await;
            let d = serde_json::to_string(&self.serial).unwrap();
            ws.send_text(format!("{{\"op\":1,\"d\":{}}}", d)).await?;
        }
    }
}
