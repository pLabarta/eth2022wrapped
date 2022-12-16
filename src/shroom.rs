use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use serde::{de::DeserializeOwned, Deserialize};

pub type Query = String;
pub type QueryId = String;

#[derive(PartialEq, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryData {
    pub results: Option<Vec<Vec<serde_json::Value>>>,
    pub column_labels: Vec<String>,
    pub column_types: Vec<String>,
    pub status: String,
    pub message: Option<String>,
    pub started_at: String,
    pub ended_at: String,
}

pub struct FailureData {}

pub fn extract_token(id: &QueryId) -> String {
    let json: serde_json::Value = serde_json::from_str(&id).unwrap();
    json["token"].as_str().unwrap().to_string()
}

#[derive(PartialEq)]
pub enum QueryResult {
    Running,
    Finished(QueryData),
    Failure(serde_json::Value),
}

impl Display for QueryResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryResult::Running => write!(f, "running"),
            QueryResult::Finished(data) => write!(
                f,
                "results: {:?}, column_labels: {:?}, column_types: {:?}, status: {}, started_at: {}, ended_at: {}",
                data.results, data.column_labels, data.column_types, data.status, data.started_at, data.ended_at
            ),
            QueryResult::Failure(error) => write!(f, "error: {:?}", error),
        }
    }
}

impl From<String> for QueryResult {
    fn from(s: String) -> Self {
        let json: serde_json::Value = serde_json::from_str(&s).unwrap();
        let status = json["status"].as_str().unwrap();
        match status {
            "running" => QueryResult::Running,
            "finished" => {
                let query_data: QueryData = serde_json::from_value(json).unwrap();

                QueryResult::Finished(query_data)
            }
            "failure" => {
                let error = json.clone();
                QueryResult::Failure(error)
            }
            _ => panic!("Unknown status"),
        }
    }
}

pub const QUERY_POST_URL: &str = "https://node-api.flipsidecrypto.com/queries";
pub const QUERY_GET_URL: &str = "https://node-api.flipsidecrypto.com/queries/";
pub const API_KEY: &str = "3c338bf6-8a74-4ec4-8b18-55b7d1db7004";

// pub async fn post_query(query: &Query) -> QueryId {
//     let client = reqwest::Client::new();
//     let mut map: HashMap<&str, String> = HashMap::new();
//     map.insert("sql", query.to_owned());
//     map.insert("ttl_minutes", "15".to_string());
//     map.insert("cache", "true".to_string());

//     let res = client
//         .post(QUERY_POST_URL)
//         .json(&map)
//         .header("x-api-key", API_KEY)
//         .send()
//         .await
//         .unwrap()
//         .text()
//         .await
//         .unwrap();

//     res
// }

// pub async fn get_query(token: &str) -> String {
//     let client = reqwest::Client::new();
//     let res = client
//         .get(&format!("{}{}", QUERY_GET_URL, token))
//         .header("x-api-key", API_KEY)
//         .send()
//         .await
//         .unwrap()
//         .text()
//         .await
//         .unwrap();

//     res
// }

// pub async fn wait_for_result(token: &str) -> QueryResult {
//     let mut result = QueryResult::Running;
//     while result == QueryResult::Running {
//         let query_result = get_query(token).await;
//         result = QueryResult::from(query_result);
//     }
//     result
// }
