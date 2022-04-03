use diesel::prelude::*;

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
        user_email: em.to_owned(),
        password: ps.to_owned()
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}

// run query to retrieve user by email
pub fn find_user_by_email(
    email: String,
    conn: &SqliteConnection,
) -> Result<models::User, DbError> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(user_email.eq(email))
        .get_result::<models::User>(conn)?;

        Ok(user)
}
