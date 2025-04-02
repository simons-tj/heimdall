use crate::config::Config;
use neo4rs::{ConfigBuilder, Graph};

pub async fn connect(config: &Config) -> Graph {
    let config = ConfigBuilder::default()
        .uri(config.db_uri.to_string())
        .user(config.db_user.to_string())
        .password(config.db_pass.to_string())
        .build()
        .unwrap();

    Graph::connect(config).await.unwrap()
}
