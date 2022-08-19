use std::{
    env,
    result::Result
};

use serde::Serialize;
use postgrest::Postgrest;

fn get_postgrest_client() -> postgrest::Postgrest {
    let supabase_api_key = env!("SUPABASE_API_KEY");
    let supabase_api_url = env!("SUPABASE_API_URL");

    Postgrest::new(supabase_api_url).insert_header("apikey", supabase_api_key)
}

static GRAPH_TABLE_NAME: &str = "graphs";

#[derive(Debug, Serialize)]
struct Graph {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    name: String,
    user_id: String,
    code: String
}

impl Graph {
    async fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = get_postgrest_client();

        let builder = client.from(GRAPH_TABLE_NAME);
        let operation = match &self.id {
            Some(id) => builder.eq("id", id.as_str())
                               .update(&self.partial_to_serde_string()),
            None => builder.insert(&self.to_serde_string())
        };
        let resp = operation.execute().await?;
        let _body = resp.text().await?;

        Ok(())
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
}

pub async fn insert_on_db(code: &str) -> Result<(), Box<dyn std::error::Error>> {
    let graph = Graph {
        id: Some("25e007b9-f12b-4396-899f-5d1cd454ab98".to_string()),
        name: "After adding id (Updated v2)".to_string(),
        user_id: "7febcbe7-a9d4-48b4-99c5-8c1f290ae934".to_string(),
        code: code.to_string()
    };

    graph.save().await?;

    Ok(())
}
