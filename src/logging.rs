use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write as IoWrite;
use std::panic;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::SystemTime;

static LOG_FILE: Mutex<Option<PathBuf>> = Mutex::new(None);
static DISCORD_SENT: Mutex<bool> = Mutex::new(false);

pub fn setup_logging() {
    let log_dir = PathBuf::from(".logs");
    fs::create_dir_all(&log_dir).ok();

    let log_path = log_dir.join("dwarf_the_world.log");
    *LOG_FILE.lock().unwrap() = Some(log_path.clone());

    // Clear log file on new start
    let _ = std::fs::remove_file(&log_path);
    log_to_file(&log_path, "INFO", "=== Dwarf The World Started ===");

    let webhook_status = if env::var("DISCORD_WEBHOOK_URL").map(|s| !s.is_empty()).unwrap_or(false) {
        "[OK] Discord webhook SET"
    } else {
        "[WARN] Discord webhook NOT set"
    };

    log_to_file(&log_path, "INFO", webhook_status);
    eprintln!("Logs: .logs/dwarf_the_world.log");
    eprintln!("{}", webhook_status);

    // Send startup ping to Discord so we know the system works
    send_to_discord("🟢 **Dwarf The World** launched", "INFO");
}

pub fn log_error(msg: &str) {
    eprintln!("[ERROR] {}", msg);
    if let Some(ref path) = *LOG_FILE.lock().unwrap() {
        log_to_file(path, "ERROR", msg);
    }
    send_to_discord(msg, "ERROR");
    trigger_github_dispatch(msg);
}

pub fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
    if let Some(ref path) = *LOG_FILE.lock().unwrap() {
        log_to_file(path, "INFO", msg);
    }
}

fn log_to_file(path: &PathBuf, level: &str, msg: &str) {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let log_entry = format!("[{}][{}] {}\n", timestamp, level, msg);

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = file.write_all(log_entry.as_bytes());
        let _ = file.flush();
    }
}

fn trigger_github_dispatch(crash_message: &str) {
    // Write crash log and push via git - no GitHub API needed
    write_crash_log_fallback(crash_message);
}

fn write_crash_log_fallback(crash_message: &str) {
    // Write crash log that GitHub Actions cron can pick up
    // The game pushes this to GitHub which triggers the workflow
    let crash_dir = PathBuf::from("crash-logs");
    std::fs::create_dir_all(&crash_dir).ok();

    let hostname = env::var("COMPUTERNAME")
        .or_else(|_| env::var("HOSTNAME"))
        .unwrap_or_else(|_| "UnknownPC".to_string());

    let timestamp = chrono::Utc::now().format("%Y%m%d-%H%M%S").to_string();
    let filename = format!("crash-{}.log", timestamp);
    let path = crash_dir.join(&filename);

    let content = format!(
        "Machine: {}\nTimestamp: {}\n\n{}\n",
        hostname,
        chrono::Utc::now().to_rfc3339(),
        crash_message
    );

    if let Err(e) = std::fs::write(&path, &content) {
        eprintln!("[GitHub] Failed to write crash log: {}", e);
        return;
    }

    eprintln!("[GitHub] Crash log written to {}/{}", crash_dir.display(), filename);

    // Try to git add + commit + push the crash log
    let result = std::process::Command::new("git")
        .args(["add", &path.to_string_lossy()])
        .output();

    if let Ok(output) = result {
        if output.status.success() {
            eprintln!("[GitHub] Crash log staged for commit");
            // Force add (ignore .gitignore) and commit
            let add_result = std::process::Command::new("git")
                .args(["add", "-f", &path.to_string_lossy()])
                .output();
            if !add_result.as_ref().map(|o| o.status.success()).unwrap_or(false) {
                eprintln!("[GitHub] git add -f failed");
                return;
            }
            eprintln!("[GitHub] Crash log force-added");

            let commit_result = std::process::Command::new("git")
                .args(["commit", "-m", &format!("ci: auto-archive crash log {}", timestamp)])
                .output();
            if let Ok(out) = commit_result {
                if out.status.success() {
                    eprintln!("[GitHub] Crash log committed");
                    let push_result = std::process::Command::new("git")
                        .args(["push", "origin", "HEAD:main", "--force"])
                        .output();
                    if let Ok(push_out) = push_result {
                        if push_out.status.success() {
                            eprintln!("[GitHub] Crash log pushed to GitHub!");
                        } else {
                            eprintln!("[GitHub] Push failed: {}", String::from_utf8_lossy(&push_out.stderr));
                        }
                    }
                } else {
                    eprintln!("[GitHub] Commit failed: {}", String::from_utf8_lossy(&out.stderr));
                }
            }
        } else {
            eprintln!("[GitHub] git add failed: {}", String::from_utf8_lossy(&output.stderr));
        }
    } else {
        eprintln!("[GitHub] Could not run git - is git installed?");
    }
}

fn send_to_discord(message: &str, level: &str) {
    // Prevent duplicate sends for non-errors
    if level != "ERROR" && level != "PANIC" && *DISCORD_SENT.lock().unwrap() {
        return;
    }

    // Check env var first, fall back to hardcoded webhook
    let webhook_url = env::var("DISCORD_WEBHOOK_URL")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| {
            "https://discord.com/api/webhooks/1487528275017465916/vpUjilBbp7AFwlomuJ0jH-QvBTyii6sdcNwciGRE6BPA-1Pja-pF9DBW-dvwHrlHloJW".to_string()
        });

    eprintln!("[Discord] Sending {} notification to Discord...", level);

    let color = match level {
        "PANIC" | "ERROR" => 0xFF0000u32,
        _ => 0xFFAA00u32,
    };

    let hostname = env::var("COMPUTERNAME")
        .or_else(|_| env::var("HOSTNAME"))
        .unwrap_or_else(|_| "UnknownPC".to_string());

    // Truncate long messages
    let truncated_msg = if message.len() > 900 {
        format!("{}...", &message[..900])
    } else {
        message.to_string()
    };

    let json_payload = serde_json::json!({
        "content": format!("<@520600771012591616> **[{}]**", level),
        "embeds": [{
            "title": format!("Dwarf The World - {}", level),
            "description": format!("```\n{}\n```", truncated_msg),
            "color": color,
            "footer": { "text": hostname },
            "timestamp": chrono::Utc::now().to_rfc3339()
        }]
    });

    // Use blocking reqwest - synchronous, will wait for response
    let client = match reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[Discord] Failed to build HTTP client: {}", e);
            return;
        }
    };

    match client.post(&webhook_url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&json_payload).unwrap_or_default())
        .send()
    {
        Ok(resp) => {
            if resp.status().is_success() {
                eprintln!("[Discord] {} notification sent successfully!", level);
                if level == "ERROR" || level == "PANIC" {
                    *DISCORD_SENT.lock().unwrap() = true;
                }
            } else {
                eprintln!("[Discord] Failed to send - status: {}", resp.status());
            }
        }
        Err(e) => {
            eprintln!("[Discord] Request failed: {}", e);
        }
    }
}
