use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct GeoPoint {
    lat: f64,
    lon: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PassWindow {
    start_ts: u64,
    end_ts: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct OfflineRecord {
    ts: u64,
    bytes_b64_hint: String, // keep simple for demo logs
    value_units: u64,
}

struct TerminalNode {
    location: GeoPoint,
    orbit_period_sec: u64, // simplified educational model
    pass_duration_sec: u64,
    queue: Vec<OfflineRecord>,
    max_queue_units: u64,
    accumulated_units: u64,
    hard_locked: bool,
}

impl TerminalNode {
    fn new(lat: f64, lon: f64) -> Self {
        Self {
            location: GeoPoint { lat, lon },
            orbit_period_sec: 90 * 60, // \~90 min LEO-style period (simplified)
            pass_duration_sec: 8 * 60, // \~8 min pass window (simplified)
            queue: Vec::new(),
            max_queue_units: 10_000,
            accumulated_units: 0,
            hard_locked: false,
        }
    }

    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Simplified deterministic pass estimator.
    /// Real systems use TLE/SGP4. This is a local offline approximation for terminal logic.
    fn current_pass(&self, now_ts: u64) -> Option<PassWindow> {
        let cycle = now_ts % self.orbit_period_sec;
        if cycle < self.pass_duration_sec {
            let start = now_ts - cycle;
            Some(PassWindow {
                start_ts: start,
                end_ts: start + self.pass_duration_sec,
            })
        } else {
            None
        }
    }

    fn enqueue(&mut self, value_units: u64, note: &str) -> Result<(), String> {
        if self.hard_locked {
            return Err("hard-locked".into());
        }
        if self.accumulated_units + value_units > self.max_queue_units {
            self.hard_locked = true;
            return Err("tier/cap breached -> hard lock".into());
        }
        let rec = OfflineRecord {
            ts: Self::now(),
            bytes_b64_hint: note.into(),
            value_units,
        };
        self.accumulated_units += value_units;
        self.queue.push(rec);
        Ok(())
    }

    fn burst_sync_if_pass(&mut self) -> String {
        let now = Self::now();
        match self.current_pass(now) {
            None => format!(
                "NO PASS @ lat={}, lon={} | queued={}, units={}",
                self.location.lat,
                self.location.lon,
                self.queue.len(),
                self.accumulated_units
            ),
            Some(pass) => {
                let n = self.queue.len();
                // In production: open encrypted backhaul, send queue, await ack, then clear.
                self.persist_burst_log(n, &pass);
                self.queue.clear();
                // keep accumulated_units until settlement policy says otherwise
                format!(
                    "PASS WINDOW {}-{} | burst sent records={} | lat={} lon={}",
                    pass.start_ts,
                    pass.end_ts,
                    n,
                    self.location.lat,
                    self.location.lon
                )
            }
        }
    }

    fn persist_burst_log(&self, count: usize, pass: &PassWindow) {
        if let Ok(mut f) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("satellite_burst_log.txt")
        {
            let _ = writeln!(
                f,
                "pass={}-{} records={} units={} loc={},{}",
                pass.start_ts,
                pass.end_ts,
                count,
                self.accumulated_units,
                self.location.lat,
                self.location.lon
            );
        }
    }
}

fn main() {
    // Example: Makhonyeni baseline, but coordinates are inputs (usable worldwide)
    let mut node = TerminalNode::new(-27.42, 32.18);

    let _ = node.enqueue(150, "offline-pay-1");
    let _ = node.enqueue(90, "offline-pay-2");

    println!("{}", node.burst_sync_if_pass());
    println!("queued_after={}", node.queue.len());
          }
