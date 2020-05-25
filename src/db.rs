use std::sync::Arc;
use tokio_postgres::{Client, Error};

use crate::models::Post;

pub type Db = Arc<DbContext>;

pub struct DbContext {
    client: Client
}

impl DbContext {
    pub async fn with_client(client: Client) -> Result<DbContext, Error> {
        DbContext::create_tables(&client).await?;

        Ok(DbContext {
            client,
        })
    }

    async fn create_tables(client: &Client) -> Result<(), Error> {
        Post::create_table(client).await
    }

    pub async fn insert_post(&self, p: &Post) -> Result<u64, Error>{
        p.insert(&self.client).await
    }

    pub async fn get_all_posts(&self) -> Result<Vec<Post>, Error> {
        let mut posts = Vec::new();
        for row in self.client.query("SELECT id, author, message FROM posts", &[]).await? {
            let t = Post::from_row(&row);
            posts.push(t);
        }
        Ok(posts)
    }
}