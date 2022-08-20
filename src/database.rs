use std::{env, result::Result};

use postgrest::Postgrest;
use serde::Serialize;
use serde_json::Value;

fn get_postgrest_client() -> postgrest::Postgrest {
    let supabase_api_key = env!("SUPABASE_API_KEY");
    let supabase_api_url = env!("SUPABASE_API_URL");

    Postgrest::new(supabase_api_url).insert_header("apikey", supabase_api_key)
}

static GRAPH_TABLE_NAME: &str = "graphs";

#[derive(Debug, Serialize)]
pub struct Graph {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    name: String,
    user_id: String,
    code: String,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            id: None,
            name: String::new(),
            user_id: "7febcbe7-a9d4-48b4-99c5-8c1f290ae934".to_string(),
            code: String::new(),
        }
    }

    pub fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_code(&mut self, code: &str) {
        self.code = code.to_string();
    }

    fn to_serde_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn partial_to_serde_string(&self) -> String {
        let partial = serde_json::json!({
            "code": self.code.clone(),
            "name": self.name.clone(),
        });
        serde_json::to_string(&partial).unwrap()
    }

    async fn insert(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let client = get_postgrest_client();

        let resp_body = client
            .from(GRAPH_TABLE_NAME)
            .insert(self.to_serde_string())
            .execute()
            .await?
            .text()
            .await?;

        let id = get_id_from_response_body(&resp_body);
        self.set_id(id);

        Ok(())
    }

    async fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let client = get_postgrest_client();

        client
            .from(GRAPH_TABLE_NAME)
            .eq("id", self.id.as_ref().unwrap())
            .update(self.partial_to_serde_string())
            .execute()
            .await?
            .text()
            .await?;

        Ok(())
    }

    pub async fn save(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let operation_result = if self.id.is_none() {
            self.insert().await
        } else {
            self.update().await
        };

        match operation_result {
            Ok(_) => Ok(self.id.as_ref().unwrap().clone()),
            Err(e) => Err(e),
        }
    }
}

fn get_id_from_response_body(resp_body: &str) -> Option<String> {
    let as_json: Value = serde_json::from_str(resp_body).unwrap();

    if let Value::Array(arr) = as_json {
        if !arr.is_empty() {
            let graph = arr[0].as_object().unwrap();
            let id = graph.get("id").unwrap().as_str().unwrap().to_string();
            return Some(id);
        }
    }

    None
}
