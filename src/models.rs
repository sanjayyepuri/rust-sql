use tokio_postgres::{Client, Error, Row};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    pub id: i32,
    pub author: String, 
    pub message: String,
}

impl Post {
    pub async fn create_table(db: &Client) -> Result<(), Error> { 
        db.batch_execute("
            CREATE TABLE IF NOT EXISTS posts (
                id      SERIAL PRIMARY KEY,
                author  TEXT,
                message TEXT
            )
        ").await
    }

    pub fn new(author: String, message: String) -> Post {
        Post {
            id: -1,
            author, 
            message
        }
    } 

    pub fn from_row(row: &Row) -> Post { 
        Post {
            id: row.get(0),
            author: row.get(1),
            message: row.get(2),
        }
    }

    pub async fn insert(&self, db: &Client) -> Result<u64, Error> {
        db.execute("INSERT INTO posts (author, message) VALUES ($1, $2)", 
            &[&(self.author), &(self.message)]).await
    }

    pub async fn delete(&self, db: &Client) -> Result<u64, Error> {
        db.execute("DELETE FROM posts WHERE id=$1", &[&(self.id)]).await
    }
}
