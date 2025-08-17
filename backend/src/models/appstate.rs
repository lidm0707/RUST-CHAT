use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_ws::Session;

/// AppState เก็บ workspace ทั้งหมด
pub struct CacheRoom {
    pub id: u32,
    pub ss_agents: Option<Vec<Arc<Mutex<Session>>>>,
    pub ss_customer: Option<Arc<Mutex<Session>>>,
}

pub struct AppState {
    pub rooms: Mutex<HashMap<u32, CacheRoom>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            rooms: Mutex::new(HashMap::new()),
        }
    }
}
