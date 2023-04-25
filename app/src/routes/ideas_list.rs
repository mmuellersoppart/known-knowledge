use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ConnectionTrait, DatabaseBackend, EntityTrait, QueryOrder, QueryResult, Statement, FromQueryResult};
use sea_query;
use sea_query::PostgresQueryBuilder;
use serde::Serialize;
use uuid::Uuid;

use super::AppState;
use::entity::idea;
use migration::iden::Idea;

#[derive(Debug, Serialize, FromQueryResult)]
pub struct ListIdea {
    id: Uuid,
    name: String,
    context: Option<String>,
}

// pub async fn ideas_list(
//     State(app_state): State<AppState>,
// ) -> Result<Json<Vec<ListIdea>>, StatusCode> {
//     let ideas = idea::Entity::find()
//         .order_by_asc(idea::Column::Name)
//         .all(&app_state.db)
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
//
//     let ideas: Vec<ListIdea> = ideas
//         .iter()
//         .map(|idea| ListIdea {
//             id: idea.id,
//             name: idea.name.clone(),
//             context: idea.context.clone(),
//         })
//         .collect();
//     Ok(Json(ideas))
// }

pub async fn ideas_list(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<ListIdea>>, StatusCode> {

    let (sql, values) = sea_query::Query::select()
        .columns([
            Idea::Id,
            Idea::Name,
            Idea::Context,
        ])
        .from(Idea::Table)
        .build(PostgresQueryBuilder);

    let stmt = Statement::from_sql_and_values(DatabaseBackend::Postgres, &sql.as_str(), values);

    let query_result: Vec<QueryResult> = app_state.db.query_all(stmt).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let ideas: Vec<ListIdea> = query_result.into_iter()
        .map(|result| {
            let id = result.try_get::<Uuid>("", "id").expect("failed to get id");
            let name = result.try_get::<String>("", "name").expect("failed to get name");
            let context = result.try_get::<Option<String>>("", "context").expect("failed to get context");
            ListIdea {
                id,
                name,
                context,
            }
        })
        .collect();

    Ok(Json(ideas))
}