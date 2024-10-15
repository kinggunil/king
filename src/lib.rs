
use rand::Rng;
pub fn k_rand(min: i64, max: i64) -> i64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

use std::thread;
use std::time::Duration;
pub fn k_sleep(seconds: f64) {
    let duration = Duration::from_secs_f64(seconds);
    thread::sleep(duration);
}

use std::time::{SystemTime, UNIX_EPOCH};
/// 현재 유닉스 타임을 초 단위로 반환
pub fn k_time() -> u64 {
    // 현재 시간
    let start = SystemTime::now();

    // 유닉스 에포크 이후의 경과 시간을 초 단위로 변환
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

/// 현재 유닉스 타임을 마이크로초 단위로 반환
pub fn k_microtime() -> u64 {
    // 현재 시간
    let start = SystemTime::now();
    // 유닉스 에포크 이후의 경과 시간을 마이크로초 단위로 변환하고 u64로 반환
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros() as u64
}

pub fn k_number_format<T: ToString>(num: T, decimals: usize) -> String {
    // 숫자를 문자열로 변환하고 형식화
    let formatted = format!("{:.*}", decimals, num.to_string().parse::<f64>().unwrap());
    let parts: Vec<&str> = formatted.split('.').collect();
    let int_part = parts[0]
        .chars()
        .rev()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(",")
        .chars()
        .rev()
        .collect::<String>();
    if parts.len() > 1 {
        format!("{}.{}", int_part, parts[1])
    } else {
        int_part
    }
}

use chrono::{Local, TimeZone, Utc};
/// 유닉스 타임(초 또는 마이크로초)을 로컬 시간으로 변환하여 "Y-m-d H:i:s" 형식으로 반환
pub fn k_datetime(unix_time: u64) -> String {
    // 유닉스 타임이 마이크로초인지 초 단위인지 확인
    let datetime = if unix_time > 1_000_000_000_000 {
        // 마이크로초 단위인 경우
        let secs = (unix_time / 1_000_000) as i64;
        let nanos = ((unix_time % 1_000_000) as u32) * 1_000;
        Utc.timestamp_opt(secs, nanos).unwrap()
    } else {
        // 초 단위인 경우
        Utc.timestamp_opt(unix_time as i64, 0).unwrap()
    };

    // 로컬 타임존으로 변환
    let local_time = datetime.with_timezone(&Local);

    // "Y-m-d H:i:s" 형식으로 변환
    local_time.format("%Y-%m-%d %H:%M:%S").to_string()
}


use std::time::Instant;
static mut START_TIME: Option<Instant> = None;

pub fn k_tic() {
    unsafe {
        START_TIME = Some(Instant::now());
    }
}

pub fn k_toc() {
    unsafe {
        if let Some(start) = START_TIME {
            let duration = start.elapsed();
            println!("\nElapsed time: {:?}\n", duration);
        } else {
            println!("Error: kson_tic() was not called.");
        }
        START_TIME = None;  // Reset START_TIME
    }
}

use reqwest::blocking;
use std::error::Error;

/// URL을 입력받아 해당 URL의 텍스트를 반환하는 동기 함수
pub fn k_get_url(url: &str) -> std::result::Result<String, Box<dyn Error>> {
    // 동기적으로 GET 요청을 보내고 응답의 텍스트를 가져옴
    let response = blocking::get(url)?.text()?;
    Ok(response)
}

/*
fn main() {
    let a=king::k_get_url("https://api.binance.com/api/v3/ticker/price");
    let b=a.unwrap();
    let c:serde_json::Value=serde_json::from_str(&b).unwrap();
    println!("{}",c[0]);//
}
 */


use sha2::{Sha256, Digest};
/// 입력값으로 `String`을 받아서 SHA-256 해시 값을 계산하는 함수
pub fn k_sha256(input: String) -> String {
    // 새로운 Sha256 해시 객체 생성
    let mut hasher = Sha256::new();

    // 입력 문자열을 해시 객체에 입력
    hasher.update(input);

    // 최종 해시 값 계산
    let result = hasher.finalize();

    // 해시 결과를 16진수로 변환하여 반환
    format!("{:x}", result)
}

use std::fs::File;
use std::io::{Write, Read};

/// 파일에 데이터를 쓰는 함수 (파일명과 데이터를 String 타입으로 받음)
pub fn k_write(file_name: String, data: String) -> std::io::Result<()> {
    // 파일을 생성하거나 열기 (쓰기 모드)
    let mut file = File::create(file_name)?;
    
    // 데이터를 파일에 쓰기
    file.write_all(data.as_bytes())?;
    
    Ok(())
}

/// 파일에서 데이터를 읽는 함수 (파일명을 String 타입으로 받음)
pub fn k_read(file_name: String) -> std::io::Result<String> {
    // 파일 열기 (읽기 모드)
    let mut file = File::open(file_name)?;
    
    // 파일 내용을 담을 String 생성
    let mut contents = String::new();
    
    // 파일에서 데이터를 읽어 contents에 저장
    file.read_to_string(&mut contents)?;
    
    Ok(contents)
}
