use blog::app;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    let app = app(pool).await?;

    Ok(app.into())
}
