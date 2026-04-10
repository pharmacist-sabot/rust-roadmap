//! localStorage wrapper for progress persistence.

use crate::models::roadmap::NodeStatus;
use std::collections::HashMap;

const STORAGE_KEY: &str = "rust_roadmap_progress";

/// Persist the progress map to `localStorage`.
/// Format: pipe-separated `"id:status"` pairs, e.g. `"intro:d|setup_env:p"`.
pub fn save_progress(progress: &HashMap<String, NodeStatus>) {
  let window = match web_sys::window() {
    Some(w) => w,
    None => return,
  };

  let storage = match window.local_storage() {
    Ok(Some(s)) => s,
    _ => return,
  };

  let serialized: String = progress
    .iter()
    .map(|(id, &status)| format!("{}:{}", id, status_to_str(status)))
    .collect::<Vec<_>>()
    .join("|");

  let _ = storage.set_item(STORAGE_KEY, &serialized);
}

/// Load the progress map from `localStorage`.
/// Returns an empty map if nothing is stored or parsing fails.
pub fn load_progress() -> HashMap<String, NodeStatus> {
  let mut map = HashMap::new();

  let window = match web_sys::window() {
    Some(w) => w,
    None => return map,
  };

  let storage = match window.local_storage() {
    Ok(Some(s)) => s,
    _ => return map,
  };

  let raw = match storage.get_item(STORAGE_KEY) {
    Ok(Some(s)) if !s.is_empty() => s,
    _ => return map,
  };

  for pair in raw.split('|') {
    let mut parts = pair.splitn(2, ':');
    if let (Some(id), Some(status_str)) = (parts.next(), parts.next())
      && let Some(status) = str_to_status(status_str)
    {
      map.insert(id.to_string(), status);
    }
  }

  map
}

fn status_to_str(status: NodeStatus) -> &'static str {
  match status {
    NodeStatus::Untouched => "u",
    NodeStatus::InProgress => "p",
    NodeStatus::Done => "d",
    NodeStatus::Skipped => "s",
  }
}

fn str_to_status(s: &str) -> Option<NodeStatus> {
  match s {
    // Short form (current)
    "u" => Some(NodeStatus::Untouched),
    "p" => Some(NodeStatus::InProgress),
    "d" => Some(NodeStatus::Done),
    "s" => Some(NodeStatus::Skipped),
    // Long form (legacy / forward compat)
    "untouched" => Some(NodeStatus::Untouched),
    "in_progress" => Some(NodeStatus::InProgress),
    "done" => Some(NodeStatus::Done),
    "skipped" => Some(NodeStatus::Skipped),
    _ => None,
  }
}
