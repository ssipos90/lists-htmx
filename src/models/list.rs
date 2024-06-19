#[derive(sqlx::FromRow)]
pub struct SlimList {
    pub id: i32,
    pub name: String,
    pub completed_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow)]
pub struct List {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    //pub created_by: i32,
    pub completed_at: Option<chrono::NaiveDateTime>,
    //pub completed_by: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateList {
    pub name: String,
    pub description: Option<String>,
}
