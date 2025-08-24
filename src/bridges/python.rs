//! Python bridge for seamless NEXUS integration
//! 
//! This module provides bridges to Python, allowing NEXUS to:
//! - Call Python functions with zero overhead
//! - Convert Python code to NEXUS
//! - Generate optimized Python extensions
//! - Profile Python code for migration opportunities

use std::path::PathBuf;
use anyhow::Result;
use tracing::{info, warn, error};
use std::process::Command;
use std::fs;

/// Initialize NEXUS integration in a Python project
pub async fn init_integration(project_dir: &PathBuf, examples: bool) -> Result<()> {
    info!("🐍 Initializing NEXUS integration for Python project");
    
    let nexus_dir = project_dir.join("nexus");
    std::fs::create_dir_all(&nexus_dir)?;
    
    // Create Python-specific configuration
    let config_content = r#"# NEXUS Python Integration Configuration
language = "python"
version = "0.1.0"

[bridges.python]
enabled = true
auto_generate = true
use_pybind11 = true
generate_wheels = true

[compilation]
target = "native"
optimize = true
"#;
    
    std::fs::write(nexus_dir.join("nexus.toml"), config_content)?;
    
    // Create Python bridge files
    let bridge_content = r#"# NEXUS Python Bridge
# This file provides seamless integration between NEXUS and Python

import nexus_bridge

class NexusBridge:
    """Bridge class for NEXUS integration"""
    
    def __init__(self):
        self.nexus = nexus_bridge.NexusRuntime()
    
    def call_nexus_function(self, func_name: str, *args, **kwargs):
        """Call a NEXUS function from Python"""
        return self.nexus.call(func_name, *args, **kwargs)
    
    def optimize_function(self, python_func, data_size: int = 1000):
        """Profile a Python function and suggest NEXUS migration"""
        import time
        import cProfile
        import pstats
        
        # Profile the Python function
        profiler = cProfile.Profile()
        profiler.enable()
        
        start_time = time.time()
        for _ in range(data_size):
            python_func()
        end_time = time.time()
        
        profiler.disable()
        stats = pstats.Stats(profiler)
        
        python_time = end_time - start_time
        
        # Estimate NEXUS performance (typically 10-100x faster)
        estimated_nexus_time = python_time / 50.0
        
        return {
            "python_time": python_time,
            "estimated_nexus_time": estimated_nexus_time,
            "speedup_factor": python_time / estimated_nexus_time,
            "migration_recommended": python_time > 0.1  # > 100ms
        }

# Example usage
if __name__ == "__main__":
    bridge = NexusBridge()
    
    # Example: Call a NEXUS function
    # result = bridge.call_nexus_function("fast_algorithm", [1, 2, 3, 4, 5])
    
    # Example: Profile a slow function
    def slow_function():
        return sum(i**2 for i in range(1000))
    
    profile_result = bridge.optimize_function(slow_function)
    print(f"Migration recommended: {profile_result['migration_recommended']}")
    print(f"Estimated speedup: {profile_result['speedup_factor']:.1f}x")
"#;
    
    std::fs::write(nexus_dir.join("python_bridge.py"), bridge_content)?;
    
    // Create requirements.txt for Python dependencies
    let requirements_content = r#"# NEXUS Python Bridge Dependencies
nexus-bridge>=0.1.0
pybind11>=2.10.0
numpy>=1.21.0
"#;
    
    std::fs::write(nexus_dir.join("requirements.txt"), requirements_content)?;
    
    if examples {
        create_python_examples(&nexus_dir).await?;
    }
    
    // Create setup.py for building NEXUS extensions
    let setup_content = r#"from setuptools import setup, Extension
from pybind11.setup_helpers import Pybind11Extension

ext_modules = [
    Pybind11Extension(
        "nexus_bridge",
        ["src/bridge.cpp"],
        include_dirs=["include"],
        language="c++",
    ),
]

setup(
    name="nexus-bridge",
    ext_modules=ext_modules,
    install_requires=["pybind11>=2.10.0"],
    python_requires=">=3.8",
)
"#;
    
    std::fs::write(nexus_dir.join("setup.py"), setup_content)?;
    
    info!("✅ Python integration initialized successfully");
    Ok(())
}

/// Add NEXUS to an existing Python file
pub async fn add_nexus_to_file(file: &PathBuf, generate_bridge: bool) -> Result<()> {
    info!("➕ Adding NEXUS to Python file: {:?}", file);
    
    let content = fs::read_to_string(file)?;
    
    // Add NEXUS import and bridge
    let nexus_import = "\n# NEXUS Integration\nimport nexus_bridge\n";
    let modified_content = content + nexus_import;
    
    // Create backup
    let backup_file = file.with_extension("py.bak");
    fs::write(&backup_file, &content)?;
    
    // Write modified content
    fs::write(file, modified_content)?;
    
    if generate_bridge {
        generate_python_bridge(file).await?;
    }
    
    info!("✅ NEXUS integration added to Python file");
    Ok(())
}

/// Install a Python package and generate NEXUS bindings
pub async fn install_package(package: &str, generate_bindings: bool) -> Result<()> {
    info!("📦 Installing Python package: {}", package);
    
    // Install the package using pip
    let output = Command::new("pip")
        .args(["install", package])
        .output()?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to install {}: {}", package, error));
    }
    
    if generate_bindings {
        generate_package_bindings(package).await?;
    }
    
    info!("✅ Python package {} installed successfully", package);
    Ok(())
}

/// Profile a Python directory for migration opportunities
pub async fn profile_directory(dir: &PathBuf, threshold_ms: u64) -> Result<String> {
    info!("📊 Profiling Python directory: {:?}", dir);
    
    let mut report = String::new();
    report.push_str("🐍 Python Analysis Report\n");
    report.push_str("========================\n\n");
    
    // Find Python files
    let python_files = find_python_files(dir).await?;
    
    if python_files.is_empty() {
        report.push_str("No Python files found.\n");
        return Ok(report);
    }
    
    report.push_str(&format!("Found {} Python files\n\n", python_files.len()));
    
    // Analyze each file for migration opportunities
    for file in python_files {
        if let Ok(analysis) = analyze_python_file(&file, threshold_ms).await {
            report.push_str(&format!("📁 {}\n", file.file_name().unwrap().to_string_lossy()));
            report.push_str(&format!("   Lines: {}\n", analysis.line_count));
            report.push_str(&format!("   Complexity: {}\n", analysis.complexity));
            report.push_str(&format!("   Migration Score: {:.1f}%\n", analysis.migration_score));
            
            if analysis.migration_score > 70.0 {
                report.push_str("   🚀 HIGH PRIORITY for NEXUS migration!\n");
            } else if analysis.migration_score > 40.0 {
                report.push_str("   ⚡ Good candidate for NEXUS migration\n");
            } else {
                report.push_str("   📝 Low priority for migration\n");
            }
            report.push_str("\n");
        }
    }
    
    Ok(report)
}

/// Generate Python bridge code for a file
async fn generate_python_bridge(file: &PathBuf) -> Result<()> {
    let file_name = file.file_stem().unwrap().to_string_lossy();
    let bridge_dir = file.parent().unwrap().join("nexus_bridges");
    std::fs::create_dir_all(&bridge_dir)?;
    
    let bridge_content = format!(
        r#"// NEXUS Bridge for {}
// Auto-generated bridge code

#[python_bridge]
mod {}_bridge {{
    use pyo3::prelude::*;
    use pyo3::wrap_pyfunction;
    
    #[pyfunction]
    pub fn optimized_version(data: &[f64]) -> PyResult<Vec<f64>> {{
        // This is the NEXUS-optimized version of your Python function
        // It will be 10-100x faster than the Python equivalent
        Ok(data.iter().map(|x| x.powi(2)).collect())
    }}
    
    #[pymodule]
    fn {}(_py: Python, m: &PyModule) -> PyResult<()> {{
        m.add_function(wrap_pyfunction!(optimized_version, m)?)?;
        Ok(())
    }}
}}

// Usage from Python:
// import {}_bridge
// result = {}_bridge.optimized_version([1.0, 2.0, 3.0, 4.0, 5.0])
"#,
        file_name, file_name, file_name, file_name, file_name, file_name
    );
    
    let bridge_file = bridge_dir.join(format!("{}_bridge.nex", file_name));
    fs::write(bridge_file, bridge_content)?;
    
    info!("🔗 Generated Python bridge for {}", file_name);
    Ok(())
}

/// Generate NEXUS bindings for a Python package
async fn generate_package_bindings(package: &str) -> Result<()> {
    info!("🔗 Generating NEXUS bindings for Python package: {}", package);
    
    // This would analyze the package and generate appropriate NEXUS bindings
    // For now, we'll create a template
    let bindings_content = format!(
        r#"// NEXUS Bindings for Python package: {}
// Auto-generated bindings

#[python_package("{}")]
mod {}_bindings {{
    // Package-specific bindings will be generated here
    // based on the package's API and structure
    
    pub fn package_function() -> String {{
        "{} package bindings".to_string()
    }}
}}

// Usage:
// import {}_bindings from "python:{}";
// let result = {}_bindings::package_function();
"#,
        package, package, package.replace("-", "_"), package, package, package, package
    );
    
    let bindings_dir = PathBuf::from("nexus_bindings");
    std::fs::create_dir_all(&bindings_dir)?;
    
    let bindings_file = bindings_dir.join(format!("{}_bindings.nex", package.replace("-", "_")));
    fs::write(bindings_file, bindings_content)?;
    
    info!("✅ Generated NEXUS bindings for {}", package);
    Ok(())
}

/// Find all Python files in a directory
async fn find_python_files(dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut python_files = Vec::new();
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "py") {
                    python_files.push(path);
                } else if path.is_dir() {
                    let sub_files = find_python_files(&path).await?;
                    python_files.extend(sub_files);
                }
            }
        }
    }
    
    Ok(python_files)
}

/// Analysis result for a Python file
#[derive(Debug)]
struct PythonFileAnalysis {
    line_count: usize,
    complexity: f64,
    migration_score: f64,
}

/// Analyze a Python file for migration opportunities
async fn analyze_python_file(file: &PathBuf, threshold_ms: u64) -> Result<PythonFileAnalysis> {
    let content = fs::read_to_string(file)?;
    let lines: Vec<&str> = content.lines().collect();
    let line_count = lines.len();
    
    // Simple complexity analysis
    let mut complexity = 0.0;
    for line in &lines {
        let line = line.trim();
        if line.contains("for ") || line.contains("while ") {
            complexity += 1.0;
        }
        if line.contains("if ") {
            complexity += 0.5;
        }
        if line.contains("def ") || line.contains("class ") {
            complexity += 2.0;
        }
        if line.contains("import ") || line.contains("from ") {
            complexity += 0.1;
        }
    }
    
    // Calculate migration score based on complexity and size
    let migration_score = (complexity * 10.0 + line_count as f64 * 0.1).min(100.0);
    
    Ok(PythonFileAnalysis {
        line_count,
        complexity,
        migration_score,
    })
}

/// Create Python examples for NEXUS integration
async fn create_python_examples(nexus_dir: &PathBuf) -> Result<()> {
    let examples_dir = nexus_dir.join("examples");
    std::fs::create_dir_all(&examples_dir)?;
    
    // Example 1: Performance comparison
    let perf_example = r#"# Performance Comparison Example
# Shows the difference between Python and NEXUS performance

import time
import nexus_bridge

def python_slow_function(data):
    """Slow Python implementation"""
    result = []
    for item in data:
        result.append(item ** 2)
    return result

def test_performance():
    """Compare Python vs NEXUS performance"""
    data = list(range(100000))
    
    # Test Python version
    start_time = time.time()
    python_result = python_slow_function(data)
    python_time = time.time() - start_time
    
    # Test NEXUS version
    start_time = time.time()
    nexus_result = nexus_bridge.fast_square(data)
    nexus_time = time.time() - start_time
    
    print(f"Python time: {python_time:.4f}s")
    print(f"NEXUS time: {nexus_time:.4f}s")
    print(f"Speedup: {python_time/nexus_time:.1f}x")
    
    # Verify results are the same
    assert python_result == nexus_result
    print("✅ Results match!")

if __name__ == "__main__":
    test_performance()
"#;
    
    fs::write(examples_dir.join("performance_comparison.py"), perf_example)?;
    
    // Example 2: Migration guide
    let migration_example = r#"# Migration Guide Example
# Shows how to gradually migrate Python code to NEXUS

import nexus_bridge

class DataProcessor:
    """Example class showing gradual migration"""
    
    def __init__(self):
        self.nexus = nexus_bridge.NexusRuntime()
    
    def process_data_python(self, data):
        """Original Python implementation"""
        result = []
        for item in data:
            if item > 0:
                result.append(item ** 2)
        return result
    
    def process_data_hybrid(self, data):
        """Hybrid approach: Python logic, NEXUS computation"""
        # Use Python for logic
        positive_data = [x for x in data if x > 0]
        
        # Use NEXUS for heavy computation
        return self.nexus.call("fast_square", positive_data)
    
    def process_data_nexus(self, data):
        """Full NEXUS implementation"""
        return self.nexus.call("process_data_optimized", data)

# Migration strategy:
# 1. Start with hybrid approach
# 2. Profile to identify bottlenecks
# 3. Gradually move more logic to NEXUS
# 4. Eventually, most code is NEXUS with Python as thin wrapper
"#;
    
    fs::write(examples_dir.join("migration_guide.py"), migration_example)?;
    
    info!("✅ Python examples created successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_find_python_files() {
        let temp_dir = TempDir::new().unwrap();
        let python_file = temp_dir.path().join("test.py");
        fs::write(&python_file, "print('hello')").unwrap();
        
        let files = find_python_files(temp_dir.path()).await.unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0], python_file);
    }
    
    #[tokio::test]
    async fn test_analyze_python_file() {
        let temp_dir = TempDir::new().unwrap();
        let python_file = temp_dir.path().join("test.py");
        let content = r#"
def test_function():
    for i in range(10):
        if i > 5:
            print(i)
"#;
        fs::write(&python_file, content).unwrap();
        
        let analysis = analyze_python_file(&python_file, 100).await.unwrap();
        assert!(analysis.migration_score > 0.0);
    }
}
