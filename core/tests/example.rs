use rstest::rstest;

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

#[rstest]
#[case("SELECT toString(2 + 2)", "4")]
#[case("SELECT toString(10 * 5)", "50")]
#[case("SELECT 'hello'", "hello")]
#[clickhouse_testing::test]
async fn rstest_successfully_cases(
    #[ignore] client: clickhouse_testing::Client,
    #[case] query: &str,
    #[case] expected: &str,
) {
    let result: String = client.query(query).fetch_one().await.unwrap();

    assert_eq!(result, expected);
}
