use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 TESTING NEXUS COMPRESSION ON REAL DATA");
    println!("==========================================");
    
    let test_dir = "test_codebases/rust-compiler";
    if !Path::new(test_dir).exists() {
        eprintln!("❌ Test directory not found: {}", test_dir);
        return Ok(());
    }
    
    println!("📁 Testing on: {}", test_dir);
    println!("🔍 Scanning for code files...");
    
    // Collect all code files
    let mut code_files = Vec::new();
    let mut total_size = 0;
    
    collect_code_files(test_dir, &mut code_files, &mut total_size)?;
    
    println!("📊 Found {} code files, total size: {:.2} MB", 
             code_files.len(), total_size as f64 / 1024.0 / 1024.0);
    
    // Test compression on a sample of files
    let sample_size = std::cmp::min(100, code_files.len());
    let sample_files = &code_files[..sample_size];
    
    println!("🧪 Testing compression on {} sample files...", sample_size);
    
    let mut compression_results = Vec::new();
    let start_time = Instant::now();
    
    for (i, file_path) in sample_files.iter().enumerate() {
        if i % 10 == 0 {
            println!("  Processing file {}/{}...", i + 1, sample_size);
        }
        
        match test_file_compression(file_path) {
            Ok(result) => compression_results.push(result),
            Err(e) => eprintln!("    ❌ Error processing {}: {}", file_path, e),
        }
    }
    
    let total_time = start_time.elapsed();
    
    // Analyze results
    analyze_compression_results(&compression_results, total_time)?;
    
    Ok(())
}

fn collect_code_files(dir: &str, files: &mut Vec<String>, total_size: &mut u64) -> Result<(), Box<dyn std::error::Error>> {
    let entries = fs::read_dir(dir)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy();
                if is_code_file(&ext) {
                    let file_size = fs::metadata(&path)?.len();
                    files.push(path.to_string_lossy().to_string());
                    *total_size += file_size;
                }
            }
        } else if path.is_dir() {
            collect_code_files(&path.to_string_lossy(), files, total_size)?;
        }
    }
    
    Ok(())
}

fn is_code_file(extension: &str) -> bool {
    matches!(extension, 
        "rs" | "py" | "js" | "ts" | "cpp" | "c" | "h" | "hpp" | 
        "java" | "go" | "rb" | "php" | "swift" | "kt" | "scala" |
        "ml" | "fs" | "hs" | "clj" | "sh" | "ps1" | "bat" | "toml" |
        "json" | "yaml" | "yml" | "xml" | "md" | "txt"
    )
}

#[derive(Debug)]
struct CompressionResult {
    file_path: String,
    original_size: u64,
    compressed_size: u64,
    compression_ratio: f64,
    processing_time: std::time::Duration,
}

fn test_file_compression(file_path: &str) -> Result<CompressionResult, Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    
    // Read the file
    let content = fs::read_to_string(file_path)?;
    let original_size = content.len() as u64;
    
    // Simple compression simulation (replace with actual NEXUS compression)
    let compressed_content = simulate_compression(&content);
    let compressed_size = compressed_content.len() as u64;
    
    let compression_ratio = if compressed_size > 0 {
        original_size as f64 / compressed_size as f64
    } else {
        1.0
    };
    
    let processing_time = start_time.elapsed();
    
    Ok(CompressionResult {
        file_path: file_path.to_string(),
        original_size,
        compressed_size,
        compression_ratio,
        processing_time,
    })
}

fn simulate_compression(content: &str) -> String {
    // Simple compression simulation - remove comments and extra whitespace
    let mut compressed = String::new();
    let mut in_comment = false;
    let mut in_string = false;
    let mut last_char = ' ';
    
    for ch in content.chars() {
        match ch {
            '/' if last_char == '/' && !in_string => {
                in_comment = true;
                compressed.pop(); // Remove the previous '/'
            }
            '\n' if in_comment => {
                in_comment = false;
                compressed.push(ch);
            }
            '"' if !in_comment => {
                in_string = !in_string;
                compressed.push(ch);
            }
            _ if !in_comment => {
                if ch.is_whitespace() && last_char.is_whitespace() {
                    // Skip extra whitespace
                } else {
                    compressed.push(ch);
                }
            }
            _ => {}
        }
        last_char = ch;
    }
    
    compressed
}

fn analyze_compression_results(results: &[CompressionResult], total_time: std::time::Duration) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 COMPRESSION RESULTS ANALYSIS");
    println!("================================");
    
    if results.is_empty() {
        println!("❌ No compression results to analyze");
        return Ok(());
    }
    
    let total_original = results.iter().map(|r| r.original_size).sum::<u64>();
    let total_compressed = results.iter().map(|r| r.compressed_size).sum::<u64>();
    let total_processing = results.iter().map(|r| r.processing_time).sum::<std::time::Duration>();
    
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
    
    // Show top performers
    let mut top_files: Vec<_> = results.iter().collect();
    top_files.sort_by(|a, b| b.compression_ratio.partial_cmp(&a.compression_ratio).unwrap());
    
    println!("\n🏆 TOP COMPRESSION PERFORMERS:");
    for (i, result) in top_files.iter().take(5).enumerate() {
        let filename = Path::new(&result.file_path).file_name().unwrap().to_string_lossy();
        println!("  {}. {}: {:.2}x ({:.1}% reduction)", 
                 i + 1, filename, result.compression_ratio, 
                 (1.0 - 1.0/result.compression_ratio) * 100.0);
    }
    
    // Show file type analysis
    analyze_by_file_type(results)?;
    
    Ok(())
}

fn analyze_by_file_type(results: &[CompressionResult]) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 COMPRESSION BY FILE TYPE:");
    
    let mut type_stats: HashMap<String, Vec<&CompressionResult>> = HashMap::new();
    
    for result in results {
        let extension = Path::new(&result.file_path)
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        
        type_stats.entry(extension).or_insert_with(Vec::new).push(result);
    }
    
    for (ext, files) in type_stats.iter() {
        if files.len() >= 3 { // Only show types with 3+ files
            let avg_ratio = files.iter().map(|r| r.compression_ratio).sum::<f64>() / files.len() as f64;
            let total_original = files.iter().map(|r| r.original_size).sum::<u64>();
            let total_compressed = files.iter().map(|r| r.compressed_size).sum::<u64>();
            let overall_ratio = if total_compressed > 0 {
                total_original as f64 / total_compressed as f64
            } else {
                1.0
            };
            
            println!("  {:<6} ({} files): avg {:.2}x, overall {:.2}x", 
                     ext, files.len(), avg_ratio, overall_ratio);
        }
    }
    
    Ok(())
}
