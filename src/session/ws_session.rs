use std::time::Instant;

pub struct WsSession {
  pub id: usize,
  /// Client must send ping at least once per 10 seconds, otherwise we drop
  /// connection.
  pub _hb: Instant,
  pub room: String,
  pub name: String,
}