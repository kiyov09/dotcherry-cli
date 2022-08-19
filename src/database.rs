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
struct Gragh {
    name: String,
    user_id: String,
    code: String
}

impl Gragh {
    async fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = get_postgrest_client();

        let resp = client
            .from(GRAPH_TABLE_NAME)
            .insert(
                self.to_string().unwrap()
            )
            .execute()
            .await?;

        let body = resp.text().await?;

        println!("{:}", body);

        Ok(())
    }

    fn to_string(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}

pub async fn insert_on_db(code: &str) -> Result<(), Box<dyn std::error::Error>> {
    let graph = Gragh {
        name: "Inserted from Graph method".to_string(),
        user_id: "7febcbe7-a9d4-48b4-99c5-8c1f290ae934".to_string(),
        code: code.to_string()
    };

    graph.save().await?;

    Ok(())
}
