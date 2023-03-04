use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CreateIdea {
    pub name: String,
    pub context: Option<String>,
    pub description: Option<String>,
}