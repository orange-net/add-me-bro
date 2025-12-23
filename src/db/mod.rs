use sea_orm::{Database, DatabaseConnection, DbErr};

#[derive(Clone)]
pub struct DBPool {
    pub conn_str: String,
    pub db: DatabaseConnection,
}

impl DBPool {
    pub async fn new(conn_str: String) -> Result<Self, DbErr> {
        tracing::debug!(
            "Initializing database connection pool with conn string {}",
            conn_str
        );

        let db = Database::connect(&conn_str).await?;
        db.ping().await?;

        tracing::info!(
            "Successfully established connection to database {}",
            conn_str
        );

        let pool = DBPool { conn_str, db };
        Ok(pool)
    }

    pub async fn ping(&self) -> bool {
        self.db.ping().await.is_ok()
    }
}

pub struct DBPoolBuilder {
    host: String,
    port: String,
    db_name: String,
    username: String,
    password: String,
}

#[allow(unused)]
impl DBPoolBuilder {
    pub fn host(&mut self, host: String) -> &mut Self {
        self.host = host;
        self
    }

    pub fn port(&mut self, port: u32) -> &mut Self {
        self.port = port.to_string();
        self
    }

    pub fn username(&mut self, username: String) -> &mut Self {
        self.username = username;
        self
    }

    pub fn password(&mut self, pass: String) -> &mut Self {
        self.password = pass;
        self
    }

    pub fn db_name(&mut self, db_name: String) -> &mut Self {
        self.db_name = db_name;
        self
    }

    pub fn default() -> Self {
        DBPoolBuilder {
            host: "localhost".into(),
            port: "5432".into(),
            username: "postgres".into(),
            db_name: "add-me-bro".into(),
            password: "postgres".into(),
        }
    }

    fn build_conn_string(&self) -> String {
        // builds pg connection string
        //"postgres://postgres:password@localhost/test"
        let conn_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.db_name
        );
        conn_string
    }

    pub async fn build(&self) -> Result<DBPool, DbErr> {
        let pool = DBPool::new(self.build_conn_string()).await?;
        Ok(pool)
    }
}
