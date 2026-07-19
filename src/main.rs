use std::{thread, time::Duration, collections::VecDeque};
use rand::{thread_rng, Rng};

#[allow(dead_code)]
struct Message {
    id: u32,
    content: String,
    retries: u32,
}

fn main() {
    let mut queue: VecDeque<Message> = VecDeque::new();
    let mut rng = thread_rng();

    // 建立 5 則訊息
    for i in 1..=5 {
        queue.push_back(Message { id: i, content: format!("訊息 {}", i), retries: 0 });
    }

    println!("--- 衛星網路模擬啟動 (惡劣天氣模式) ---");

    const MAX_RETRIES: u32 = 3; // 設定重試上限

    while !queue.is_empty() {
        let mut msg = queue.pop_front().unwrap();

        if rng.gen_bool(0.6) { // 模擬失敗情況
            msg.retries += 1;
        
            if msg.retries > MAX_RETRIES {
                println!("[錯誤] 訊息 {} 超過最大重試次數，已棄置 (Dead Letter)", msg.id);
                continue; 
            }

            println!("[失敗] 訊息 {} 發送失敗 (重試: {})，等待重試...", msg.id, msg.retries);
            
            // 這裡稍作調整：我們讓它在重試前等待，然後放回最前面
            thread::sleep(Duration::from_secs(2)); 
            queue.push_front(msg); 
        
        } else {
            println!("[成功] 訊息 {} ({}) 已送達！", msg.id, msg.content);
        }
    }
    
    println!("--- 模擬結束 ---");
}