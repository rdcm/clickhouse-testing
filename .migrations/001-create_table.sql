CREATE TABLE test_data (
    data String
)
ENGINE = MergeTree()
ORDER BY (data);