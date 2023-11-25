use std::error::Error;
use sqlx::Row;

pub struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String
}

async fn create(book: &Book, pool: &sqlx::PgPool)-> Result<(), Box<dyn Error>>{
    let query = "ISERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";
    sqlx::query(query).bind(&book.title).bind(&book.author).bind(&book.isbn).execute(pool).await?;
    Ok(()) 
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://dev:123123@172.17.0.2:5432/dev";
    let conn = sqlx::postgres::PgPool::connect(url).await?;
    
    sqlx::migrate!("./migrations").run(&conn).await?;

    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&conn)
        .await?;
        
    let sum:i32 = res.get("sum") ;
    println!("1+1 = {}", sum);
    Ok(())
}
