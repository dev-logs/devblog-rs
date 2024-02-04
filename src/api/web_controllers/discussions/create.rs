// use leptos::{server, ServerFnError};
//
// #[server(AxumExtract, "/api")]
// pub async fn axum_extract() -> Result<String, ServerFnError> {
//     use axum::{extract::Query, http::Method};
//     use leptos_axum::extract;
//
//     extract(|method: Method, res: Query<MyQuery>| async move {
//         format!("{method:?} and {}", res.q)
//     },
//     )
//         .await
//         .map_err(|e| ServerFnError::ServerError("Could not extract method and query...".to_string()))
// }