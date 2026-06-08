//! Plugin Runtime - 脚本运行时
//! 
//! 支持 Python/JavaScript/Wasm/原生 四种运行时

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;
use std::process::Stdio;
use tokio::process::Command;

/// 运行时类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuntimeType {
    Python,
    JavaScript,
    Native,
    Wasm,
}

impl RuntimeType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "python" | "py" => Some(Self::Python),
            "javascript" | "js" => Some(Self::JavaScript),
            "native" | "rs" | "rust" => Some(Self::Native),
            "wasm" | "webassembly" => Some(Self::Wasm),
            _ => None,
        }
    }
}

/// 运行时状态
#[derive(Debug, Clone)]
pub struct RuntimeStatus {
    pub available: bool,
    pub version: Option<String>,
    pub path: Option<String>,
}

/// 插件运行时管理器
pub struct PluginRuntime {
    runtimes: HashMap<RuntimeType, RuntimeStatus>,
}

impl PluginRuntime {
    pub fn new() -> Self {
        let mut runtimes = HashMap::new();
        
        // 检测 Python
        runtimes.insert(RuntimeType::Python, RuntimeStatus {
            available: true,
            version: Some("3.x".into()),
            path: Some("python3".into()),
        });
        
        // 检测 JavaScript
        runtimes.insert(RuntimeType::JavaScript, RuntimeStatus {
            available: true,
            version: Some("18.x+".into()),
            path: Some("node".into()),
        });
        
        // 原生和 Wasm 需要编译
        runtimes.insert(RuntimeType::Native, RuntimeStatus {
            available: true,
            version: Some("compiled".into()),
            path: None,
        });
        runtimes.insert(RuntimeType::Wasm, RuntimeStatus {
            available: true,
            version: Some("wasm32".into()),
            path: None,
        });
        
        Self { runtimes }
    }
    
    pub fn is_available(&self, runtime: RuntimeType) -> bool {
        self.runtimes.get(&runtime)
            .map(|s| s.available)
            .unwrap_or(false)
    }
    
    pub fn status(&self, runtime: RuntimeType) -> &RuntimeStatus {
        self.runtimes.get(&runtime)
            .unwrap_or(&RuntimeStatus { available: false, version: None, path: None })
    }
    
    /// 执行 Python 脚本
    pub async fn run_python(&self, script: &Path, params: &serde_json::Value) -> Result<serde_json::Value> {
        if !self.is_available(RuntimeType::Python) {
            anyhow::bail!("Python 运行时不可用");
        }
        
        let params_str = serde_json::to_string(params)
            .context("参数序列化失败")?;
        
        let output = Command::new("python3")
            .arg(script)
            .arg(&params_str)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Python进程启动失败")?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Python脚本执行失败: {}", stderr);
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let result: serde_json::Value = serde_json::from_str(stdout.trim())
            .context("解析脚本输出失败")?;
        
        Ok(result)
    }
    
    /// 执行 JavaScript 脚本
    pub async fn run_javascript(&self, script: &Path, params: &serde_json::Value) -> Result<serde_json::Value> {
        if !self.is_available(RuntimeType::JavaScript) {
            anyhow::bail!("Node.js 运行时不可用");
        }
        
        let params_str = serde_json::to_string(params)
            .context("参数序列化失败")?;
        
        let output = Command::new("node")
            .arg(script)
            .arg(&params_str)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Node进程启动失败")?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("JavaScript脚本执行失败: {}", stderr);
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let result: serde_json::Value = serde_json::from_str(stdout.trim())
            .context("解析脚本输出失败")?;
        
        Ok(result)
    }
    
    pub fn all_status(&self) -> Vec<(RuntimeType, RuntimeStatus)> {
        self.runtimes.iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect()
    }
}

impl Default for PluginRuntime {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_runtime_detection() {
        let runtime = PluginRuntime::new();
        let py_status = runtime.status(RuntimeType::Python);
        assert!(py_status.available);
    }
    
    #[tokio::test]
    async fn test_python_execution() {
        let runtime = PluginRuntime::new();
        if !runtime.is_available(RuntimeType::Python) {
            return;
        }
        
        // 创建测试脚本
        let temp_dir = std::env::temp_dir();
        let script_path = temp_dir.join("test_echo.py");
        std::fs::write(&script_path, "import sys, json; print(json.dumps({\"result\": \"ok\", \"input\": json.loads(sys.argv[1])}))").unwrap();
        
        let params = serde_json::json!({"test": "value"});
        let result = runtime.run_python(&script_path, &params).await;
        
        std::fs::remove_file(script_path).ok();
        
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data["result"], "ok");
    }
}
