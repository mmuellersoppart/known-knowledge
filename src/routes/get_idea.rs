use axum::extract::{Path, State};
use uuid::Uuid;

use super::AppState;

pub async fn get_idea(Path(idea_id): Path<Uuid>, State(app_state): State<AppState>) {
    println!("{app_state:?}");
    println!("{idea_id}");
}
