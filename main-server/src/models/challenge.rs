use common::RunLangOutput;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error::Error;

#[derive(sqlx::FromRow, Deserialize, Serialize, Default)]
pub struct NewChallenge {
    pub description: String,
    pub judge: String,
    pub name: String,
    pub example_code: String,
}

#[derive(Serialize)]
pub struct NewChallengeWithTests {
    #[serde(flatten)]
    pub challenge: NewChallenge,
    pub tests: Option<RunLangOutput>,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Challenge {
    pub id: i32,
    #[sqlx(flatten)]
    pub challenge: NewChallenge,
    pub author: i32,
}

impl Challenge {
    pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Challenge>, Error> {
        let sql = "SELECT * FROM challenges WHERE id=$1".to_string();

        let challenge: Option<Challenge> = sqlx::query_as(&sql)
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|_| Error::DatabaseError)?;

        Ok(challenge)
    }
}
