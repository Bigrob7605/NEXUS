//! Language bridges for seamless interoperability
//! 
//! This module provides bridges to other programming languages, allowing
//! NEXUS to integrate with existing codebases and gradually migrate
//! developers to the future of programming.

use std::path::PathBuf;
use anyhow::Result;
use tracing::{info, warn, error};

pub mod python;
pub mod rust;
pub mod javascript;
pub mod cpp;
pub mod go;

/// Supported language bridges
#[derive(Debug, Clone, PartialEq)]
pub enum SupportedLanguage {
    Python,
    Rust,
    JavaScript,
    TypeScript,
    Cpp,
    Go,
    Java,
    CSharp,
}

impl std::str::FromStr for SupportedLanguage {
    type Err = anyhow::Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "python" | "py" => Ok(SupportedLanguage::Python),
            "rust" | "rs" => Ok(SupportedLanguage::Rust),
            "javascript" | "js" => Ok(SupportedLanguage::JavaScript),
            "typescript" | "ts" => Ok(SupportedLanguage::TypeScript),
            "cpp" | "c++" | "c" => Ok(SupportedLanguage::Cpp),
            "go" | "golang" => Ok(SupportedLanguage::Go),
            "java" => Ok(SupportedLanguage::Java),
            "csharp" | "c#" => Ok(SupportedLanguage::CSharp),
            _ => Err(anyhow::anyhow!("Unsupported language: {}", s)),
        }
    }
}

/// Initialize NEXUS integration in an existing project
pub async fn init_integration(
    project_dir: &PathBuf,
    language: &str,
    examples: bool,
) -> Result<()> {
    let lang: SupportedLanguage = language.parse()?;
    info!("🔗 Initializing NEXUS integration for {} project", language);
    
    match lang {
        SupportedLanguage::Python => {
            python::init_integration(project_dir, examples).await?;
        }
        SupportedLanguage::Rust => {
            rust::init_integration(project_dir, examples).await?;
        }
        SupportedLanguage::JavaScript | SupportedLanguage::TypeScript => {
            javascript::init_integration(project_dir, examples).await?;
        }
        SupportedLanguage::Cpp => {
            cpp::init_integration(project_dir, examples).await?;
        }
        SupportedLanguage::Go => {
            go::init_integration(project_dir, examples).await?;
        }
        _ => {
            warn!("⚠️  Language {} not yet fully supported, using generic integration", language);
            init_generic_integration(project_dir, language, examples).await?;
        }
    }
    
    info!("✅ NEXUS integration initialized successfully");
    Ok(())
}

/// Add NEXUS to an existing file
pub async fn add_nexus_to_file(
    language: &str,
    file: &PathBuf,
    generate_bridge: bool,
) -> Result<()> {
    let lang: SupportedLanguage = language.parse()?;
    info!("➕ Adding NEXUS to {} file: {:?}", language, file);
    
    match lang {
        SupportedLanguage::Python => {
            python::add_nexus_to_file(file, generate_bridge).await?;
        }
        SupportedLanguage::Rust => {
            rust::add_nexus_to_file(file, generate_bridge).await?;
        }
        SupportedLanguage::JavaScript | SupportedLanguage::TypeScript => {
            javascript::add_nexus_to_file(file, generate_bridge).await?;
        }
        SupportedLanguage::Cpp => {
            cpp::add_nexus_to_file(file, generate_bridge).await?;
        }
        SupportedLanguage::Go => {
            go::add_nexus_to_file(file, generate_bridge).await?;
        }
        _ => {
            return Err(anyhow::anyhow!("Language {} not yet supported", language));
        }
    }
    
    info!("✅ NEXUS integration added to file");
    Ok(())
}

/// Install a package from another language
pub async fn install_package(package: &str, generate_bindings: bool) -> Result<()> {
    let (lang, pkg_name) = parse_package_spec(package)?;
    info!("📦 Installing {} package: {}", lang, pkg_name);
    
    match lang {
        SupportedLanguage::Python => {
            python::install_package(&pkg_name, generate_bindings).await?;
        }
        SupportedLanguage::Rust => {
            rust::install_package(&pkg_name, generate_bindings).await?;
        }
        SupportedLanguage::JavaScript | SupportedLanguage::TypeScript => {
            javascript::install_package(&pkg_name, generate_bindings).await?;
        }
        SupportedLanguage::Cpp => {
            cpp::install_package(&pkg_name, generate_bindings).await?;
        }
        SupportedLanguage::Go => {
            go::install_package(&pkg_name, generate_bindings).await?;
        }
        _ => {
            return Err(anyhow::anyhow!("Language {} not yet supported", language));
        }
    }
    
    if generate_bindings {
        info!("🔗 Generated NEXUS bindings for {}", pkg_name);
    }
    
    info!("✅ Package {} installed successfully", pkg_name);
    Ok(())
}

/// Profile existing codebase for NEXUS migration opportunities
pub async fn profile_codebase(
    dir: &PathBuf,
    generate_suggestions: bool,
    threshold_ms: u64,
) -> Result<String> {
    info!("📊 Profiling codebase for migration opportunities");
    
    let mut report = String::new();
    report.push_str("🔍 NEXUS Migration Analysis Report\n");
    report.push_str("================================\n\n");
    
    // Analyze Python files
    if let Ok(python_report) = python::profile_directory(dir, threshold_ms).await {
        report.push_str(&python_report);
    }
    
    // Analyze Rust files
    if let Ok(rust_report) = rust::profile_directory(dir, threshold_ms).await {
        report.push_str(&rust_report);
    }
    
    // Analyze JavaScript/TypeScript files
    if let Ok(js_report) = javascript::profile_directory(dir, threshold_ms).await {
        report.push_str(&js_report);
    }
    
    // Analyze C++ files
    if let Ok(cpp_report) = cpp::profile_directory(dir, threshold_ms).await {
        report.push_str(&cpp_report);
    }
    
    // Analyze Go files
    if let Ok(go_report) = go::profile_directory(dir, threshold_ms).await {
        report.push_str(&go_report);
    }
    
    if generate_suggestions {
        report.push_str("\n🚀 Migration Suggestions:\n");
        report.push_str("========================\n");
        report.push_str("1. Start with performance-critical functions\n");
        report.push_str("2. Convert one module at a time\n");
        report.push_str("3. Use NEXUS for new features\n");
        report.push_str("4. Generate bridges for complex integrations\n");
        report.push_str("5. Profile regularly to identify opportunities\n");
    }
    
    Ok(report)
}

/// Parse package specification (e.g., "python:requests", "rust:serde")
fn parse_package_spec(package: &str) -> Result<(SupportedLanguage, String)> {
    let parts: Vec<&str> = package.split(':').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("Invalid package specification: {}", package));
    }
    
    let language: SupportedLanguage = parts[0].parse()?;
    let package_name = parts[1].to_string();
    
    Ok((language, package_name))
}

/// Initialize generic integration for unsupported languages
async fn init_generic_integration(
    project_dir: &PathBuf,
    language: &str,
    examples: bool,
) -> Result<()> {
    info!("🔧 Setting up generic NEXUS integration for {}", language);
    
    // Create basic NEXUS configuration
    let nexus_dir = project_dir.join("nexus");
    std::fs::create_dir_all(&nexus_dir)?;
    
    // Create basic configuration file
    let config_content = format!(
        r#"# NEXUS Integration Configuration
language = "{}"
version = "0.1.0"

[bridges]
enabled = true
auto_generate = true

[compilation]
target = "native"
optimize = true
"#,
        language
    );
    
    std::fs::write(nexus_dir.join("nexus.toml"), config_content)?;
    
    if examples {
        // Create example bridge file
        let example_content = format!(
            r#"// Example NEXUS bridge for {}
// This file shows how to integrate NEXUS with your {} codebase

#[bridge("{}")]
mod {}_bridge {{
    // Add your bridge functions here
    pub fn example_function() -> String {{
        "Hello from NEXUS bridge!".to_string()
    }}
}}
"#,
            language, language, language, language
        );
        
        std::fs::write(nexus_dir.join("examples").join("bridge.nex"), example_content)?;
    }
    
    info!("✅ Generic integration setup completed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_package_spec() {
        let (lang, pkg) = parse_package_spec("python:requests").unwrap();
        assert_eq!(lang, SupportedLanguage::Python);
        assert_eq!(pkg, "requests");
        
        let (lang, pkg) = parse_package_spec("rust:serde").unwrap();
        assert_eq!(lang, SupportedLanguage::Rust);
        assert_eq!(pkg, "serde");
    }
    
    #[test]
    fn test_parse_language() {
        let lang: SupportedLanguage = "python".parse().unwrap();
        assert_eq!(lang, SupportedLanguage::Python);
        
        let lang: SupportedLanguage = "PYTHON".parse().unwrap();
        assert_eq!(lang, SupportedLanguage::Python);
    }
}
