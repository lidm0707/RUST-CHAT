use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_ws::{handle, Message, Session};
use log::debug;
use std::sync::{Arc, Mutex};

use crate::models::appstate::{AppState, CacheRoom};

#[get("/ws/{workspace_id}/{chatroom_id}/{room_id}")]
pub async fn chat_ws(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<(u32, u32, u32)>,
    state: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let (workspace_id, chatroom_id, room_id) = path.into_inner();
    debug!(
        "Client connecting to workspace {} chatroom {} room {}",
        workspace_id, chatroom_id, room_id
    );

    // WS handshake
    let (response, session, mut msg_stream) = handle(&req, stream)?;
    let state = state.clone();
    let session = Arc::new(Mutex::new(session));

    // เก็บ session เข้า room
    {
        let mut rooms = state.rooms.lock().unwrap();
        rooms
            .entry(room_id)
            .or_insert_with(|| CacheRoom {
                id: workspace_id,
                ss_agents: None,
                ss_customer: None,
            })
            .ss_customer = Some(session.clone());
    }

    // Task รับข้อความจาก client
    let state_clone = state.clone();
    let session_clone = session.clone();
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.recv().await {
            match msg {
                Message::Text(txt) => {
                    // Broadcast ไปทุก agent ใน room
                    let rooms = state_clone.rooms.lock().unwrap();
                    if let Some(room) = rooms.get(&room_id) {
                        if let Some(ref ss_agents) = room.ss_agents {
                            for agent in ss_agents.iter() {
                                let agent = Arc::clone(agent);
                                let txt = txt.clone();
                                actix_web::rt::spawn(async move {
                                    if let Ok(mut s) = agent.lock() {
                                        let _ = s.text(txt).await;
                                    }
                                });
                            }
                        }
                    }
                }
                Message::Close(reason) => {
                    debug!("Client closed: {:?}", reason);
                    // TODO: ลบ session ออกจาก room ถ้าต้องการ
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(response)
}
