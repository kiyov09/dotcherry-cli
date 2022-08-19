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

#[derive(Debug, Serialize)]
struct Gragh {
    name: String,
    user_id: String,
    code: String
}

pub async fn insert_on_db(code: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = get_postgrest_client();

    let graph = Gragh {
        name: "After removing dotenv".to_string(),
        user_id: "7febcbe7-a9d4-48b4-99c5-8c1f290ae934".to_string(),
        code: code.to_string()
    };

    let resp = client
        .from("graphs")
        .insert(
            serde_json::to_string(&graph).unwrap()
        )
        .execute()
        .await?;

    let body = resp.text().await?;

    println!("{:}", body);

    Ok(())
}
