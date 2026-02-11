use serde::{Deserialize, Serialize};
pub enum Error {
    InvalidId(String),
    EmptyId,
    EmptyUsername,
    EmptyCollection,
    EmptyEmail,
    EmptyPassword,
    EmptyTextSearch,
    UserNotFound(String),
    UsernameAlreadyTaken,
    EmailAlreadyTaken,
    InvalidEmail(String),
    //MongoDb(MongoError),
    //Audit(AuditError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub slug: String,
    pub name: String,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    pub created: chrono::DateTime<chrono::Utc>,
}

impl Team {
    pub fn is_owner(&self) -> bool {
        false
    }
}

impl Team {
    async fn get_team(&self, _token: &str, team_id: &str) -> Result<Option<Team>> {
        Ok(Some(Team {
            id: team_id.to_string(),
            slug: team_id.to_string(),
            name: "Test Team".to_string(),
            created_at: 0,
            created: chrono::Utc::now(),
        }))
    }
}
