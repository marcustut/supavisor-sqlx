#[tokio::main]
async fn main() {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect("...")
        .await
        .unwrap();

    sqlx::query!("select * from public.user_metadata");

    println!("Hello, world!");
}
