use crate::models::agent_model::AgentModel;

pub struct TeamModel {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub agents: Vec<AgentModel>,
}
