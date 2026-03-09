use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// QuickClaw 配置文件 - 存储 Claw 身份性格定义及服务器设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClawConfig {
    /// OpenClaw 服务器地址
    pub server_url: String,
    /// API 密钥（可选）
    pub api_key: String,
    /// Claw 名称（身份定义）
    pub claw_name: String,
    /// Claw 角色描述
    pub claw_role: String,
    /// Claw 性格特征列表
    pub personality_traits: Vec<String>,
    /// 系统提示词（详细性格定义）
    pub system_prompt: String,
    /// 是否已完成初始配置向导
    pub configured: bool,
}

impl Default for ClawConfig {
    fn default() -> Self {
        Self {
            server_url: String::from("http://localhost:8080"),
            api_key: String::new(),
            claw_name: String::from("Claw"),
            claw_role: String::from("智能助手"),
            personality_traits: vec![
                String::from("友善"),
                String::from("专业"),
                String::from("耐心"),
            ],
            system_prompt: String::from(
                "你是一个友善、专业、耐心的智能助手。你会用简洁清晰的语言回答用户的问题，\
                提供准确有用的信息，并在需要时给出详细解释。",
            ),
            configured: false,
        }
    }
}

impl ClawConfig {
    /// 获取配置文件路径
    pub fn config_path() -> Option<PathBuf> {
        directories::ProjectDirs::from("com", "QuickClaw", "QuickClaw").map(|dirs| {
            let config_dir = dirs.config_dir().to_path_buf();
            config_dir.join("config.json")
        })
    }

    /// 从文件加载配置
    pub fn load() -> Self {
        let Some(path) = Self::config_path() else {
            return Self::default();
        };

        match std::fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// 保存配置到文件
    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path().ok_or("无法获取配置文件路径")?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("创建配置目录失败: {}", e))?;
        }

        let content =
            serde_json::to_string_pretty(self).map_err(|e| format!("序列化配置失败: {}", e))?;

        std::fs::write(&path, content).map_err(|e| format!("写入配置文件失败: {}", e))?;

        Ok(())
    }

    /// 获取完整的性格特征字符串
    pub fn traits_as_string(&self) -> String {
        self.personality_traits.join("、")
    }

    /// 从逗号/顿号分隔的字符串解析性格特征
    pub fn parse_traits(input: &str) -> Vec<String> {
        input
            .split(&[',', '，', '、', ';', '；'][..])
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ClawConfig::default();
        assert_eq!(config.claw_name, "Claw");
        assert_eq!(config.claw_role, "智能助手");
        assert!(!config.configured);
        assert!(!config.personality_traits.is_empty());
    }

    #[test]
    fn test_traits_as_string() {
        let config = ClawConfig {
            personality_traits: vec![
                "友善".to_string(),
                "专业".to_string(),
                "耐心".to_string(),
            ],
            ..Default::default()
        };
        assert_eq!(config.traits_as_string(), "友善、专业、耐心");
    }

    #[test]
    fn test_parse_traits() {
        let traits = ClawConfig::parse_traits("友善, 专业、耐心;幽默");
        assert_eq!(traits, vec!["友善", "专业", "耐心", "幽默"]);
    }

    #[test]
    fn test_parse_traits_chinese_comma() {
        let traits = ClawConfig::parse_traits("友善，专业，耐心");
        assert_eq!(traits, vec!["友善", "专业", "耐心"]);
    }

    #[test]
    fn test_config_serialization() {
        let config = ClawConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let restored: ClawConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.claw_name, restored.claw_name);
        assert_eq!(config.server_url, restored.server_url);
        assert_eq!(config.configured, restored.configured);
    }
}
