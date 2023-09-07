use blog::app;
use sqlx::PgPool;
use std::path::PathBuf;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_static_folder::StaticFolder(folder = "assets")] assets_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    let app = app(pool, assets_folder).await?;

    Ok(app.into())
}
