pub mod init {
    use anyhow::{bail, Context};
    use sqlx::{postgres::PgPoolOptions, PgPool};
    use std::collections::HashMap;
    use tracing::{debug, error, info};

    /// # tl;dr
    /// This server needs to be run with the --host=<host> option.
    /// If it is not found, host defaults to `db`, which is the
    /// service name specified at `compose.yml`

    /// # Usage
    /// During development, run this program with the the above mentioned argument:
    /// <br />
    /// `cargo run -- --host=localhost`
    ///
    ///
    /// # Warning
    /// The database containers service name could change at any moment
    /// so gl hf! :D
    fn pg_host() -> String {
        match std::env::args().find(|arg| arg.starts_with("--host=")) {
            Some(s) => {
                let host = s.strip_prefix("--host=").unwrap_or("");

                if host.is_empty() {
                    info!(host, "--host arg was formatted poorly");
                    "db".into()
                } else {
                    info!(host, "--host arg found");
                    host.into()
                }
            }
            None => {
                info!("--host arg was not found");
                "db".into()
            }
        }
    }

    async fn try_connection(db_url: &str) -> anyhow::Result<PgPool> {
        let mut tries = 5;
        loop {
            tries -= 1;
            let pool = PgPoolOptions::new()
                .max_connections(4)
                .connect(db_url)
                .await;

            match pool {
                Ok(p) => {
                    info!("Connected to {db_url}");
                    return Ok(p);
                }
                Err(e) => {
                    error!(db_url, "Connection failed, retries left: {}", tries,);

                    if tries == 0 {
                        return anyhow::Result::Err(e.into());
                    }
                }
            }
            if tries > 0 {
                debug!(">> Trying again in 2s...");
                std::thread::sleep(std::time::Duration::from_millis(2000));
            } else {
                error!(db_url, "Failed to connect to database");
                bail!("Failed to connect to {} - 0 tries left", db_url);
            }
        }
    }

    fn build_db_url() -> anyhow::Result<String> {
        let host = pg_host();

        let env = std::env::vars().collect::<HashMap<String, String>>();

        let user = env
            .get("POSTGRES_USER")
            .context("POSTGRES_USER environmental variable not set or found")?;
        let password = env
            .get("POSTGRES_PASSWORD")
            .context("POSTGRES_PASSWORD environmental variable not set or found")?;
        let db_name = env
            .get("POSTGRES_DB")
            .context("POSTGRES_DB environmental variable not set or found")?;

        Ok(format!("postgres://{user}:{password}@{host}/{db_name}"))
    }

    pub async fn create_pool() -> anyhow::Result<PgPool> {
        match dotenv::dotenv() {
            Ok(path) => {
                info!("Env vars loaded from: {}", path.display())
            }
            Err(error) => {
                error!("Error using dotenv: {}", error.to_string());
            }
        }

        match build_db_url() {
            Ok(url) => {
                info!("env vars extracted");
                return try_connection(&url).await;
            }
            Err(e) => {
                error!("{}", e.to_string().as_str());
                bail!(e.to_string())
            }
        };
    }
}

pub mod shutdown {
    use anyhow::Context;
    use tokio::signal;

    pub async fn gracefull() {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .context("failed to install Ctrl+C handler")
                .unwrap();
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .context("failed to install signal handler")
                .unwrap()
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }
    }
}
