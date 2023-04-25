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

impl ListIdea {
    fn from(result: QueryResult) -> ListIdea {
        ListIdea {
            id: Default::default(),
            name: "".to_string(),
            context: None,
        }
    }
}

pub async fn ideas_list(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<ListIdea>>, StatusCode> {
    let ideas = idea::Entity::find()
        .order_by_asc(idea::Column::Name)
        .all(&app_state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let ideas: Vec<ListIdea> = ideas
        .iter()
        .map(|idea| ListIdea {
            id: idea.id,
            name: idea.name.clone(),
            context: idea.context.clone(),
        })
        .collect();
    Ok(Json(ideas))
}

// pub async fn ideas_list(
//     State(app_state): State<AppState>,
// ) -> Result<Json<Vec<ListIdea>>, StatusCode> {
//
//     let (sql, values) = sea_query::Query::select()
//         .columns([
//             Idea::Name,
//             Idea::Context,
//             Idea::Description,
//         ])
//         .from(Idea::Table)
//         .build(PostgresQueryBuilder);
//
//     let stmt = Statement::from_sql_and_values(DatabaseBackend::Postgres, &sql.as_str(), values);
//
//     let query_result: Vec<QueryResult> = app_state.db.query_all(stmt).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
//
//
//
//     Err(StatusCode::INTERNAL_SERVER_ERROR)
//     // let list_ideas: Vec<ListIdea> = query_result.into_iter().map(|result| ListIdea::from())
//
//     //
//     // let ideas = idea::Entity::find()
//     //     .order_by_asc(idea::Column::Name)
//     //     .all(&app_state.db)
//     //     .await
//     //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
//     //
//     // let ideas: Vec<ListIdea> = ideas
//     //     .iter()
//     //     .map(|idea| ListIdea {
//     //         id: idea.id,
//     //         name: idea.name.clone(),
//     //         context: idea.context.clone(),
//     //     })
//     //     .collect();
//     // Ok(Json(ideas))
// }