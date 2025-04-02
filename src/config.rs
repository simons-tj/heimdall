use clap::Parser;

/// The configuration parameters for the application.
///
/// These can either be passed on the command line, or pulled from environment variables.
/// The latter is preferred as environment variables are one of the recommended ways to
/// get configuration from Kubernetes Secrets in deployment.
///
/// This is a pretty simple configuration struct as far as backend APIs go. You could imagine
/// a bunch of other parameters going here, like API keys for external services
/// or flags enabling or disabling certain features or test modes of the API.
///
/// For development convenience, these can also be read from a `.env` file in the working
/// directory where the application is started.
///
/// See `.env.sample` in the repository root for details.
#[derive(Parser, Debug)]
pub struct Config {
    #[clap(long, default_value = "development")]
    pub app_env: String,
    #[clap(long, default_value = "0.0.0.0")]
    pub app_host: String,
    #[clap(long, default_value = "8080")]
    pub app_port: u16,
    #[clap(long, default_value = "localhost:7687")]
    pub db_uri: String,
    #[clap(long, default_value = "neo4j")]
    pub db_user: String,
    #[clap(long, default_value = "strong_password")]
    pub db_pass: String,
}
