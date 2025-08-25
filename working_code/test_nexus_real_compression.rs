use std::fs;
use std::path::Path;
use std::time::Instant;
use nexus::enhanced_compression::{EnhancedCompressionEngine, EnhancedCompressionConfig};
use nexus::gamma_ast::{GammaAST, GammaNode, GammaValue, GammaNodeType, CompressionLevel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 NEXUS REAL COMPRESSION TEST - ACTUAL ENGINE!");
    println!("{}", "=".repeat(60));
    
    let test_dir = "test_codebases/rust-compiler";
    if !Path::new(test_dir).exists() {
        eprintln!("❌ Test directory not found: {}", test_dir);
        return Ok(());
    }
    
    println!("📁 Testing NEXUS engine on: {}", test_dir);
    println!("🔍 Scanning for Rust source files...");
    
    // Collect Rust source files
    let mut rust_files = Vec::new();
    let mut total_size = 0;
    
    collect_rust_files(test_dir, &mut rust_files, &mut total_size)?;
    
    println!("📊 Found {} Rust source files, total size: {:.2} MB", 
             rust_files.len(), total_size as f64 / 1024.0 / 1024.0);
    
    // Test NEXUS compression on a sample of files
    let sample_size = std::cmp::min(20, rust_files.len()); // Start with 20 files
    let sample_files = &rust_files[..sample_size];
    
    println!("🧪 Testing NEXUS compression on {} sample files...", sample_size);
    
    // Create NEXUS compression engine
    let mut config = EnhancedCompressionConfig::default();
    config.target_ratio = 5.0; // Target 5x compression
    config.enable_neuromorphic = true;
    config.enable_ai_scheduling = true;
    
    let mut engine = EnhancedCompressionEngine::new(config);
    
    let mut compression_results = Vec::new();
    let start_time = Instant::now();
    
    for (i, file_path) in sample_files.iter().enumerate() {
        println!("  Processing file {}/{}: {}", i + 1, sample_size, 
                 Path::new(file_path).file_name().unwrap().to_string_lossy());
        
        match test_nexus_compression(&mut engine, file_path).await {
            Ok(result) => compression_results.push(result),
            Err(e) => eprintln!("    ❌ Error: {}", e),
        }
    }
    
    let total_time = start_time.elapsed();
    
    // Analyze NEXUS results
    analyze_nexus_results(&compression_results, total_time)?;
    
    Ok(())
}

fn collect_rust_files(dir: &str, files: &mut Vec<String>, total_size: &mut u64) -> Result<(), Box<dyn std::error::Error>> {
    let entries = fs::read_dir(dir)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "rs" {
                    let file_size = fs::metadata(&path)?.len();
                    if file_size > 1000 && file_size < 100000 { // Files between 1KB and 100KB
                        files.push(path.to_string_lossy().to_string());
                        *total_size += file_size;
                    }
                }
            }
        } else if path.is_dir() {
            collect_rust_files(&path.to_string_lossy(), files, total_size)?;
        }
    }
    
    Ok(())
}

#[derive(Debug)]
struct NexusCompressionResult {
    file_path: String,
    original_size: u64,
    compressed_size: u64,
    compression_ratio: f64,
    ast_nodes: usize,
    patterns_identified: usize,
    processing_time: std::time::Duration,
}

async fn test_nexus_compression(
    engine: &mut EnhancedCompressionEngine, 
    file_path: &str
) -> Result<NexusCompressionResult, Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    
    // Read the file
    let content = fs::read_to_string(file_path)?;
    let original_size = content.len() as u64;
    
    // Parse into AST
    let ast = parse_rust_source(&content);
    let ast_nodes = ast.nodes.len();
    
    // Apply NEXUS compression
    let start_compression = Instant::now();
    let result = engine.compress_ast(&ast).await?;
    let compression_time = start_compression.elapsed();
    
    let compressed_size = result.compressed_size;
    let compression_ratio = if compressed_size > 0 {
        original_size as f64 / compressed_size as f64
    } else {
        1.0
    };
    
    let total_time = start_time.elapsed();
    
    println!("    ✅ {}: {:.2}x compression, {} → {} bytes, {} patterns", 
             Path::new(file_path).file_name().unwrap().to_string_lossy(),
             compression_ratio, original_size, compressed_size, result.patterns_identified);
    
    Ok(NexusCompressionResult {
        file_path: file_path.to_string(),
        original_size,
        compressed_size: compressed_size as u64,
        compression_ratio,
        ast_nodes,
        patterns_identified: result.patterns_identified,
        processing_time: total_time,
    })
}

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

fn analyze_nexus_results(results: &[NexusCompressionResult], total_time: std::time::Duration) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🚀 NEXUS COMPRESSION RESULTS ANALYSIS");
    println!("{}", "=".repeat(60));
    
    if results.is_empty() {
        println!("❌ No compression results to analyze");
        return Ok(());
    }
    
    let total_original = results.iter().map(|r| r.original_size).sum::<u64>();
    let total_compressed = results.iter().map(|r| r.compressed_size).sum::<u64>();
    let total_patterns = results.iter().map(|r| r.patterns_identified).sum::<usize>();
    
    let overall_ratio = if total_compressed > 0 {
        total_original as f64 / total_compressed as f64
    } else {
        1.0
    };
    
    let avg_ratio = results.iter().map(|r| r.compression_ratio).sum::<f64>() / results.len() as f64;
    let max_ratio = results.iter().map(|r| r.compression_ratio).fold(1.0, f64::max);
    let min_ratio = results.iter().map(|r| r.compression_ratio).fold(f64::INFINITY, f64::min);
    
    println!("📁 Files processed: {}", results.len());
    println!("⏱️  Total processing time: {:.2?}", total_time);
    println!("📊 Overall compression ratio: {:.2}x", overall_ratio);
    println!("📈 Average compression ratio: {:.2}x", avg_ratio);
    println!("🚀 Best compression: {:.2}x", max_ratio);
    println!("📉 Worst compression: {:.2}x", min_ratio);
    println!("💾 Size reduction: {:.1}%", (1.0 - 1.0/overall_ratio) * 100.0);
    println!("🔍 Total patterns identified: {}", total_patterns);
    println!("📊 Average patterns per file: {:.1}", total_patterns as f64 / results.len() as f64);
    
    // Show top performers
    let mut top_files: Vec<_> = results.iter().collect();
    top_files.sort_by(|a, b| b.compression_ratio.partial_cmp(&a.compression_ratio).unwrap());
    
    println!("\n🏆 TOP NEXUS COMPRESSION PERFORMERS:");
    for (i, result) in top_files.iter().take(5).enumerate() {
        let filename = Path::new(&result.file_path).file_name().unwrap().to_string_lossy();
        println!("  {}. {}: {:.2}x ({:.1}% reduction, {} patterns)", 
                 i + 1, filename, result.compression_ratio, 
                 (1.0 - 1.0/result.compression_ratio) * 100.0,
                 result.patterns_identified);
    }
    
    // Show size distribution
    println!("\n📊 COMPRESSION RATIO DISTRIBUTION:");
    let mut ratio_ranges = vec![(1.0..1.5, 0), (1.5..2.0, 0), (2.0..3.0, 0), (3.0..5.0, 0), (5.0..f64::INFINITY, 0)];
    
    for result in results {
        for (range, count) in &mut ratio_ranges {
            if range.contains(&result.compression_ratio) {
                *count += 1;
                break;
            }
        }
    }
    
    for (range, count) in ratio_ranges {
        let range_str = if range.end == f64::INFINITY {
            format!("{:.1}+x", range.start)
        } else {
            format!("{:.1}-{:.1}x", range.start, range.end)
        };
        println!("  {}: {} files", range_str, count);
    }
    
    Ok(())
}
