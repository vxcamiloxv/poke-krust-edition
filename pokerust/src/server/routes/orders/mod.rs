use tide::{Request, Result};
use crate::server::models::animal::Animal;

pub async fn order_shoes(mut req: Request<()>) -> Result {
    let Animal { name, legs } = req.body_json().await?;
    println!("Data: {}, {}", name, legs);
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}
