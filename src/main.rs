use sqlx::Row;
use std::error::Error;

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3);";
    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;
    Ok(())
}
async fn update(book: &Book, isbn: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE book SET title = $1, author = $2 WHERE isbn = $3;";
    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&isbn)
        .execute(pool)
        .await?;
    Ok(())
}

async fn read(pool: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
    let q = "SELECT title, author, isbn FROM book";
    let query = sqlx::query(q);
    let rows = query.fetch_all(pool).await?;
    let books = rows
        .iter()
        .map(|row| Book {
            title: row.get("title"),
            author: row.get("author"),
            isbn: row.get("isbn"),
        })
        .collect();
    Ok(books)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://dev:123123@172.17.0.2:5432/dev";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    // let book = Book {
    //     title: String::from("Salem's Lot 3"),
    //     author: String::from("Stephen king"),
    //     isbn: String::from("978-0-385-00751-1"),
    // };
    // create(&book, &pool).await?;
    let books = read(&pool).await?;
    println!("{:?}", books);
    Ok(())
}
