//! Python 扩展运行时
//! 
//! 支持调用 Python 编写的扩展脚本

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Python 运行时
pub struct PythonRuntime {
    python_path: Option<PathBuf>,
    /// 扩展名 -> 脚本路径
    scripts: HashMap<String, PathBuf>,
}

impl PythonRuntime {
    pub fn new() -> Self {
        Self {
            python_path: find_python(),
            scripts: HashMap::new(),
        }
    }
    
    /// 注册扩展脚本
    pub fn register_script(&mut self, name: &str, path: PathBuf) {
        self.scripts.insert(name.to_string(), path);
    }
    
    /// 检查 Python 是否可用
    pub fn is_available(&self) -> bool {
        self.python_path.is_some()
    }
    
    /// 执行扩展脚本
    /// 
    /// # Arguments
    /// * `name` - 扩展名称
    /// * `params` - 参数字典（JSON序列化后传给Python）
    /// 
    /// # Returns
    /// Python脚本返回的JSON结果
    pub fn execute(&self, name: &str, params: HashMap<String, serde_json::Value>) -> Result<serde_json::Value> {
        let script_path = self.scripts.get(name)
            .context(format!("扩展脚本未注册: {}", name))?;
        
        let python = self.python_path.as_ref()
            .context("Python 未安装")?;
        
        let params_json = serde_json::to_string(&params)
            .context("参数序列化失败")?;
        
        // 执行 Python 脚本
        let output = Command::new(python)
            .arg(script_path)
            .env("OPENFETCH_PARAMS", &params_json)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .context("执行Python脚本失败")?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Python脚本执行失败: {}", stderr);
        }
        
        // 解析JSON输出
        let stdout = String::from_utf8_lossy(&output.stdout);
        let result: serde_json::Value = serde_json::from_str(stdout.trim())
            .context(format!("解析输出失败: {}", stdout))?;
        
        Ok(result)
    }
    
    /// 下载文件
    pub fn download_file(&self, name: &str, url: &str, dest: &Path) -> Result<PathBuf> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), serde_json::json!("download"));
        params.insert("url".to_string(), serde_json::json!(url));
        params.insert("dest".to_string(), serde_json::json!(dest.to_string_lossy().to_string()));
        
        let result = self.execute(name, params)?;
        
        // 返回下载文件路径
        result.get("path")
            .and_then(|p| p.as_str())
            .map(PathBuf::from)
            .context("下载结果中缺少文件路径")
    }
    
    /// 获取媒体信息
    pub fn get_metadata(&self, name: &str, url: &str) -> Result<serde_json::Value> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), serde_json::json!("metadata"));
        params.insert("url".to_string(), serde_json::json!(url));
        
        self.execute(name, params)
    }
}

/// 查找 Python 解释器
fn find_python() -> Option<PathBuf> {
    // 优先使用 python3
    for cmd in &["python3", "python", "python3.11", "python3.10", "python3.9"] {
        if let Ok(output) = Command::new("which").arg(cmd).output() {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path.is_empty() {
                    return Some(PathBuf::from(path));
                }
            }
        }
    }
    
    // 检查常见路径
    let common_paths = [
        "/usr/bin/python3",
        "/usr/local/bin/python3",
        "/opt/homebrew/bin/python3",
    ];
    
    for path in &common_paths {
        if Path::new(path).exists() {
            return Some(PathBuf::from(path));
        }
    }
    
    None
}

/// 获取 Python 版本
pub fn get_python_version() -> Option<String> {
    let python = find_python()?;
    
    let output = Command::new(&python)
        .arg("--version")
        .output()
        .ok()?;
    
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

/// 检查必需的 Python 包
pub fn check_dependencies() -> HashMap<String, bool> {
    let mut deps = HashMap::new();
    
    let required = ["requests", "yt_dlp", "aiohttp"];
    
    for pkg in required {
        let output = Command::new("python3")
            .args(&["-c", &format!("import {}", pkg)])
            .output();
        
        deps.insert(
            pkg.to_string(), 
            output.map(|o| o.status.success()).unwrap_or(false)
        );
    }
    
    deps
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_python_detection() {
        let rt = PythonRuntime::new();
        assert!(rt.is_available(), "Python 应该可用");
        
        if let Some(version) = get_python_version() {
            println!("Python 版本: {}", version);
        }
    }
    
    #[test]
    fn test_dependency_check() {
        let deps = check_dependencies();
        println!("依赖检查: {:?}", deps);
    }
}
