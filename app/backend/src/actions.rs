use diesel::prelude::*;
use uuid::Uuid;

use crate::models;

type DbError = Box<dyn std::error::Error + Send + Sync>;

// use Diesel to input new database row
pub fn insert_new_user(
    // prevent collision with 'email' column
    em: String,
    ps: String,
    ins: String,
    ing: String,
    conn: &SqliteConnection,
) -> Result<models::User, DbError> {
    use crate::schema::users::dsl::*;

    let new_user = models::User {
        id: Uuid::new_v4().to_string(),
        user_email: em.to_owned(),
        password: ps.to_owned(),
        instructions: ins.to_owned(),
        ingredients: ing.to_owned(),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?; // ERROR CHECKING NEEDED
    println!("new user with email {} has been added!", em);

    Ok(new_user)
}

// query by email
pub fn find_user_by_email(
    conn: &SqliteConnection,
) -> Result<Option<Vec<models::User>>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(user_email.eq("Snow".to_string()))
        .limit(5)
        .load::<models::User>(conn)
        .optional()?; // ERROR CHECKING NEEDED

    Ok(user)
}
