# satellite-msg-app
A resilient messaging application designed for high-latency, unstable satellite network conditions.
satellite-msg-app is more than just a messaging tool; it is an experiment in "resilient communication." We are committed to overcoming high-latency, unstable satellite network conditions to ensure that every message and every game command reaches its destination, even in extreme physical environments.
Key Features 
Resilient Design：Offline-first architecture, automated retries upon failure, ensuring data integrity under extreme conditions.
Low-Bandwidth Optimization：Extreme compression techniques to allow data transmission even in low-bandwidth environments.
Open Architecture：An open architecture designed for future gaming interactions and message synchronization, allowing you to interact freely from anywhere in the universe.

在網路覆蓋的邊緣，在訊號被惡劣天氣吞噬的瞬間，我們依然需要保持連結。
不僅僅是一個訊息傳送工具，它是一項關於「韌性通訊」的實驗。我們致力於克服高延遲、不穩定的衛星網路環境，確保每一條訊息、每一個遊戲指令，都能在極端的物理條件下精準送達。
離線優先，失敗自動重試，確保資料在極端情況下的完整性。
低頻寬優化：極致壓縮技術，讓訊息在微弱訊號中穿梭。
開放架構：為未來的遊戲互動與訊息同步打下基礎，讓你在宇宙的任何角落都能自由互動。
開始使用這是一個基於 Rust 的韌性通訊研究專案。

Getting Started / 開始使用
This is a Rust-based research project focused on resilient communication.
這是一個基於 Rust 的韌性通訊研究專案。

# Clone the repository / 克隆倉庫
git clone https://github.com/NeoXCoin/satellite-msg-app.git

# Enter the directory / 進入目錄
cd satellite-msg-app

# Run the simulation / 運行模擬
cargo run

---

### Technical Approach / 技術思路

To achieve "Resilient Communication" under extreme conditions, we focus on:
為了在極端條件下實現「韌性通訊」，我們聚焦於：

*   **Store-and-Forward Logic / 儲存並轉發機制**: Messages are never lost; they are persisted in a local database (SQLite) until      successful delivery is confirmed.
*   訊息永不丟失；在確認成功送達前，訊息會持續存儲在本地資料庫 (SQLite) 中。
*   **Exponential Backoff / 指數退避演算法**: Smart retry strategy to prevent network congestion and battery drain during bad weather.
*   智慧重試策略，防止惡劣天氣下的網路堵塞與電力損耗。
*   **Protocol Buffers / 二進位序列化**: Minimizing data packet size for transmission over low-bandwidth satellite links.
*   極小化數據封包，專為低頻寬衛星連線設計。

---

### Roadmap / 路線圖

*   [ ] **Phase 1: CLI Simulation** - Implementing the core resilient queuing logic. (完成核心韌性隊列邏輯)
*   [ ] **Phase 2: Local Storage Integration** - Integrating SQLite for robust data persistence. (整合 SQLite 實現數據持久化)
*   [ ] **Phase 3: Network Resilience Testing** - Stress testing against simulated high-loss/high-latency scenarios. (對抗高丟包與高延遲的壓力測試)
*   [ ] **Phase 4: UI Development** - Developing an optimistic-UI frontend for end-users. (開發前端介面)
