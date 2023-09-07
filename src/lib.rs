use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use dal::{Blog, Dal};
use shuttle_runtime::CustomError;
use sqlx::{migrate::Migrator, PgPool};
use tracing::error;

mod dal;

async fn blog(State(dal): State<Dal>) -> Result<Json<Blog>, StatusCode> {
    let blog = dal.get_blog("tmp").await.map_err(|error| {
        error!(%error, "failed to get blog from storage");
        StatusCode::NOT_FOUND
    })?;

    Ok(Json(blog))
}

static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn app(pool: PgPool) -> Result<Router, CustomError> {
    MIGRATOR.run(&pool).await.map_err(CustomError::new)?;

    let router = Router::new()
        .route("/", get(blog))
        .with_state(Dal::new(pool));

    Ok(router)
}
