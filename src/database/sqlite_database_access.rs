use async_trait::async_trait;
use sqlx::{pool::PoolConnection, query::Query, sqlite::SqliteArguments, Sqlite};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::data::db_types::session_row::insert_streaks;
use crate::data::db_types::{SessionRow, StreakRow};
use crate::data::Streak;

use super::DatabaseAccess;

#[derive(Debug, Clone)]
pub struct SqliteDatabaseAccess {
    database_connection: Arc<Mutex<PoolConnection<Sqlite>>>,
}

impl SqliteDatabaseAccess {
    pub fn new(database_connection: PoolConnection<Sqlite>) -> Self {
        Self {
            database_connection: Arc::new(Mutex::new(database_connection)),
        }
    }
}

#[async_trait]
impl DatabaseAccess for SqliteDatabaseAccess {
    async fn create_session(&self, interval: chrono::Duration) -> Result<i64, ()> {
        let mut conn = self.database_connection.lock().await;
        let res: Result<_, _> =
            sqlx::query("INSERT INTO table_session (sample_interval_s) VALUES (?)")
                .bind(interval.num_seconds())
                .execute(&mut *conn)
                .await;
        match res {
            Ok(r) => Ok(r.last_insert_rowid()),
            Err(_) => Err(()),
        }
    }
    async fn save_streak(&self, streak: &Streak, session_id: i64) -> Result<i64, ()> {
        let mut conn = self.database_connection.lock().await;
        let insert_streak_query: Query<'_, Sqlite, SqliteArguments<'_>> = sqlx::query::<Sqlite>("INSERT INTO table_streak (process_name, website_name, begin_date, duration_s) VALUES (?, ?, ?, ?)")
            .bind(&streak.process_name.0)
            .bind(&streak.website_name)
            .bind(streak.begin_date.timestamp())
            .bind(streak.duration.num_seconds());
        let streak_id = insert_streak_query
            .execute(&mut *conn)
            .await
            .expect("Insertion in streak table failed")
            .last_insert_rowid();

        sqlx::query::<Sqlite>("INSERT INTO session_streak (session_id, streak_id) VALUES (?, ?)")
            .bind(session_id)
            .bind(streak_id)
            .execute(&mut *conn)
            .await
            .expect("Insertion in join table failed");
        Ok(streak_id)
    }

    async fn read_streak(&self, id: i64) -> Result<StreakRow, ()> {
        let mut conn = self.database_connection.lock().await;
        match sqlx::query_as::<Sqlite, StreakRow>("SELECT * FROM table_streak WHERE id = ?")
            .bind(id)
            .fetch_one(&mut *conn)
            .await
        {
            Ok(r) => Ok(r),
            Err(_) => Err(()),
        }
    }

    async fn read_session(&self, id: i64) -> Result<SessionRow, ()> {
        let mut conn = self.database_connection.lock().await;
        let mut session =
            match sqlx::query_as::<Sqlite, SessionRow>("SELECT * FROM table_session WHERE id = ?")
                .bind(id)
                .fetch_one(&mut *conn)
                .await
            {
                Ok(r) => r,
                Err(_) => return Err(()),
            };
        let streaks = match sqlx::query_as::<Sqlite, StreakRow>("SELECT * from table_streak, session_streak where session_streak.session_id = ?
                                                                     AND session_streak.streak_id = table_streak.id").bind(id).fetch_all(&mut *conn).await {
                                                                         Ok(r) => r,
                                                                         Err(_) => return Err(())
                                                                     };

        insert_streaks(&mut session, streaks);
        Ok(session)
    }
}

#[cfg(test)]
pub mod tests {
    use sqlx::{sqlite, Row, SqlitePool};
    use tempfile::{tempdir, TempDir};

    use crate::aggregator::session::Session;
    use crate::data::wrappers::{ProcessName, WebsiteName};
    use crate::database::init::apply_migrations;

    use super::*;
    use std::collections::HashSet;

    pub async fn create_tmp_db() -> (TempDir, SqlitePool) {
        let temp_dir = tempdir().expect("Could not create temporary file");
        let mut path = temp_dir.path().to_path_buf();
        path.push("tmp_db.sqlite");
        let connect_options = sqlite::SqliteConnectOptions::new()
            .filename(&path)
            .create_if_missing(true);
        (
            temp_dir,
            SqlitePool::connect_with(connect_options).await.unwrap(),
        )
    }

    pub fn get_database_access(conn: PoolConnection<Sqlite>) -> SqliteDatabaseAccess {
        SqliteDatabaseAccess::new(conn)
    }

    pub fn build_some_streak(
        duration: chrono::Duration,
        pid: i32,
        process_name: &str,
        website_name: Option<WebsiteName>,
    ) -> Streak {
        Streak {
            begin_date: chrono::Local::now(),
            duration,
            pid,
            process_name: ProcessName {
                0: process_name.to_owned(),
            },
            website_name,
            window_names: HashSet::default(),
        }
    }
    pub fn build_some_session(sample_interval: chrono::Duration) -> Session {
        Session {
            session_id: 0,
            sample_interval,
        }
    }

    #[tokio::test]
    pub async fn test_save_streak() {
        let streak_id = 100;
        let (_temp_dir, pool) = create_tmp_db().await;
        apply_migrations(&pool).await.unwrap();
        let db_access = get_database_access(pool.acquire().await.unwrap());
        let s = build_some_streak(chrono::Duration::seconds(20), 10, "toto", None);
        let streak_id = db_access.save_streak(&s, streak_id).await.unwrap();

        let query_result = sqlx::query("SELECT id from table_streak")
            .fetch_all(&mut pool.acquire().await.unwrap())
            .await
            .unwrap();
        assert_eq!(query_result.len(), 1);

        let read_streak_id: i64 = query_result.first().unwrap().try_get("id").unwrap();
        assert_eq!(read_streak_id, streak_id);

        let tmp_connection = &mut pool.acquire().await.unwrap();
        let query_join_table = sqlx::query("SELECT * from session_streak")
            .fetch_all(tmp_connection)
            .await
            .unwrap();
        assert_eq!(query_join_table.len(), 1);
    }

    #[tokio::test]
    pub async fn test_read_streak() {
        let (_temp_dir, pool) = create_tmp_db().await;
        let sampling_interval = chrono::Duration::seconds(2);
        apply_migrations(&pool).await.unwrap();
        let db_access = get_database_access(pool.acquire().await.unwrap());
        let session = build_some_session(sampling_interval);
        let streak = build_some_streak(sampling_interval, 20, "tata", None);
        let streak_id = db_access
            .save_streak(&streak, session.session_id)
            .await
            .unwrap();

        // Test
        let read_streak = db_access.read_streak(streak_id).await.unwrap();

        // Check output
        assert_eq!(read_streak.id, streak_id);
        assert_eq!(read_streak.process_name, streak.process_name);
        assert_eq!(read_streak.website_name, streak.website_name);
        assert_eq!(read_streak.duration, streak.duration);
    }
    #[tokio::test]
    pub async fn test_create_session() {
        let (_temp_dir, pool) = create_tmp_db().await;
        let sampling_interval = chrono::Duration::seconds(2);
        apply_migrations(&pool).await.unwrap();
        let db_access = get_database_access(pool.acquire().await.unwrap());

        let session_id = db_access.create_session(sampling_interval).await;
        assert!(session_id.is_ok());
    }
}
