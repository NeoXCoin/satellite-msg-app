mod tester;
use std::{thread, time::Duration, collections::VecDeque, fs::OpenOptions, io::Write};
use rand::{thread_rng, Rng};

#[derive(PartialEq, Debug)]
enum NetworkMode { Satellite, GroundStation }

struct Message { id: u32, retries: u32 }

// 新增：寫入 CSV 函數
fn log_to_csv(id: u32, status: &str, sat: &str, mode: &str) {
    let mut file = OpenOptions::new()
        .create(true).append(true).open("simulation_results.csv")
        .expect("無法開啟日誌檔案");
    writeln!(file, "{},{},{},{}", id, status, sat, mode).expect("寫入失敗");
}

fn main() {
    // 建立/清空 CSV 標題
    let _ = std::fs::write("simulation_results.csv", "ID,STATUS,SATELLITE,MODE\n");

    let _ = tester::init_db();
    let available_sats = vec!["STARLINK_01", "ONEWEB_02", "IRIS_03"];
    let mut current_best_sat = tester::scan_available_satellites(available_sats.clone()).unwrap_or("NONE".to_string());
    
    let mut queue: VecDeque<Message> = VecDeque::new();
    for i in 1..=20 { queue.push_back(Message { id: i, retries: 0 }); }

    let mut time_step = 0;
    let mut rng = thread_rng();

    while !queue.is_empty() {
        let mut msg = queue.pop_front().unwrap();
        time_step += 1;

        let mode = if time_step > 5 && time_step <= 12 { NetworkMode::GroundStation } else { NetworkMode::Satellite };

        match mode {
            NetworkMode::GroundStation => {
                println!("[地面模式] 訊息 {} 送達", msg.id);
                log_to_csv(msg.id, "SUCCESS", "GROUND", "GROUND");
            }
            NetworkMode::Satellite => {
                let failure_rate = 0.3 + (time_step as f64 * 0.1);
                if rng.gen_bool(failure_rate.min(0.8)) {
                    msg.retries += 1;
                    if msg.retries >= 2 {
                        if let Some(new_sat) = tester::scan_available_satellites(available_sats.clone()) {
                            current_best_sat = new_sat;
                            log_to_csv(msg.id, "SWITCHED", &current_best_sat, "SATELLITE");
                        }
                    }
                    // ... 在處理 Satellite 失敗的邏輯區塊內 ...
                    if msg.retries > 3 {
                        println!("[系統警告] 訊號極度不穩，進入 2 秒冷卻等待...");
                        thread::sleep(Duration::from_secs(2)); // 強制冷卻 2 秒
    
                        log_to_csv(msg.id, "FAILED", &current_best_sat, "SATELLITE");
                        // 不用棄置，繼續嘗試，或者在這裡將重試次數歸零
                        msg.retries = 0; 
                        queue.push_front(msg);
                    }
                } else {
                    println!("[成功] 訊息 {} 送達", msg.id);
                    log_to_csv(msg.id, "SUCCESS", &current_best_sat, "SATELLITE");
                }
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
    println!("--- 模擬結束，數據已儲存至 simulation_results.csv ---");
}