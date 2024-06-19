#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow)]
pub struct FullUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
