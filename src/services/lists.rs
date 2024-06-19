use sqlx::PgPool;

use crate::{
    common::{PaginatedResult, Pagination},
    error::ServiceError,
    models
};

type PaginatedLists = PaginatedResult<models::SlimList>;

#[tracing::instrument(skip(pool))]
pub async fn fetch_lists(
    pool: &PgPool,
    Pagination {
        page,
        items_per_page,
    }: Pagination,
) -> Result<PaginatedLists, ServiceError> {
    let skip = (page - 1) * items_per_page;
    let lists = sqlx::query_as!(
        models::SlimList,
        r#"
        SELECT
            id,
            name,
            completed_at,
            created_at,
            updated_at
        FROM lists
        ORDER BY created_at DESC
        LIMIT $1::INTEGER
        OFFSET $2::INTEGER
        "#,
        items_per_page,
        skip
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ServiceError::DatabaseError("Failed to fetch lists.".into(), e))?;

    let total = sqlx::query!("SELECT COUNT(id) AS count FROM lists")
        .fetch_one(pool)
        .await
        .map_err(|e| ServiceError::DatabaseError("Failed to fetch total count.".into(), e))?;

    let total_count: i64 = total.count.unwrap_or(0);

    let total_pages = (total_count as f64 / items_per_page as f64).ceil() as i32;

    Ok(PaginatedLists {
        items: lists,
        items_per_page,
        page,
        total_pages,
    })
}

#[tracing::instrument(skip(pool))]
pub async fn fetch_list(pool: &PgPool, list_id: i32) -> Result<Option<models::List>, ServiceError> {
    sqlx::query_as!(models::List, "SELECT * FROM lists WHERE id = $1", list_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| ServiceError::DatabaseError("Failed to fetch list.".into(), e))
}

#[tracing::instrument(skip(pool))]
pub async fn create_list(
    pool: &PgPool,
    dto: models::CreateList,
) -> Result<models::List, ServiceError>{
    sqlx::query_as!(
        models::List,
        r#"
        INSERT INTO lists (name, description)
        VALUES ($1, $2)
        RETURNING *
        "#,
        dto.name,
        dto.description
    )
    .fetch_one(pool)
    .await
    .map_err(|e| ServiceError::DatabaseError("Failed to create list.".into(), e))
}
