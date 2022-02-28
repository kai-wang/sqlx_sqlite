#![allow(dead_code)]
#![allow(unused)]

use sqlx::sqlite::{SqlitePool, SqliteRow};
use sqlx::{FromRow, Row};

#[derive(Debug, FromRow)]
struct Contact {
  contact_id: i64,
  name: String
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
  // connect the test.db from current directory
  let pool = SqlitePool::connect("sqlite://test.db").await?;
  
  // Create a table if not existing
  sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS contacts (
      contact_id INTEGER PRIMARY KEY,
      name TEXT NOT NULL
    );"#
  )
  .execute(&pool)
  .await?;

  // insert some new data
  let row: (i64, ) = sqlx::query_as("insert into contacts (name) values ($1) returning contact_id")
      .bind("JamesBond")
      .fetch_one(&pool)
      .await?;

  // Query 1 - fetch all records;
  let rows = sqlx::query("SELECT * FROM contacts")
      .fetch_all(&pool)
      .await?;

  let str_result = rows
      .iter()
      .map(|r| format!("{} - {}", r.get::<i64, _>("contact_id"), r.get::<String, _>("name")))
      .collect::<Vec<String>>()
      .join(", ");

  println!("\n fetch all results \n {} ", str_result);

  // Query 2 - select query with map()
  let query = sqlx::query("SELECT contact_id, name FROM contacts");
  let contacts: Vec<Contact> = query
      .map(|row: SqliteRow| Contact {
          contact_id: row.get("contact_id"),
          name: row.get("name")
      })
      .fetch_all(&pool)
      .await?;

  println!("\n select contacts from query.map...\n{:?}", contacts);

  // Query 3 - Select query_as (using derive FromRow)

  let select_query = sqlx::query_as::<_, Contact>("SELECT contact_id, name FROM contacts");
  let contacts: Vec<Contact> = select_query.fetch_all(&pool).await?;
  println!("\n=== select contacts with query.map... \n{:?}", contacts);

  Ok(())
}
