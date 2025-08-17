use actix_ws::Session;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Workspace == Account
#[derive(Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: u32,
    pub name: String,
    pub teams: HashMap<u32, Team>,
    pub chatrooms: HashMap<u32, Chatroom>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Team {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub agents: HashMap<u32, Agent>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Agent {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub session: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Customer {
    pub id: u32,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

/// Chatroom = grouping of conversations (like a project / channel)
#[derive(Clone, Serialize, Deserialize)]
pub struct Chatroom {
    pub id: u32,
    pub team_id: u32,
    pub rooms: HashMap<u32, Room>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: u32,
    pub customer_id: u32,
    pub channel_type: String,
    pub agent_id: Option<u32>, // conversation อาจยังไม่ assign agent
    pub state: ConversationState,
    pub messages: Vec<Message>,

    #[serde(skip)] // ไม่ serialize session
    pub sessions: Vec<Arc<Mutex<Session>>>,

    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    pub id: u32,
    pub room_id: u32,
    pub sender_id: u32, // อาจเป็น customer หรือ agent
    pub sender_type: SenderType,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SenderType {
    Customer,
    Agent,
    System,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConversationState {
    Open,
    Resolved,
    Snoozed,
    Bot,
}

impl Workspace {
    pub fn new(id: u32, name: impl Into<String>) -> Self {
        Workspace {
            id,
            name: name.into(),
            teams: HashMap::new(),
            chatrooms: HashMap::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl Chatroom {
    pub fn new(id: u32, team_id: u32) -> Self {
        Chatroom {
            id,
            team_id,
            rooms: HashMap::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl Room {
    pub fn new(id: u32, customer_id: u32, channel_type: impl Into<String>) -> Self {
        Room {
            id,
            customer_id,
            channel_type: channel_type.into(),
            agent_id: None,
            state: ConversationState::Open,
            messages: Vec::new(),
            sessions: Vec::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Workspace {
            id: 0,
            name: "Default Workspace".to_string(),
            teams: HashMap::new(),
            chatrooms: HashMap::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl Default for Chatroom {
    fn default() -> Self {
        Chatroom {
            id: 0,
            team_id: 0,
            rooms: HashMap::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl Default for Room {
    fn default() -> Self {
        Room {
            id: 0,
            customer_id: 0,
            channel_type: "default".to_string(),
            agent_id: None,
            state: ConversationState::Open,
            messages: Vec::new(),
            sessions: Vec::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}
