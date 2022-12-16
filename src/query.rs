use std::collections::HashMap;

use gloo_net::http::Request;

use crate::shroom::{QueryId, QueryResult, API_KEY, QUERY_GET_URL, QUERY_POST_URL};

pub async fn post_query(query: &str) -> QueryId {
    let mut map: HashMap<&str, String> = HashMap::new();
    map.insert("sql", query.to_owned());
    map.insert("ttl_minutes", "15".to_string());
    map.insert("cache", "true".to_string());

    let request = Request::post(QUERY_POST_URL)
        .header("x-api-key", API_KEY)
        .json(&map)
        .unwrap();

    let response = request.send().await.unwrap();
    let text = response.text().await.unwrap();

    text
}

pub async fn get_query(token: &str) -> String {
    let request = Request::get(&format!("{}{}", QUERY_GET_URL, token)).header("x-api-key", API_KEY);

    let response = request.send().await.unwrap();
    let text = response.text().await.unwrap();

    text
}

pub async fn wait_for_result(token: &str) -> QueryResult {
    let mut result = QueryResult::Running;
    while result == QueryResult::Running {
        let query_result = get_query(token).await;
        result = QueryResult::from(query_result);
    }
    result
}
