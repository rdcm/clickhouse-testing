# About

A crate that simplifies writing integration tests for ClickHouse, inspired by sqlx.

# Getting Started

Set `CONFIG_PATH` to a JSON file (relative to the test package directory):

```json
{
  "db": {
    "url": "http://localhost:8123",
    "db": "db",
    "user": "user",
    "password": "password"
  },
  "migration_source": ".migrations"
}
```

For example, run `CONFIG_PATH=../../config/test/migrator.json cargo test`.
`CLICKHOUSE_URL`, `CLICKHOUSE_DB`, `CLICKHOUSE_USER`, `CLICKHOUSE_PASSWORD`, and
`MIGRATIONS_DIR` override the corresponding JSON values when set. A missing or
invalid `CONFIG_PATH` file fails test setup without falling back to `.env`.

Without `CONFIG_PATH`, the library retains its original `.env` behavior. Set
the following variables there:

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

# Behavior

- Each test creates a unique database in the ClickHouse instance. Unique key: `{module_name}` + `{test_name}` + `{run_id}`.
- Before each test, the new schema is applied to `"test_db"`.
- After successful test execution, `"test_db"` is deleted. If any error occurs, `"test_db"` is preserved for investigation.

Enjoy!
