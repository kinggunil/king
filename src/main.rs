
fn main() {
    let a=king::k_get_url("https://api.binance.com/api/v3/ticker/price");
    let b=a.unwrap();
    let c:serde_json::Value=serde_json::from_str(&b).unwrap();
    println!("{}",c[0]);
}
