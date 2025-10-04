use serde_json::Value;
use std::path::PathBuf;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ConfigFile {
    pub path: String,
    pub content: Value,
    pub exists: bool,
}

#[tauri::command]
pub async fn read_config_file(config_type: String) -> Result<ConfigFile, String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;

    let path = match config_type.as_str() {
        "user" => home_dir.join(".claude/settings.json"),
        "project" => std::env::current_dir()
            .unwrap()
            .join(".claude/settings.json"),
        "project_local" => std::env::current_dir()
            .unwrap()
            .join(".claude/settings.local.json"),
        "enterprise_macos" => PathBuf::from("/Library/Application Support/ClaudeCode/managed-settings.json"),
        "enterprise_linux" => PathBuf::from("/etc/claude-code/managed-settings.json"),
        "enterprise_windows" => PathBuf::from("C:\\ProgramData\\ClaudeCode\\managed-settings.json"),
        "mcp_macos" => PathBuf::from("/Library/Application Support/ClaudeCode/managed-mcp.json"),
        "mcp_linux" => PathBuf::from("/etc/claude-code/managed-mcp.json"),
        "mcp_windows" => PathBuf::from("C:\\ProgramData\\ClaudeCode\\managed-mcp.json"),
        _ => return Err("Invalid configuration type".to_string()),
    };

    let path_str = path.to_string_lossy().to_string();

    if path.exists() {
        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        let json_content: Value = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        Ok(ConfigFile {
            path: path_str,
            content: json_content,
            exists: true,
        })
    } else {
        Ok(ConfigFile {
            path: path_str,
            content: Value::Object(serde_json::Map::new()),
            exists: false,
        })
    }
}

#[tauri::command]
pub async fn write_config_file(config_type: String, content: Value) -> Result<(), String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;

    let path = match config_type.as_str() {
        "user" => home_dir.join(".claude/settings.json"),
        "project" => {
            let project_path = std::env::current_dir().unwrap().join(".claude");
            std::fs::create_dir_all(&project_path)
                .map_err(|e| format!("Failed to create .claude directory: {}", e))?;
            project_path.join("settings.json")
        },
        "project_local" => {
            let project_path = std::env::current_dir().unwrap().join(".claude");
            std::fs::create_dir_all(&project_path)
                .map_err(|e| format!("Failed to create .claude directory: {}", e))?;
            project_path.join("settings.local.json")
        },
        _ => return Err("Cannot write to enterprise configuration files".to_string()),
    };

    let json_content = serde_json::to_string_pretty(&content)
        .map_err(|e| format!("Failed to serialize JSON: {}", e))?;

    std::fs::write(&path, json_content)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn list_config_files() -> Result<Vec<String>, String> {
    let mut configs = vec![];

    // User settings
    if let Some(home) = dirs::home_dir() {
        let user_settings = home.join(".claude/settings.json");
        if user_settings.exists() {
            configs.push("user".to_string());
        }
    }

    // Project settings
    let current_dir = std::env::current_dir().unwrap();
    let project_settings = current_dir.join(".claude/settings.json");
    if project_settings.exists() {
        configs.push("project".to_string());
    }

    let project_local_settings = current_dir.join(".claude/settings.local.json");
    if project_local_settings.exists() {
        configs.push("project_local".to_string());
    }

    // Enterprise settings (read-only)
    if cfg!(target_os = "macos") {
        let enterprise_path = PathBuf::from("/Library/Application Support/ClaudeCode/managed-settings.json");
        if enterprise_path.exists() {
            configs.push("enterprise_macos".to_string());
        }

        let mcp_path = PathBuf::from("/Library/Application Support/ClaudeCode/managed-mcp.json");
        if mcp_path.exists() {
            configs.push("mcp_macos".to_string());
        }
    } else if cfg!(target_os = "linux") {
        let enterprise_path = PathBuf::from("/etc/claude-code/managed-settings.json");
        if enterprise_path.exists() {
            configs.push("enterprise_linux".to_string());
        }

        let mcp_path = PathBuf::from("/etc/claude-code/managed-mcp.json");
        if mcp_path.exists() {
            configs.push("mcp_linux".to_string());
        }
    } else if cfg!(target_os = "windows") {
        let enterprise_path = PathBuf::from("C:\\ProgramData\\ClaudeCode\\managed-settings.json");
        if enterprise_path.exists() {
            configs.push("enterprise_windows".to_string());
        }

        let mcp_path = PathBuf::from("C:\\ProgramData\\ClaudeCode\\managed-mcp.json");
        if mcp_path.exists() {
            configs.push("mcp_windows".to_string());
        }
    }

    Ok(configs)
}