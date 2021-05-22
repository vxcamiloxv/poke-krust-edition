mod server;

use server::routes::orders::order_shoes;
use tide::Result;

#[async_std::main]
async fn main() -> Result<()> {
    let mut app = tide::new();
    app.at("/orders/shoes").post(order_shoes);
    app.listen("localhost:3000").await?;
    Ok(())
}
