use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::get,
    Router,
};
use dal::Dal;
use handlebars::{handlebars_helper, no_escape, Handlebars};
use shuttle_runtime::CustomError;
use sqlx::{migrate::Migrator, PgPool};
use std::path::PathBuf;
use tower_http::services::ServeDir;
use tracing::error;

mod dal;

async fn home(
    State(AppState { dal, templates }): State<AppState>,
) -> Result<Html<String>, StatusCode> {
    let blogs = dal.get_blogs().await.map_err(|error| {
        error!(%error, "failed to get blogs from storage");
        StatusCode::NOT_FOUND
    })?;

    let page = templates.render("home", &blogs).map_err(|error| {
        error!(%error, "failed to home page");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Html(page))
}

async fn blog(
    Path(title): Path<String>,
    State(AppState { dal, templates }): State<AppState>,
) -> Result<Html<String>, StatusCode> {
    let blog = dal.get_blog(&title).await.map_err(|error| {
        error!(%error, "failed to get blog from storage");
        StatusCode::NOT_FOUND
    })?;

    let page = templates.render("blog", &blog).map_err(|error| {
        error!(%error, "failed to render blog");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Html(page))
}

static MIGRATOR: Migrator = sqlx::migrate!();

#[derive(Clone)]
struct AppState {
    dal: Dal,
    templates: Handlebars<'static>,
}

handlebars_helper!(markdown: |md: String| comrak::markdown_to_html(&md, &Default::default()));

pub async fn app(pool: PgPool, assets_folder: PathBuf) -> Result<Router, CustomError> {
    MIGRATOR.run(&pool).await.map_err(CustomError::new)?;

    let mut handlebars = Handlebars::new();
    handlebars.register_helper("markdown", Box::new(markdown));
    handlebars.register_escape_fn(no_escape);
    handlebars.register_templates_directory(".hbs", assets_folder.join("templates"))?;

    let state = AppState {
        dal: Dal::new(pool),
        templates: handlebars,
    };

    let router = Router::new()
        .route("/", get(home))
        .route("/:title", get(blog))
        .nest_service("/static", ServeDir::new(assets_folder.join("static")))
        .with_state(state);

    Ok(router)
}
