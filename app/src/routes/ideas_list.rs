use axum::{extract::State, http::StatusCode, Json};
use axum::extract::Query;
use sea_orm::{ConnectionTrait, DatabaseBackend, EntityTrait, QueryOrder, QueryResult, Statement, FromQueryResult};
use sea_query;
use sea_query::{Expr, Iden, IntoColumnRef, PostgresQueryBuilder, SelectStatement};
use sea_query::extension::postgres::PgExpr;
use serde::{Deserialize, Serialize};
use tracing::log::{log, Level};
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

#[derive(Debug, Deserialize)]
pub struct Params {
    name_filter: Option<String>,
    context_filter: Option<String>
}

fn apply_filter(mut query: SelectStatement, col: impl IntoColumnRef, text: Option<String>) -> SelectStatement {
    if let Some(text) = text {
        return query.and_where(Expr::col(col).ilike(text + "%")).to_owned()
    }

    query
}

pub async fn ideas_list(
    Query(params): Query<Params>,
    State(app_state): State<AppState>,
) -> Result<Json<Vec<ListIdea>>, StatusCode> {


    log!(Level::Debug, "{params:?}");

     let query = sea_query::Query::select()
        .columns([
            Idea::Id,
            Idea::Name,
            Idea::Context,
        ])
        .from(Idea::Table)
         .to_owned();

    let query = apply_filter(query, Idea::Name, params.name_filter);
    let query = apply_filter(query, Idea::Context, params.context_filter);

    let (sql, values) = query.build(PostgresQueryBuilder);

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