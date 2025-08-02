use sqlx::{postgres::PgPoolOptions, PgPool, Postgres, Transaction, Executor};

/// Create a test pool, a unique schema for isolation, set search_path, and run migrations.
/// Uses DATABASE_URL or defaults to postgres://postgres:postgres@127.0.0.1:5432/cpc
pub async fn test_pool_with_schema() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432/cpc".to_string());

    // Connect (not lazy) so we can execute DDL and migrations.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("connect test db");

    // We create a unique schema name using pg backend pid to avoid collisions in parallel.
    // Using a random suffix from gen_random_uuid() if pgcrypto available; fallback to pid.
    let mut conn = pool.acquire().await.expect("acquire conn");
    let schema: (String,) = sqlx::query_as(
        "SELECT 'test_schema_' || to_char(clock_timestamp(), 'YYYYMMDDHH24MISSMS') || '_' || pg_backend_pid()",
    )
    .fetch_one(&mut *conn)
    .await
    .expect("compute schema name");
    let schema_name = schema.0;

    // Create the schema and set search_path
    let create = format!("CREATE SCHEMA {};", schema_name);
    conn.execute(create.as_str()).await.expect("create schema");
    let set_path = format!("SET search_path TO {}, public;", schema_name);
    conn.execute(set_path.as_str()).await.expect("set search_path");

    // Run migrations relative to this crate (cooperative_fundraising)
    // The macro path is set explicitly to the package-local migrations dir.
    sqlx::migrate!("./migrations")
        .run(&mut *conn)
        .await
        .expect("apply migrations");

    // Stash the schema name in a temp table for later retrieval by transactions that may come via a different connection.
    conn.execute(
        "CREATE TEMP TABLE _test_schema (name text) ON COMMIT PRESERVE ROWS;",
    )
    .await
    .ok();
    let ins = format!("INSERT INTO _test_schema(name) VALUES ('{}');", schema_name);
    conn.execute(ins.as_str()).await.ok();

    pool
}

/// A test transaction helper that ensures the correct search_path is set and rolls back at the end.
/// Call rollback() at the end of the test. Dropping without commit in sqlx will also roll back.
pub struct TestTx<'a> {
    pub tx: Option<Transaction<'a, Postgres>>,
}

impl<'a> TestTx<'a> {
    pub async fn new(pool: &'a PgPool) -> Self {
        let mut tx = pool.begin().await.expect("begin tx");

        // Attempt to read schema name from temp table if exists; otherwise, select latest created test_schema_*.
        let schema_row: Option<(String,)> = sqlx::query_as(
            "SELECT name FROM _test_schema LIMIT 1",
        )
        .fetch_optional(&mut *tx)
        .await
        .ok();

        let schema_name = if let Some((name,)) = schema_row {
            name
        } else {
            // Fallback: pick the most recently created test schema by oid
            let (name,): (String,) = sqlx::query_as(
                "SELECT nspname FROM pg_namespace WHERE nspname LIKE 'test_schema_%' ORDER BY oid DESC LIMIT 1",
            )
            .fetch_one(&mut *tx)
            .await
            .expect("find latest test schema");
            name
        };

        let set_path = format!("SET search_path TO {}, public;", schema_name);
        sqlx::query(&set_path).execute(&mut *tx).await.expect("set search_path in tx");

        Self { tx: Some(tx) }
    }

    pub async fn rollback(&mut self) {
        if let Some(tx) = self.tx.take() {
            tx.rollback().await.expect("rollback");
        }
    }
}