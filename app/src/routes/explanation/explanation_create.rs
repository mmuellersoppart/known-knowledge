use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use entity::{
    explainable, explainable_markdown, explanation, idea, sea_orm_active_enums::ExplainableType,
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set, TransactionTrait};
use serde::Deserialize;
use uuid::Uuid;

use crate::{route_utils::app_errors::AppError, routes::AppState};

#[derive(Deserialize, Debug)]
pub struct CreateMarkdownExercise {
    title: String,
    markdown: String,
}

pub async fn create_explanation_markdown(
    State(app_state): State<AppState>,
    Path(idea_id): Path<Uuid>,
    Json(markdown_data): Json<CreateMarkdownExercise>,
) -> Result<(), AppError> {
    let txn = app_state.db.begin().await.map_err(AppError::from)?;

    // confirm id is legit
    let active_idea = idea::Entity::find_by_id(idea_id)
        .one(&app_state.db)
        .await
        .map_err(AppError::from)?;

    let idea_id = if let Some(active_idea) = active_idea {
        active_idea.id
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "Failed to find idea."));
    };

    // Create Explainable Id
    let active_explainable = explainable::ActiveModel {
        ..Default::default()
    };

    let explainable_id = active_explainable
        .save(&txn)
        .await
        .map_err(AppError::from)?
        .id
        .unwrap();

    // Create markdown
    let markdown = explainable_markdown::ActiveModel {
        explainable_id: Set(explainable_id),
        markdown: Set(markdown_data.markdown),
        ..Default::default()
    };

    explainable_markdown::Entity::insert(markdown)
        .exec(&txn)
        .await
        .map_err(AppError::from)?;

    // Create explanation
    let explanation = explanation::ActiveModel {
        idea_id: Set(idea_id),
        explainable_id: Set(explainable_id),
        explainable_type: Set(Some(ExplainableType::Markdown)),
        title: Set(markdown_data.title),
        ..Default::default()
    };

    explanation.save(&txn).await.map_err(AppError::from)?;

    txn.commit().await.map_err(AppError::from)?;

    Ok(())
}

pub async fn create_explanation_external(// State(app_state): State<AppState>,
    // Path(idea_id): Path<Uuid>,
    // Json(idea_data): Json<CreateMarkdownExercise>,
) -> String {
    // get an active model
    // let active_idea = idea::Entity::find_by_id(idea_id);

    // set the variables

    // save

    String::from("hehe")
}
