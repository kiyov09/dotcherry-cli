use std::{
    env,
    result::Result
};

use serde::Serialize;
use postgrest::Postgrest;
use serde_json::Value;

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

// static mut GRAPH_INSTANCE: Graph = Graph {
//     id: None,
//     name: String::new(),
//     user_id: String::new(),
//     code: String::new()
// };
static mut GRAPH_INSTANCE: Option<Graph> = None ;

impl Graph {
    fn new() -> Self  {
        Self {
            id: None,
            name: String::new(),
            user_id: String::new(),
            code: String::new()
        }
    }

    fn get_instance() -> &'static mut Graph {
        unsafe {
            match GRAPH_INSTANCE {
                Some(ref mut graph) => graph,
                None => {
                    let graph = Graph::new();
                    GRAPH_INSTANCE = Some(graph);
                    GRAPH_INSTANCE.as_mut().unwrap()
                }
            }
        }
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

    async fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let client = get_postgrest_client();

        let builder = client.from(GRAPH_TABLE_NAME);
        let operation = match &self.id {
            Some(id) => builder.eq("id", id.as_str())
                               .update(&self.partial_to_serde_string()),
            None => builder.insert(&self.to_serde_string())
        };
        let resp = operation.execute().await?;
        let body = resp.text().await?;

        let as_json: Value = serde_json::from_str(&body).unwrap();
        if let Value::Array(arr) = as_json {
            if !arr.is_empty() {
                let graph = arr[0].as_object().unwrap();
                let id = graph.get("id").unwrap().as_str().unwrap().to_string();
                self.id = Some(id);
            }
        }

        Ok(())
    }
}

pub async fn save_graph(name: &str, code: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut graph = Graph::get_instance();

    graph.name = name.to_string();
    graph.user_id = "7febcbe7-a9d4-48b4-99c5-8c1f290ae934".to_string();
    graph.code = code.to_string();

    graph.save().await?;

    Ok(())
}
