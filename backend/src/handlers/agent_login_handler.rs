use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_ws::{handle, Message, Session};
use log::{debug, error};
use std::sync::{Arc, Mutex};

use crate::models::appstate::{AppState, CacheRoom};

#[get("/ws/agent/{workspace_id}/{chatroom_id}/{room_id}")]
pub async fn agent_login_ss_handler(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<(u32, u32, u32)>,
    state: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let (workspace_id, chatroom_id, room_id) = path.into_inner();
    debug!(
        "Agent connecting to workspace {} chatroom {} room {}",
        workspace_id, chatroom_id, room_id
    );

    // WebSocket handshake
    let (response, session, mut msg_stream) = handle(&req, stream)?;
    let session = Arc::new(Mutex::new(session));

    // เก็บ session agent เข้า room
    {
        if let Ok(mut rooms) = state.rooms.lock() {
            rooms
                .entry(room_id)
                .or_insert_with(|| CacheRoom {
                    id: room_id,
                    ss_agents: Some(vec![]),
                    ss_customer: None,
                })
                .ss_agents
                .get_or_insert(vec![])
                .push(session.clone());
        } else {
            error!("Failed to lock rooms (poisoned mutex) while adding agent session");
        }
    }

    // Task อ่าน message
    let state_clone = state.clone();
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.recv().await {
            if let Message::Text(txt) = msg {
                if let Ok(rooms) = state_clone.rooms.lock() {
                    if let Some(room) = rooms.get(&room_id) {
                        if let Some(ss_customer) = &room.ss_customer {
                            let txt_clone = txt.clone();
                            let ss_customer = Arc::clone(ss_customer);
                            actix_web::rt::spawn(async move {
                                if let Ok(mut s) = ss_customer.lock() {
                                    if let Err(e) = s.text(txt_clone).await {
                                        error!("Failed to send to customer: {:?}", e);
                                    }
                                }
                            });
                        }
                    }
                } else {
                    error!("Failed to lock rooms (poisoned mutex) while broadcasting");
                }
            }
        }
    });

    Ok(response)
}
