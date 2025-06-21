


use async_trait::async_trait;

#[async_trait]
pub trait AiService {
    async fn generate_insights(&self, text: &str) -> Result<String, String>;
}


