use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_ws::{Message, Session};
use futures_util::StreamExt;
use std::sync::{Arc, Mutex};

/// เก็บ agent sessions ไว้ใน memory (ตัวอย่างง่าย ๆ)
type AgentSessions = Arc<Mutex<Vec<Session>>>;

pub async fn agent_ws(
    req: HttpRequest,
    body: web::Payload,
    agents: web::Data<AgentSessions>,
) -> Result<HttpResponse, Error> {
    // เปิด WebSocket
    let (res, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    // clone เพื่อใส่เข้า sessions
    let mut agents_ref = agents.lock().unwrap();
    agents_ref.push(session.clone());

    // spawn task แยกไว้สำหรับรับข้อความจาก agent
    let agents2 = agents.clone();
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Text(text) => {
                    println!("Agent ส่งข้อความ: {}", text);

                    // broadcast หาลูกค้าหรือ agent คนอื่น
                    let mut sessions = agents2.lock().unwrap();
                    for s in sessions.iter_mut() {
                        let _ = s.text(format!("Broadcast: {}", text));
                    }
                }
                Message::Ping(bytes) => {
                    let _ = session.pong(&bytes);
                }
                Message::Close(reason) => {
                    println!("Agent disconnect: {:?}", reason);
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(res)
}
