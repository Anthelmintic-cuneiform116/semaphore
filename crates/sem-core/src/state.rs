use std::collections::HashMap;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LightState {
    Green = 0,
    Yellow = 1,
    Red = 2,
}

impl LightState {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "green" => Some(Self::Green),
            "yellow" => Some(Self::Yellow),
            "red" => Some(Self::Red),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct SessionEntry {
    state: LightState,
    source: String,
    reason: String,
    updated_at: Instant,
}

#[derive(Debug, Clone)]
pub struct StateMachine {
    sessions: HashMap<String, SessionEntry>,
    idle_timeout: Duration,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new(Duration::from_secs(300))
    }
}

impl StateMachine {
    pub fn new(idle_timeout: Duration) -> Self {
        Self {
            sessions: HashMap::new(),
            idle_timeout,
        }
    }

    pub fn set_session(
        &mut self,
        session_id: &str,
        state: LightState,
        source: &str,
        reason: &str,
    ) -> LightState {
        if state == LightState::Green {
            self.sessions.remove(session_id);
        } else {
            self.sessions.insert(
                session_id.to_string(),
                SessionEntry {
                    state,
                    source: source.to_string(),
                    reason: reason.to_string(),
                    updated_at: Instant::now(),
                },
            );
        }
        self.aggregated()
    }

    pub fn prune_stale(&mut self) -> LightState {
        let timeout = self.idle_timeout;
        self.sessions
            .retain(|_, entry| entry.updated_at.elapsed() < timeout);
        self.aggregated()
    }

    pub fn aggregated(&self) -> LightState {
        self.sessions
            .values()
            .map(|entry| entry.state)
            .max()
            .unwrap_or(LightState::Green)
    }

    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn red_wins_over_yellow() {
        let mut sm = StateMachine::default();
        sm.set_session("a", LightState::Yellow, "cursor", "thinking");
        sm.set_session("b", LightState::Red, "cursor", "writing");
        assert_eq!(sm.aggregated(), LightState::Red);
    }

    #[test]
    fn green_session_removes_entry() {
        let mut sm = StateMachine::default();
        sm.set_session("a", LightState::Red, "cursor", "writing");
        sm.set_session("a", LightState::Green, "cursor", "idle");
        assert_eq!(sm.aggregated(), LightState::Green);
    }
}
