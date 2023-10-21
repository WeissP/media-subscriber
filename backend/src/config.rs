use getset_scoped::Getters;
use std::net::SocketAddr;

#[derive(clap::Parser, Debug, Getters)]
#[command(version)]
#[get = "pub"]
pub struct Config {
    /// The connection URL for the Postgres database this application should use.
    #[clap(long, env = "MS_DATABASE_URL")]
    database_url: String,

    /// The name of the session cookie.
    #[clap(
        long,
        env = "MS_SESSION_COOKIE_NAME",
        default_value = "media_subscriber_session"
    )]
    session_cookie_name: String,

    /// The path to the front-end public directory.
    #[clap(long, env = "MS_FRONT_PUBLIC")]
    front_public: String,

    /// The server port.
    #[clap(long, env = "MS_SERVER_PORT")]
    server_port: String,

    /// The server host.
    #[clap(long, env = "MS_SERVER_HOST", default_value = "127.0.0.1")]
    server_host: String,
}

impl Config {
    pub fn socket_addr(&self) -> SocketAddr {
        format!("{}:{}", self.server_host, self.server_port)
            .parse()
            .expect("Can not parse address and port")
    }
}
