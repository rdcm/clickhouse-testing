# About

A crate that simplifies writing integration tests for ClickHouse, inspired by sqlx.

# Getting Started

Add the following environment variables to a `.env` file:

```bash
MIGRATIONS_DIR=".migrations"
CLICKHOUSE_URL="http://localhost:8123"
CLICKHOUSE_DB="db"
CLICKHOUSE_USER="user"
CLICKHOUSE_PASSWORD="password"
```

Write a test:

```rust
#[clickhouse_testing::test]
async fn test1(client: clickhouse_testing::Client) {
    let version = client
        .query("SELECT version()")
        .fetch_one::<String>()
        .await
        .unwrap();

    assert_eq!(version, "25.5.2.47");
}
```

# Guaranties

- Each test creates a unique database in the ClickHouse instance. Unique key: `{module_name}` + `{test_name}` + `{run_id}`.
- Before each test, the new schema is applied to `"test_db"`.
- After successful test execution, `"test_db"` is deleted. If any error occurs, `"test_db"` is preserved for investigation.

Enjoy!