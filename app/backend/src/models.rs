use serde::{Deserialize, Serialize};

use crate::schema::users;

// for login
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub user_email: String,
    pub password: String,
    pub instructions: String,
    pub ingredients: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub user_email: String,
    pub password: String,
    pub instructions: String,
    pub ingredients: String,
}
