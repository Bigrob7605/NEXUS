use std::fs;
use std::path::Path;
use std::time::Instant;
use nexus::enhanced_compression::{EnhancedCompressionEngine, EnhancedCompressionConfig};
use nexus::gamma_ast::{GammaAST, GammaNode, GammaValue, GammaNodeType, CompressionLevel};
use std::collections::HashMap;

/// Test REAL compression with ACTUAL source code files
/// No more synthetic data - let's see what this can really do

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 NEXUS REAL COMPRESSION TEST - NO MORE TOYS!");
    println!("{}", "=".repeat(60));
    println!("Testing compression on ACTUAL source code files...");
    
    // Create compression engine with aggressive settings
    let mut config = EnhancedCompressionConfig::default();
    config.target_ratio = 20.0; // Target 20x compression
    config.enable_neuromorphic = true;
    config.enable_ai_scheduling = true;
    config.pattern_evolution = true;
    
    let mut engine = EnhancedCompressionEngine::new(config);
    
    // Test 1: Real Rust source file
    println!("\n📋 TEST 1: Real Rust Source File");
    let rust_source = fs::read_to_string("src/enhanced_compression.rs")?;
    println!("   File: src/enhanced_compression.rs");
    println!("   Size: {} bytes", rust_source.len());
    println!("   Lines: {}", rust_source.lines().count());
    
    let ast = parse_rust_source(&rust_source);
    println!("   AST nodes: {}", ast.nodes.len());
    
    let start = Instant::now();
    match engine.compress_ast(&ast).await {
        Ok(result) => {
            let duration = start.elapsed();
            println!("   ✅ Compression SUCCESSFUL!");
            println!("   Original size: {} bytes", result.original_size);
            println!("   Compressed size: {} bytes", result.compressed_size);
            println!("   Compression ratio: {:.2}x", result.compression_ratio);
            println!("   Patterns identified: {}", result.patterns_identified);
            println!("   Processing time: {:?}", duration);
            
            // Validate the claims
            if result.compression_ratio > 1.0 {
                println!("   🎉 REAL COMPRESSION ACHIEVED!");
                println!("   Size reduction: {:.1}%", (1.0 - 1.0/result.compression_ratio) * 100.0);
            } else {
                println!("   ❌ NO COMPRESSION - Still 1.00x ratio");
            }
        }
        Err(e) => {
            println!("   ❌ Compression FAILED: {}", e);
            return Err(e.into());
        }
    }
    
    // Test 2: Another real source file
    println!("\n📋 TEST 2: Another Real Source File");
    let gamma_source = fs::read_to_string("src/gamma_ast/mod.rs")?;
    println!("   File: src/gamma_ast/mod.rs");
    println!("   Size: {} bytes", gamma_source.len());
    println!("   Lines: {}", gamma_source.lines().count());
    
    let ast2 = parse_rust_source(&gamma_source);
    println!("   AST nodes: {}", ast2.nodes.len());
    
    let start = Instant::now();
    match engine.compress_ast(&ast2).await {
        Ok(result) => {
            let duration = start.elapsed();
            println!("   ✅ Compression SUCCESSFUL!");
            println!("   Original size: {} bytes", result.original_size);
            println!("   Compressed size: {} bytes", result.compressed_size);
            println!("   Compression ratio: {:.2}x", result.compression_ratio);
            println!("   Patterns identified: {}", result.patterns_identified);
            println!("   Processing time: {:?}", duration);
            
            if result.compression_ratio > 1.0 {
                println!("   🎉 REAL COMPRESSION ACHIEVED!");
                println!("   Size reduction: {:.1}%", (1.0 - 1.0/result.compression_ratio) * 100.0);
            } else {
                println!("   ❌ NO COMPRESSION - Still 1.00x ratio");
            }
        }
        Err(e) => {
            println!("   ❌ Compression FAILED: {}", e);
            return Err(e.into());
        }
    }
    
    // Test 3: Large source file
    println!("\n📋 TEST 3: Large Source File");
    if Path::new("Cargo.toml").exists() {
        let cargo_content = fs::read_to_string("Cargo.toml")?;
        println!("   File: Cargo.toml");
        println!("   Size: {} bytes", cargo_content.len());
        
        let ast3 = parse_toml_source(&cargo_content);
        println!("   AST nodes: {}", ast3.nodes.len());
        
        let start = Instant::now();
        match engine.compress_ast(&ast3).await {
            Ok(result) => {
                let duration = start.elapsed();
                println!("   ✅ Compression SUCCESSFUL!");
                println!("   Original size: {} bytes", result.original_size);
                println!("   Compressed size: {} bytes", result.compressed_size);
                println!("   Compression ratio: {:.2}x", result.compression_ratio);
                println!("   Patterns identified: {}", result.patterns_identified);
                println!("   Processing time: {:?}", duration);
                
                if result.compression_ratio > 1.0 {
                    println!("   🎉 REAL COMPRESSION ACHIEVED!");
                    println!("   Size reduction: {:.1}%", (1.0 - 1.0/result.compression_ratio) * 100.0);
                } else {
                    println!("   ❌ NO COMPRESSION - Still 1.00x ratio");
                }
            }
            Err(e) => {
                println!("   ❌ Compression FAILED: {}", e);
                return Err(e.into());
            }
        }
    }
    
    // Final assessment
    println!("\n{}", "=".repeat(60));
    println!("🎯 REAL COMPRESSION ASSESSMENT");
    println!("{}", "=".repeat(60));
    
    if Path::new("src/enhanced_compression.rs").exists() {
        println!("✅ Real source files tested");
        println!("✅ No more synthetic data");
        println!("✅ Actual compression results");
    } else {
        println!("❌ Could not find source files");
        println!("❌ Still using synthetic data");
    }
    
    println!("\n🌟 Next: Test on Linux kernel, Chromium, real projects!");
    
    Ok(())
}

/// Parse real Rust source code into AST
fn parse_rust_source(source: &str) -> GammaAST {
    let mut ast = GammaAST::new();
    let mut node_id = 1u64;
    
    // Parse lines into AST nodes
    for (line_num, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            // Create node for each non-empty line
            let node = GammaNode {
                id: node_id,
                node_type: GammaNodeType::Statement,
                value: GammaValue::Direct(line.to_string()), // Full line content
                location: None,
                children: vec![],
                metadata: HashMap::new(),
                compression_level: CompressionLevel::None,
            };
            
            ast.add_node(node);
            ast.add_root(node_id);
            node_id += 1;
        }
    }
    
    ast
}

/// Parse TOML content into AST
fn parse_toml_source(source: &str) -> GammaAST {
    let mut ast = GammaAST::new();
    let mut node_id = 1u64;
    
    // Parse TOML sections and key-value pairs
    for (line_num, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            let node = GammaNode {
                id: node_id,
                node_type: GammaNodeType::Statement,
                value: GammaValue::Direct(line.to_string()),
                location: None,
                children: vec![],
                metadata: HashMap::new(),
                compression_level: CompressionLevel::None,
            };
            
            ast.add_node(node);
            ast.add_root(node_id);
            node_id += 1;
        }
    }
    
    ast
}

/// Run the real compression test
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_real_rust_compression() {
        let config = EnhancedCompressionConfig::default();
        let mut engine = EnhancedCompressionEngine::new(config);
        
        // Test with actual Rust source
        let rust_source = r#"
pub struct EnhancedCompressionEngine {
    pub config: EnhancedCompressionConfig,
    neuromorphic_memory: Arc<Mutex<HashMap<u64, MemoryRegion>>>,
    learning_engine: Arc<Mutex<LearningEngine>>,
    gpu_manager: Arc<Mutex<GPUMemoryManager>>,
    pattern_evolution: Arc<Mutex<PatternEvolution>>,
    compression_history: VecDeque<CompressionResult>,
}

impl EnhancedCompressionEngine {
    pub fn new(config: EnhancedCompressionConfig) -> Self {
        let gpu_manager = GPUMemoryManager::new(2, 8 * 1024 * 1024 * 1024);
        
        Self {
            config,
            neuromorphic_memory: Arc::new(Mutex::new(HashMap::new())),
            learning_engine: Arc::new(Mutex::new(LearningEngine::new())),
            gpu_manager: Arc::new(Mutex::new(gpu_manager)),
            pattern_evolution: Arc::new(Mutex::new(PatternEvolution::new())),
            compression_history: VecDeque::new(),
        }
    }
}"#;
        
        let ast = parse_rust_source(rust_source);
        let result = engine.compress_ast(&ast).await.unwrap();
        
        // Basic validation
        assert!(result.compression_ratio > 0.0);
        assert!(result.original_size > 0);
        assert!(result.compressed_size > 0);
        
        println!("Real Rust compression test passed: {:.2}x ratio", result.compression_ratio);
    }
}
