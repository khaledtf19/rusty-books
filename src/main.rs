use std::error::Error;
use sqlx::Row;

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

async fn read(pool: &sqlx::PgPool)-> Result<Book, Box<dyn Error>> {
    let q = "SELECT title, author, isbn FROM book";
    let query = sqlx::query(q);
    let row = query.fetch_one(pool).await?;
    let book = Book {
        title: row.get("title"),
        author: row.get("author"),
        isbn: row.get("isbn")
    };
    Ok(book)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://dev:123123@172.17.0.2:5432/dev";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    // let book = Book {
    //     title: String::from("Salem's Lot 2"),
    //     author: String::from("Stephen king"),
    //     isbn: String::from("978-0-385-00751-1"),
    // };
    // update(&book, &book.isbn, &pool).await?;
    let book = read(&pool).await?;
    println!("{}", book.title);
    Ok(())
}
