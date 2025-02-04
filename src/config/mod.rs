#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_port: u16,
    pub base_url: String,
    pub max_slug_length: usize,
    pub redis_url: String,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            server_port: 8080,
            base_url: "http://localhost:8080".to_string(),
            max_slug_length: 8,
            redis_url: "redis://redis:6379".to_string(),
        }
    }

    pub fn from_env() -> Self {
        Self {
            server_port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            base_url: std::env::var("BASE_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            max_slug_length: 8,
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://redis:6379".to_string()),
        }
    }
}
