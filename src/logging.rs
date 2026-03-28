use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write as IoWrite;
use std::panic;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::SystemTime;

static LOG_FILE: Mutex<Option<PathBuf>> = Mutex::new(None);

pub fn setup_logging() {
    let log_dir = PathBuf::from(".logs");
    fs::create_dir_all(&log_dir).ok();

    let log_path = log_dir.join("dwarf_the_world.log");
    *LOG_FILE.lock().unwrap() = Some(log_path.clone());

    // Panic hook that logs to file + Discord
    let log_path_clone = log_path.clone();
    panic::set_hook(Box::new(move |panic_info| {
        let msg = format!("PANIC: {}", panic_info);
        log_to_file(&log_path_clone, "PANIC", &msg);
        send_to_discord(&msg, "PANIC");
        eprintln!("{}", msg);
    }));

    log_to_file(&log_path, "INFO", "=== Dwarf The World Started ===");
    println!("Logs: .logs/dwarf_the_world.log");

    if env::var("DISCORD_WEBHOOK_URL").map(|s| !s.is_empty()).unwrap_or(false) {
        println!("[OK] Discord webhook configured - errors will be sent to Discord");
    } else {
        println!("[WARN] DISCORD_WEBHOOK_URL not set - set it to receive crash alerts on Discord");
    }
}

pub fn log_error(msg: &str) {
    if let Some(ref path) = *LOG_FILE.lock().unwrap() {
        log_to_file(path, "ERROR", msg);
    }
    send_to_discord(msg, "ERROR");
    eprintln!("[ERROR] {}", msg);
}

pub fn log_warn(msg: &str) {
    if let Some(ref path) = *LOG_FILE.lock().unwrap() {
        log_to_file(path, "WARN", msg);
    }
    eprintln!("[WARN] {}", msg);
}

pub fn log_info(msg: &str) {
    if let Some(ref path) = *LOG_FILE.lock().unwrap() {
        log_to_file(path, "INFO", msg);
    }
    println!("[INFO] {}", msg);
}

fn log_to_file(path: &PathBuf, level: &str, msg: &str) {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let log_entry = format!("[{}][{}] {}\n", timestamp, level, msg);

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = file.write_all(log_entry.as_bytes());
    }
}

fn send_to_discord(message: &str, level: &str) {
    let webhook_url = match env::var("DISCORD_WEBHOOK_URL") {
        Ok(url) if !url.is_empty() => {
            eprintln!("[Discord] Webhook configured, sending {}...", level);
            url
        },
        _ => {
            eprintln!("[Discord] DISCORD_WEBHOOK_URL not set - skipping Discord notification");
            return;
        }
    };

    let color = match level {
        "PANIC" => 0xFF0000,
        "ERROR" => 0xFF4444,
        "WARN" => 0xFFAA00,
        _ => 0x888888,
    };

    let hostname = env::var("COMPUTERNAME")
        .or_else(|_| env::var("HOSTNAME"))
        .unwrap_or_else(|_| "PC".to_string());

    // Truncate message if too long (Discord embed description limit)
    let truncated_msg = if message.len() > 1000 {
        format!("{}...", &message[..1000])
    } else {
        message.to_string()
    };

    let json_payload = serde_json::json!({
        "embeds": [{
            "title": format!("[{}] Dwarf The World", level),
            "description": format!("```\n{}\n```", truncated_msg),
            "color": color,
            "footer": { "text": hostname },
            "timestamp": chrono::Utc::now().to_rfc3339()
        }]
    });

    // Use blocking reqwest
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build();

    if let Ok(client) = client {
        let _ = client.post(&webhook_url)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&json_payload).unwrap_or_default())
            .send();
    }
}
