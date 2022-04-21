use serde::{Deserialize, Serialize};

use crate::schema::users;
use crate::schema::recipes;

// for login
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub user_email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub user_email: String,
    pub password: String,
}

// for recipe
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Recipe {
    pub id: String,
    pub user_email: String,
    pub prep: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRecipe {
    pub user_email: String,
    pub prep: String,
    pub name: String,
}
