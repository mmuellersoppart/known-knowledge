use axum::response::Html;

pub async fn hello() -> String {
    "hellooo!!!!".to_string()
}

pub async fn helloo() -> Html<&'static str> {
    Html("<h1>Hello!!!</h1><p>Are you there?</p>")
}
