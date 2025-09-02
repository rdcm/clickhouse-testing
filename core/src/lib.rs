use clickhouse::Row;
use dotenvy::dotenv;
use serde::Deserialize;
use std::fs::read_dir;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::{env, fs, io};

pub type Client = clickhouse::Client;
pub use clickhouse_testing_macros::test;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Env(std::env::VarError),
    Clickhouse(clickhouse::error::Error),
    Migration(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Error::Env(e)
    }
}

impl From<clickhouse::error::Error> for Error {
    fn from(e: clickhouse::error::Error) -> Self {
        Error::Clickhouse(e)
    }
}

pub async fn init_test(test_name: &str) -> Result<Client, Error> {
    _ = dotenv();

    let config = read_clickhouse_config();
    let client = create_client(&config);
    let databases = get_dbs_list(&client).await?;
    let db_name = next_db_version(&databases, test_name);

    create_database(&client, &db_name).await?;

    let test_client = client.with_database(db_name);
    apply_migrations(&test_client).await?;

    Ok(test_client)
}

pub async fn cleanup_test(client: &Client) -> Result<(), Error> {
    let current_db = get_current_db(client).await?;
    drop_db(client, &current_db).await?;

    Ok(())
}

async fn apply_migrations(client: &Client) -> Result<(), Error> {
    let migrations_path = env::var("MIGRATIONS_DIR")?;
    let project_root = get_project_root()?;

    let mut sql_files: Vec<_> = read_dir(project_root.join(migrations_path))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("sql"))
        .collect();

    sql_files.sort();

    for file in sql_files {
        let sql = fs::read_to_string(&file)?;
        client.query(&sql).execute().await?;
    }

    Ok(())
}

fn get_project_root() -> Result<PathBuf, Error> {
    let path = env::current_dir()?;

    for ancestor_path in path.ancestors() {
        let has_cargo = read_dir(ancestor_path)?.any(|p| p.unwrap().file_name() == "Cargo.lock");
        if has_cargo {
            return Ok(PathBuf::from(ancestor_path));
        }
    }

    Err(io::Error::new(ErrorKind::NotFound, "Cargo.lock not found").into())
}

fn create_client(config: &ClickhouseConfig) -> Client {
    Client::default()
        .with_url(&config.url)
        .with_database(&config.db)
        .with_user(&config.user)
        .with_password(&config.password)
}

fn read_clickhouse_config() -> ClickhouseConfig {
    ClickhouseConfig {
        url: env::var("CLICKHOUSE_URL").unwrap_or("http://localhost:8123".into()),
        db: env::var("CLICKHOUSE_DB").unwrap_or("default".into()),
        user: env::var("CLICKHOUSE_USER").unwrap_or("default".into()),
        password: env::var("CLICKHOUSE_PASSWORD").unwrap_or("".into()),
    }
}

async fn get_dbs_list(client: &Client) -> Result<Vec<Database>, Error> {
    let databases = client
        .query("SELECT name FROM system.databases")
        .fetch_all::<Database>()
        .await?;

    Ok(databases)
}

async fn create_database(client: &Client, db_name: &str) -> Result<(), Error> {
    let query = format!("CREATE DATABASE IF NOT EXISTS {}", db_name);
    client.query(&query).execute().await?;

    Ok(())
}

async fn get_current_db(client: &Client) -> Result<Database, Error> {
    let database = client
        .query("SELECT currentDatabase() AS name")
        .fetch_one::<Database>()
        .await?;

    Ok(database)
}

async fn drop_db(client: &Client, database: &Database) -> Result<(), Error> {
    client
        .query(&format!("DROP DATABASE {}", database.name))
        .execute()
        .await?;

    Ok(())
}

fn next_db_version(tests_dbs: &[Database], test_name: &str) -> String {
    let current_test_db = format!("test_db_{test_name}_");

    let db_version = tests_dbs
        .iter()
        .filter_map(|db| {
            db.name
                .strip_prefix(&current_test_db)?
                .parse::<usize>()
                .ok()
        })
        .max()
        .map(|v| v + 1)
        .unwrap_or(1);

    format!("{current_test_db}{db_version}")
}

#[derive(Debug, Deserialize, Row)]
struct Database {
    name: String,
}

#[derive(Debug)]
struct ClickhouseConfig {
    url: String,
    db: String,
    user: String,
    password: String,
}
