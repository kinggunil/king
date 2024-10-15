use king::*;

fn main() {
    let a = k_time();
    println!("{}", a);
    let a = k_microtime();
    println!("{}", a);
    let a = k_number_format(121000.1212, 2);
    println!("{}", a);
    let a = k_datetime(k_time());
    println!("{}", a);
    k_tic();
    k_sleep(1.2);
    k_toc();
    k_tic();
    let a = k_get_url("https://naver.com").unwrap();
    println!("{}", a[0..100].to_string());
    k_toc();

    let a = king::k_get_url("https://api.binance.com/api/v3/ticker/price");
    let b = a.unwrap();
    let c: serde_json::Value = serde_json::from_str(&b).unwrap();
    println!("{}", c[0]);
}
