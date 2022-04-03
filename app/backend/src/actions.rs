use diesel::prelude::*;
use uuid::Uuid;

use crate::models;

type DbError = Box<dyn std::error::Error + Send + Sync>;

// use Diesel to input new database row
pub fn insert_new_user(
    // prevent collision with 'email' column
    em: String,
    ps: String,
    conn: &SqliteConnection,
) -> Result<models::User, DbError> {
    use crate::schema::users::dsl::*;

    let new_user = models::User {
        id: Uuid::new_v4().to_string(),
        user_email: em.to_owned(),
        password: ps.to_owned()
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}

// run query to retrieve user by id
pub fn find_user_by_email(
    uid: Uuid,
    conn: &SqliteConnection,
) -> Result<Option<models::User>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(uid.to_string()))
        .first::<models::User>(conn)
        .optional()?;

        Ok(user)
}