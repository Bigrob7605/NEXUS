use std::fs;
use std::time::Instant;
use nexus::enhanced_compression::{EnhancedCompressionEngine, EnhancedCompressionConfig};
use nexus::gamma_ast::{GammaAST, GammaNode, GammaValue, GammaNodeType, CompressionLevel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 SIMPLE COMPRESSION TEST - ISOLATE THE ISSUE");
    println!("{}", "=".repeat(50));
    
    // Test with a simple string first
    let test_content = r#"
fn main() {
    println!("Hello, world!");
    let x = 42;
    let y = x * 2;
    println!("Result: {}", y);
}
"#;
    
    println!("📝 Test content ({} bytes):", test_content.len());
    println!("{}", test_content);
    
    // Create AST manually
    let ast = create_simple_ast(test_content);
    println!("🌳 AST created with {} nodes", ast.nodes.len());
    
    // Test compression
    let mut config = EnhancedCompressionConfig::default();
    config.target_ratio = 2.0;
    config.enable_neuromorphic = true;
    config.enable_ai_scheduling = true;
    
    let mut engine = EnhancedCompressionEngine::new(config);
    
    println!("🔧 Testing compression...");
    let start_time = Instant::now();
    
    match engine.compress_ast(&ast).await {
        Ok(result) => {
            println!("✅ Compression completed!");
            println!("📊 Results:");
            println!("   Original size: {} bytes", result.original_size);
            println!("   Compressed size: {} bytes", result.compressed_size);
            println!("   Compression ratio: {:.2}x", result.compression_ratio);
            println!("   Patterns identified: {}", result.patterns_identified);
            println!("   Processing time: {:?}", result.processing_time);
            
            // Calculate actual ratio
            let actual_ratio = if result.compressed_size > 0 {
                result.original_size as f64 / result.compressed_size as f64
            } else {
                1.0
            };
            
            println!("🔍 Actual ratio: {:.2}x", actual_ratio);
            
            if actual_ratio < 1.0 {
                println!("🚨 PROBLEM: Files are getting LARGER after compression!");
                println!("   Size increase: {:.1}%", (1.0 - actual_ratio) * 100.0);
            } else if actual_ratio > 1.0 {
                println!("✅ SUCCESS: Real compression achieved!");
                println!("   Size reduction: {:.1}%", (1.0 - 1.0/actual_ratio) * 100.0);
            } else {
                println!("⚠️  NEUTRAL: No compression achieved");
            }
        }
        Err(e) => {
            eprintln!("❌ Compression failed: {}", e);
        }
    }
    
    let total_time = start_time.elapsed();
    println!("⏱️  Total test time: {:?}", total_time);
    
    Ok(())
}

fn create_simple_ast(source: &str) -> GammaAST {
    let mut ast = GammaAST::new();
    let mut node_id = 1u64;
    
    // Create a simple AST structure
    for (line_num, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            let node = GammaNode {
                id: node_id,
                node_type: GammaNodeType::Statement,
                value: GammaValue::Direct(line.to_string()),
                location: None,
                children: vec![],
                metadata: std::collections::HashMap::new(),
                compression_level: CompressionLevel::None,
            };
            
            ast.add_node(node);
            ast.add_root(node_id);
            node_id += 1;
        }
    }
    
    ast
}
