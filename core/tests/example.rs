#[clickhouse_testing::test]
async fn test1(client: clickhouse_testing::Client) {
    let version = client
        .query("SELECT version()")
        .fetch_one::<String>()
        .await
        .unwrap();

    assert_eq!(version, "25.5.2.47");
}

#[clickhouse_testing::test]
async fn test2(client: clickhouse_testing::Client) {
    let version = client
        .query("SELECT version()")
        .fetch_one::<String>()
        .await
        .unwrap();

    assert_eq!(version, "25.5.2.47");
}
