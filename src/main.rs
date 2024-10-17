use king::*;
use serde_json::*;
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

    let a = k_sha256("hello world".to_string());
    println!("{}", a);
    println!("{:#?}", a);

    let a = k_write("test.txt", a);
    match a {
        Ok(()) => println!("File written successfully!"),
        Err(e) => println!("Failed to write to file: {}", e),
    }

    let b = k_read("test.txt").unwrap();
    println!("{}", b);

    kset!(a);
    kset!(a["name"]="kinggunil");
    kset!(a["age"]=20);
    println!("{:#?}", a);
    
    kset!(b);
    kset!(b[0]="test0");
    kset!(b[1]="test1");
    kset!(b[2]="test2");
    println!("{:#?}", b);

    kset!(c);
    kset!(c["age"]["korean"]=20);
    kset!(c["age"]["usa"]=18);
    kset!(c["age"]["test"]["aa"][0]=77);
    println!("{:#?}", c);

    kset!(d);
    kset!(d[0][0]=20);
    kset!(d[0][1]=30);
    kset!(d[1][0]=40);
    kset!(d[1][1]=50);
    println!("{:#?}", d);


    kset!(ab);
    kset!(ab["data"] = "test");
    println!("{:#?}", ab["data"]);
    println!("*test : {:#?}", kget!(ab["data"] => String));
    


    let json=r#"
    [
    [
        1729135620000,
        "67537.51000000",
        "67557.51000000",
        "67532.00000000",
        "67557.51000000",
        "5.31780000",
        1729135679999,
        "359153.94242100",
        1206,
        "3.90923000",
        "264023.00093650",
        "0"
    ],
    [
        1729135680000,
        "67557.51000000",
        "67589.98000000",
        "67547.63000000",
        "67585.53000000",
        "11.46317000",
        1729135739999,
        "774529.23389550",
        1325,
        "8.21896000",
        "555303.87791390",
        "0"
    ]
    ]"#;
    let json2:Value=serde_json::from_str(json).unwrap();
    kset!(e);
    kset!(e["data"]=json2);
    println!("{:#?}",e);
    println!("{:#?}",kget!(e["data"][0][0] => i64));

    println!("{:#?}",kget!(b[0] => String));
    println!("{:#?}",kget!(b[0] => String));
    println!("{:#?}",kget!(a["age"] => String));
    println!("{:#?}",10000+kget!(c["age"]["test"]["aa"][0] => i32));
    println!("{:#?}",kget!(d[0][1] => i64));
    println!("{:#?}",kget!(d[0][1] => i64));
/*
    결과값
    Object {
        "age": Number(20),
        "name": String("kinggunil"),
    }
    Array [
        String("test0"),
        String("test1"),
        String("test2"),
    ]
    Object {
        "age": Object {
            "korean": Number(20),
            "usa": Number(18),
        },
    }

kget!(b[0] => String) 으로 하면 String 타입으로 test0가 출력되고
kget!(a["age"]=>i64)로 하면 i64로 20이 출력되고
kget!(c["age"]["usa"]=>i64)로 하면 i64로 18이 출력되는 
kget!을 하면 어떤 타입이 오던 무조건적으로 =>뒤의 타입으로 강제 변환될수 있게 해줘    
kget! 메크로 작성해줘

*/
}
