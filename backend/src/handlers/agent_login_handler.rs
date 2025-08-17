use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_ws::{handle, Message, Session};
use futures::lock::Mutex;
use log::{debug, error};
use std::sync::Arc;

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
    let my_session = session.clone();

    // เก็บ session agent เข้า room
    {
        let mut rooms = state.rooms.lock().await;
        let room = rooms.entry(room_id).or_insert_with(|| CacheRoom {
            id: room_id,
            ss_agents: Some(vec![]),
            ss_customer: None,
        });

        let agents = room.ss_agents.get_or_insert(vec![]);
        agents.push(session.clone());
    }

    // Task อ่าน message
    let state_clone = state.clone();
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.recv().await {
            if let Message::Text(txt) = msg {
                let rooms = state_clone.rooms.lock().await;
                if let Some(room) = rooms.get(&room_id) {
                    let txt_clone = txt.clone();

                    // ส่งให้ customer
                    if let Some(ss_customer) = &room.ss_customer {
                        let ss_customer: Arc<Mutex<Session>> = Arc::clone(ss_customer);
                        let txt_to_customer = txt_clone.clone();
                        actix_web::rt::spawn(async move {
                            let mut s = ss_customer.lock().await;
                            if let Err(e) = s.text(txt_to_customer).await {
                                error!("Failed to send to customer: {:?}", e);
                            }
                        });
                    }

                    // ส่งให้ agent คนอื่น
                    if let Some(ss_agents) = &room.ss_agents {
                        for agent in ss_agents {
                            if Arc::ptr_eq(agent, &my_session) {
                                continue; // ข้ามตัวเอง
                            }
                            let agent_clone: Arc<Mutex<Session>> = Arc::clone(agent);
                            let txt_to_agent = txt_clone.clone();
                            actix_web::rt::spawn(async move {
                                let mut s = agent_clone.lock().await;
                                if let Err(e) = s.text(txt_to_agent).await {
                                    error!("Failed to send to agent: {:?}", e);
                                }
                            });
                        }
                    }
                }
            }
        }
    });

    Ok(response)
}
