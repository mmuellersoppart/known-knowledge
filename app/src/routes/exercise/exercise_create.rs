use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set, TransactionTrait};
use serde::Deserialize;
use uuid::Uuid;

use crate::{route_utils::app_errors::AppError, routes::AppState};

use entity::{
    exercise, exerciseable, exerciseable_external, idea, sea_orm_active_enums::ExerciseableType, exerciseable_notecard, card,
};


// External
#[derive(Deserialize, Debug)]
pub struct CreateExternalExercise {
    title: String,
    link: String,
}

pub async fn create_exercise_external(
    State(app_state): State<AppState>,
    Path(idea_id): Path<Uuid>,
    Json(external_data): Json<CreateExternalExercise>,
) -> Result<(), AppError> {
    let txn = app_state.db.begin().await.map_err(AppError::from)?;

    // Create Explainable Id
    let active_exerciseable = exerciseable::ActiveModel {
        ..Default::default()
    };

    let exerciseable_id = active_exerciseable
        .save(&txn)
        .await
        .map_err(AppError::from)?
        .id
        .unwrap();

    // Create markdown
    let external = exerciseable_external::ActiveModel {
        exerciseable_id: Set(exerciseable_id),
        link: Set(external_data.link),
        ..Default::default()
    };

    exerciseable_external::Entity::insert(external)
        .exec(&txn)
        .await
        .map_err(AppError::from)?;

    // Create explanation
    let exercise = exercise::ActiveModel {
        idea_id: Set(idea_id),
        exerciseable_id: Set(exerciseable_id),
        exerciseable_type: Set(Some(ExerciseableType::External)),
        title: Set(external_data.title),
        ..Default::default()
    };

    exercise.save(&txn).await.map_err(AppError::from)?;

    txn.commit().await.map_err(AppError::from)?;

    Ok(())
}


// Notecard
#[derive(Deserialize, Debug)]
pub struct CreateNotecardDeck { // This is really for the deck (cards made separately)
    title: String,
}

pub async fn create_exercise_notecard(
        State(app_state): State<AppState>,
        Path(idea_id): Path<Uuid>,
        Json(deck_data): Json<CreateNotecardDeck>,
        ) -> Result<(), AppError> {
    let txn = app_state.db.begin().await.map_err(AppError::from)?;

    // Create Explainable Id
    let active_exerciseable = exerciseable::ActiveModel {
        ..Default::default()
    };

    let exerciseable_id = active_exerciseable
        .save(&txn)
        .await
        .map_err(AppError::from)?
        .id
        .unwrap();

    // Create markdown
    let deck = exerciseable_notecard::ActiveModel {
        exerciseable_id: Set(exerciseable_id),
        ..Default::default()
    };

    exerciseable_notecard::Entity::insert(deck)
        .exec(&txn)
        .await
        .map_err(AppError::from)?;

    // Create explanation
    let exercise = exercise::ActiveModel {
        idea_id: Set(idea_id),
        exerciseable_id: Set(exerciseable_id),
        exerciseable_type: Set(Some(ExerciseableType::Notecard)),
        title: Set(deck_data.title),
        ..Default::default()
    };

    exercise.save(&txn).await.map_err(AppError::from)?;

    txn.commit().await.map_err(AppError::from)?;

    Ok(())
}


// Notecard
#[derive(Deserialize, Debug)]
pub struct CreateCard {
    front: String,
    back: String,
}

pub async fn create_card(
        State(app_state): State<AppState>,
        Path(deck_id): Path<Uuid>,
        Json(card_data): Json<CreateCard>,
        ) -> Result<(), AppError> {
    let txn = app_state.db.begin().await.map_err(AppError::from)?;

    // TODO: check if deck exists

    // Create markdown
    let new_card = card::ActiveModel {
        deck_id: Set(deck_id),
        front: Set(Some(card_data.front)),
        back: Set(Some(card_data.back)),
        ..Default::default()
    };

    card::Entity::insert(new_card)
        .exec(&txn)
        .await
        .map_err(AppError::from)?;

    txn.commit().await.map_err(AppError::from)?;

    Ok(())
}