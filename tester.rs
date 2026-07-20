use rusqlite::{params, Connection, Result};
use std::time::Instant;
use rand::Rng; // 用來模擬隨機經緯度

pub fn init_db() -> Result<()> {
    let conn = Connection::open("satellite_data.db")?;
    // 這裡移除了 SQL 前綴
    conn.execute(
        "CREATE TABLE IF NOT EXISTS satellite_test (
            id INTEGER PRIMARY KEY,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            rtt_ms REAL,
            lat REAL,
            lon REAL
        )",
        [],
    )?;
    Ok(())
}

pub fn run_speed_test() -> Result<()> {
    let conn = Connection::open("satellite_data.db")?;
    let start = Instant::now();
    
    // 模擬網路延遲
    let mut rng = rand::thread_rng();
    let delay = rng.gen_range(100..800); // 模擬衛星延遲 100-800ms
    std::thread::sleep(std::time::Duration::from_millis(delay));

    let duration = start.elapsed().as_millis() as f64;
    
    // 模擬衛星當前位置
    let lat = rng.gen_range(-90.0..90.0);
    let lon = rng.gen_range(-180.0..180.0);

    conn.execute(
        "INSERT INTO satellite_test (rtt_ms, lat, lon) VALUES (?1, ?2, ?3)",
        params![duration, lat, lon],
    )?;
    
    Ok(())
}

pub fn scan_available_satellites(candidates: Vec<&str>) -> Option<String> {
    let mut best_sat = None;
    let mut min_rtt = f64::MAX;
    let mut rng = rand::thread_rng();

    println!("--- 開始掃描周邊衛星訊號 ---");
    for sat in candidates {
        // 模擬 Ping 測試過程
        let start = Instant::now();
        let delay = rng.gen_range(100..800); 
        std::thread::sleep(std::time::Duration::from_millis(delay));
        let rtt = start.elapsed().as_millis() as f64;

        println!("偵測到衛星 {}，延遲: {}ms", sat, rtt);

        if rtt < min_rtt {
            min_rtt = rtt;
            best_sat = Some(sat.to_string());
        }
    }
    match best_sat {
        Some(ref sat) => println!(">>> 鎖定最佳衛星: {} (延遲: {}ms)", sat, min_rtt),
        None => println!(">>> [警告] 未發現任何可用衛星"),
    }
    
    best_sat // 這裡就是關鍵：回傳 Option<String>
}