use std::path::PathBuf;

use chrono::Utc;
use directories::ProjectDirs;
use sqlx::{Row, SqlitePool};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::data::{AccountKind, MinecraftAccount};
use crate::error::{AuthError, AuthResult};

/// SQLite-backed credentials store for offline accounts.
///
/// Accounts are persisted to `auth.db` in the launcher data directory.
pub struct CredentialsStore {
    pool: SqlitePool,
    default_user: Mutex<Option<Uuid>>,
}

impl CredentialsStore {
    /// Creates a new credentials store, initializing the database if needed.
    pub async fn new() -> AuthResult<Self> {
        let data_dir = Self::data_dir()?;
        tokio::fs::create_dir_all(&data_dir).await?;

        let db_path = data_dir.join("auth.db");
        let pool = SqlitePool::connect(&format!("sqlite://{}?mode=rwc", db_path.display())).await?;

        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;

        let default_user = Self::load_default_user(&pool).await?;

        Ok(Self {
            pool,
            default_user: Mutex::new(default_user),
        })
    }

    /// Gets the launcher data directory.
    fn data_dir() -> AuthResult<PathBuf> {
        ProjectDirs::from("com", "tcc", "launcher")
            .map(|dirs| dirs.data_dir().to_path_buf())
            .ok_or(AuthError::DataDirNotFound)
    }

    fn account_from_row(row: &sqlx::sqlite::SqliteRow) -> MinecraftAccount {
        let expires: i64 = row.get("expires");
        let _kind: String = row.get("kind");
        MinecraftAccount {
            id: Uuid::from_bytes(
                row.get::<Vec<u8>, _>("id")
                    .try_into()
                    .expect("account id is a 16-byte UUID"),
            ),
            username: row.get("username"),
            access_token: row.get::<Option<String>, _>("access_token").unwrap_or_default(),
            refresh_token: row
                .get::<Option<String>, _>("refresh_token")
                .unwrap_or_default(),
            expires: chrono::DateTime::from_timestamp(expires, 0).unwrap_or_else(Utc::now),
            kind: AccountKind::Offline,
        }
    }

    /// Loads the default user ID from the database.
    async fn load_default_user(pool: &SqlitePool) -> AuthResult<Option<Uuid>> {
        let row = sqlx::query("SELECT user_id FROM default_user LIMIT 1")
            .fetch_optional(pool)
            .await?;

        Ok(row.map(|r| {
            let bytes: Vec<u8> = r.get("user_id");
            Uuid::from_bytes(bytes.try_into().expect("user id is a 16-byte UUID"))
        }))
    }

    /// Lists all accounts in the store.
    pub async fn list_accounts(&self) -> Vec<MinecraftAccount> {
        sqlx::query("SELECT id, username, access_token, refresh_token, expires, kind FROM accounts ORDER BY username")
            .fetch_all(&self.pool)
            .await
            .map(|rows| rows.iter().map(Self::account_from_row).collect())
            .unwrap_or_default()
    }

    /// Gets a specific account by ID.
    pub async fn get_account(&self, id: Uuid) -> Option<MinecraftAccount> {
        sqlx::query("SELECT id, username, access_token, refresh_token, expires, kind FROM accounts WHERE id = ?")
            .bind(id.as_bytes())
            .fetch_optional(&self.pool)
            .await
            .ok()
            .flatten()
            .map(|row| Self::account_from_row(&row))
    }

    /// Adds an offline account and saves it.
    pub async fn add_offline_account_and_save(&self, username: String) -> AuthResult<MinecraftAccount> {
        let account = super::offline::offline_account(username);
        self.add_account(&account).await?;
        Ok(account)
    }

    /// Adds an account to the database.
    async fn add_account(&self, account: &MinecraftAccount) -> AuthResult<()> {
        sqlx::query(
            "INSERT INTO accounts (id, username, access_token, refresh_token, expires, kind) \
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(account.id.as_bytes())
        .bind(&account.username)
        .bind(&account.access_token)
        .bind(&account.refresh_token)
        .bind(account.expires.timestamp())
        .bind("offline")
        .execute(&self.pool)
        .await?;

        // If this is the first account, set it as default
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM accounts")
            .fetch_one(&self.pool)
            .await?;

        if count == 1 {
            self.set_default_user(Some(account.id)).await?;
        }

        Ok(())
    }

    /// Removes an account.
    pub async fn remove_account(&self, id: Uuid) -> AuthResult<()> {
        sqlx::query("DELETE FROM accounts WHERE id = ?")
            .bind(id.as_bytes())
            .execute(&self.pool)
            .await?;

        // If we removed the default user, clear it
        let mut default = self.default_user.lock().await;
        if *default == Some(id) {
            *default = None;
            sqlx::query("DELETE FROM default_user")
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }

    /// Sets the default user.
    pub async fn set_default_user(&self, id: Option<Uuid>) -> AuthResult<()> {
        let mut default = self.default_user.lock().await;
        *default = id;

        if let Some(id) = id {
            sqlx::query("INSERT OR REPLACE INTO default_user (user_id) VALUES (?)")
                .bind(id.as_bytes())
                .execute(&self.pool)
                .await?;
        } else {
            sqlx::query("DELETE FROM default_user")
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }

    /// Gets the default account.
    pub async fn default_account(&self) -> AuthResult<Option<MinecraftAccount>> {
        let default = self.default_user.lock().await;
        if let Some(id) = *default {
            return Ok(self.get_account(id).await);
        }
        Ok(None)
    }

    /// Resolves the default account ID.
    pub async fn resolve_default_id(&self) -> AuthResult<Option<Uuid>> {
        let default = self.default_user.lock().await;
        Ok(*default)
    }

    /// Commits an account (for compatibility with auth service interface).
    pub async fn commit_account(&self, account: MinecraftAccount, _events: &tcc_events::EventBus) -> AuthResult<()> {
        self.add_account(&account).await
    }

    /// Commits a refreshed account (no-op for offline accounts).
    pub async fn commit_refreshed_account(&self, account: MinecraftAccount) -> AuthResult<()> {
        self.add_account(&account).await
    }
}
