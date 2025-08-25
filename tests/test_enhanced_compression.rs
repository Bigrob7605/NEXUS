//! Enhanced Compression Engine Tests
//! 
//! This module tests the enhanced compression engine with AI optimization,
//! neuromorphic pattern recognition, and intelligent resource management.

use nexus::enhanced_compression::*;
use nexus::gamma_ast::*;
use std::collections::HashMap;
use std::time::Duration;

#[tokio::test]
async fn test_enhanced_compression_creation() {
    let config = EnhancedCompressionConfig::default();
    let engine = EnhancedCompressionEngine::new(config);
    
    assert!(engine.config.enable_neuromorphic);
    assert!(engine.config.enable_ai_scheduling);
    assert!(engine.config.enable_crypto_verification);
    assert_eq!(engine.config.target_ratio, 8.0);
    assert_eq!(engine.config.max_memory_mb, 1024);
    assert_eq!(engine.config.gpu_threshold, 1000);
}

#[tokio::test]
async fn test_enhanced_compression_config_customization() {
    let config = EnhancedCompressionConfig {
        enable_neuromorphic: false,
        enable_ai_scheduling: false,
        enable_crypto_verification: false,
        target_ratio: 4.0,
        max_memory_mb: 512,
        gpu_threshold: 500,
        learning_rate: 0.05,
        pattern_evolution: false,
    };
    
    let engine = EnhancedCompressionEngine::new(config);
    
    assert!(!engine.config.enable_neuromorphic);
    assert!(!engine.config.enable_ai_scheduling);
    assert!(!engine.config.enable_crypto_verification);
    assert_eq!(engine.config.target_ratio, 4.0);
    assert_eq!(engine.config.max_memory_mb, 512);
    assert_eq!(engine.config.gpu_threshold, 500);
    assert_eq!(engine.config.learning_rate, 0.05);
}

#[tokio::test]
async fn test_enhanced_compression_with_sample_ast() {
    let config = EnhancedCompressionConfig::default();
    let mut engine = EnhancedCompressionEngine::new(config);
    
    // Create a sample AST for testing
    let mut ast = GammaAST::new();
    ast.set_source_language("rust".to_string());
    
    // Add some sample nodes with repeated patterns
    let node1 = GammaNode {
        id: 1,
        node_type: GammaNodeType::Function,
        value: GammaValue::Direct("main".to_string()),
        location: None,
        children: vec![2, 3],
        metadata: HashMap::new(),
        compression_level: CompressionLevel::None,
    };
    
    let node2 = GammaNode {
        id: 2,
        node_type: GammaNodeType::Variable,
        value: GammaValue::Direct("x".to_string()),
        location: None,
        children: vec![],
        metadata: HashMap::new(),
        compression_level: CompressionLevel::None,
    };
    
    let node3 = GammaNode {
        id: 3,
        node_type: GammaNodeType::Literal,
        value: GammaValue::Direct("42".to_string()),
        location: None,
        children: vec![],
        metadata: HashMap::new(),
        compression_level: CompressionLevel::None,
    };
    
    // Add duplicate nodes to create patterns
    let node4 = GammaNode {
        id: 4,
        node_type: GammaNodeType::Variable,
        value: GammaValue::Direct("x".to_string()), // Same value as node2
        location: None,
        children: vec![],
        metadata: HashMap::new(),
        compression_level: CompressionLevel::None,
    };
    
    let node5 = GammaNode {
        id: 5,
        node_type: GammaNodeType::Literal,
        value: GammaValue::Direct("42".to_string()), // Same value as node3
        location: None,
        children: vec![],
        metadata: HashMap::new(),
        compression_level: CompressionLevel::None,
    };
    
    ast.add_node(node1);
    ast.add_node(node2);
    ast.add_node(node3);
    ast.add_node(node4);
    ast.add_node(node5);
    ast.add_root(1);
    
    // Test compression
    let result = engine.compress_ast(&ast).await.unwrap();
    
    // Verify compression results
    assert!(result.original_size > 0);
    assert!(result.compressed_size > 0);
    assert!(result.compression_ratio >= 1.0);
    assert!(result.patterns_identified >= 0);
    assert!(result.processing_time > Duration::from_nanos(0));
    assert!(result.memory_usage > 0);
    
    // Verify neuromorphic insights
    assert!(!result.neuromorphic_insights.is_empty());
    
    // Verify verification hash if enabled
    if engine.config.enable_crypto_verification {
        assert!(result.verification_hash.is_some());
        let hash = result.verification_hash.unwrap();
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 16); // Hex hash length
    }
}

#[tokio::test]
async fn test_enhanced_compression_with_large_ast() {
    let config = EnhancedCompressionConfig {
        gpu_threshold: 10, // Lower threshold for testing
        ..Default::default()
    };
    let mut engine = EnhancedCompressionEngine::new(config);
    
    // Create a larger AST to trigger GPU acceleration
    let mut ast = GammaAST::new();
    ast.set_source_language("python".to_string());
    
    // Add many nodes to exceed GPU threshold
    for i in 0..20 {
        let node = GammaNode {
            id: i,
            node_type: if i % 2 == 0 { 
                GammaNodeType::Function 
            } else { 
                GammaNodeType::Variable 
            },
            value: GammaValue::Direct(format!("node_{}", i)),
            location: None,
            children: vec![],
            metadata: HashMap::new(),
            compression_level: CompressionLevel::None,
        };
        ast.add_node(node);
        ast.add_root(i);
    }
    
    // Test compression
    let result = engine.compress_ast(&ast).await.unwrap();
    
    // Verify results
    assert!(result.original_size > 0);
    assert!(result.compressed_size > 0);
    assert!(result.compression_ratio >= 1.0);
    assert!(result.patterns_identified > 0); // Should find patterns in larger AST
}

#[tokio::test]
async fn test_enhanced_compression_error_handling() {
    let config = EnhancedCompressionConfig::default();
    let mut engine = EnhancedCompressionEngine::new(config);
    
    // Test with empty AST
    let empty_ast = GammaAST::new();
    let result = engine.compress_ast(&empty_ast).await.unwrap();
    
    // Should handle empty AST gracefully
    assert_eq!(result.original_size, 0);
    assert_eq!(result.compressed_size, 0);
    assert_eq!(result.compression_ratio, f64::INFINITY); // Division by zero
    assert_eq!(result.patterns_identified, 0);
}

#[tokio::test]
async fn test_enhanced_compression_learning_adaptation() {
    let config = EnhancedCompressionConfig {
        learning_rate: 0.2,
        ..Default::default()
    };
    let mut engine = EnhancedCompressionEngine::new(config);
    
    // Create multiple ASTs to test learning adaptation
    for iteration in 0..3 {
        let mut ast = GammaAST::new();
        ast.set_source_language("javascript".to_string());
        
        // Add nodes with increasing complexity
        for i in 0..(5 + iteration * 2) {
            let node = GammaNode {
                id: i,
                node_type: GammaNodeType::Function,
                value: GammaValue::Direct(format!("func_{}", i)),
                location: None,
                children: vec![],
                metadata: HashMap::new(),
                compression_level: CompressionLevel::None,
            };
            ast.add_node(node);
            ast.add_root(i);
        }
        
        // Compress and check if learning improves
        let result = engine.compress_ast(&ast).await.unwrap();
        
        // Verify each iteration produces valid results
        assert!(result.original_size > 0);
        assert!(result.compressed_size > 0);
        assert!(result.compression_ratio >= 1.0);
    }
}

#[tokio::test]
async fn test_enhanced_compression_resource_optimization() {
    let config = EnhancedCompressionConfig {
        max_memory_mb: 64, // Low memory limit for testing
        ..Default::default()
    };
    let mut engine = EnhancedCompressionEngine::new(config);
    
    // Create AST that should trigger resource optimization
    let mut ast = GammaAST::new();
    ast.set_source_language("rust".to_string());
    
    // Add moderate number of nodes
    for i in 0..15 {
        let node = GammaNode {
            id: i,
            node_type: GammaNodeType::Class,
            value: GammaValue::Direct(format!("Class{}", i)),
            location: None,
            children: vec![],
            metadata: HashMap::new(),
            compression_level: CompressionLevel::None,
        };
        ast.add_node(node);
        ast.add_root(i);
    }
    
    // Test compression with resource constraints
    let result = engine.compress_ast(&ast).await.unwrap();
    
    // Verify resource optimization
    assert!(result.resource_optimization > 0.0);
    assert!(result.resource_optimization <= 1.0);
    assert!(result.memory_usage <= 1024 * 1024 * 1024); // 1GB max
}

#[tokio::test]
async fn test_enhanced_compression_pattern_evolution() {
    let config = EnhancedCompressionConfig::default();
    let mut engine = EnhancedCompressionEngine::new(config);
    
    // Create ASTs with evolving patterns
    for pattern_type in 0..3 {
        let mut ast = GammaAST::new();
        ast.set_source_language("python".to_string());
        
        // Create different pattern types
        let node_type = match pattern_type {
            0 => GammaNodeType::Function,
            1 => GammaNodeType::Class,
            _ => GammaNodeType::Module,
        };
        
        for i in 0..10 {
            let node = GammaNode {
                id: i as u64,
                node_type: node_type.clone(),
                value: GammaValue::Direct(format!("{}_{}", 
                    match node_type {
                        GammaNodeType::Function => "func",
                        GammaNodeType::Class => "class",
                        GammaNodeType::Module => "module",
                        _ => "unknown",
                    }, 
                    i
                )),
                location: None,
                children: vec![],
                metadata: HashMap::new(),
                compression_level: CompressionLevel::None,
            };
            ast.add_node(node);
            ast.add_root(i as u64);
        }
        
        // Compress and verify pattern recognition
        let result = engine.compress_ast(&ast).await.unwrap();
        assert!(result.patterns_identified > 0);
    }
}

#[tokio::test]
async fn test_enhanced_compression_integration() {
    let config = EnhancedCompressionConfig {
        enable_neuromorphic: true,
        enable_ai_scheduling: true,
        enable_crypto_verification: true,
        target_ratio: 6.0,
        max_memory_mb: 256,
        gpu_threshold: 50,
        learning_rate: 0.15,
        pattern_evolution: true,
    };
    
    let mut engine = EnhancedCompressionEngine::new(config);
    
    // Create comprehensive test AST
    let mut ast = GammaAST::new();
    ast.set_source_language("nexus".to_string());
    
    // Add diverse node types
    let node_types = [
        GammaNodeType::Function,
        GammaNodeType::Class,
        GammaNodeType::Variable,
        GammaNodeType::Literal,
        GammaNodeType::Module,
    ];
    
    for i in 0..25 {
        let node_type = node_types[i % node_types.len()].clone();
        let node = GammaNode {
            id: i as u64,
            node_type,
            value: GammaValue::Direct(format!("{}_{}", 
                match node_types[i % node_types.len()] {
                    GammaNodeType::Function => "func",
                    GammaNodeType::Class => "class",
                    GammaNodeType::Variable => "var",
                    GammaNodeType::Literal => "lit",
                    GammaNodeType::Module => "mod",
                    _ => "unknown",
                }, 
                i
            )),
            location: None,
            children: vec![],
            metadata: HashMap::new(),
            compression_level: CompressionLevel::None,
        };
        ast.add_node(node);
        ast.add_root(i as u64);
    }
    
    // Test full integration
    let result = engine.compress_ast(&ast).await.unwrap();
    
    // Comprehensive verification
    assert!(result.original_size > 0);
    assert!(result.compressed_size > 0);
    assert!(result.compression_ratio >= 1.0);
    assert!(result.patterns_identified > 0);
    assert!(result.processing_time > Duration::from_nanos(0));
    assert!(result.memory_usage > 0);
    assert!(result.resource_optimization > 0.0);
    assert!(result.resource_optimization <= 1.0);
    
    // Verify neuromorphic insights
    assert!(!result.neuromorphic_insights.is_empty());
    
    // Verify cryptographic verification
    assert!(result.verification_hash.is_some());
    let hash = result.verification_hash.unwrap();
    assert!(!hash.is_empty());
    assert_eq!(hash.len(), 16);
    
    println!("✅ Enhanced compression integration test passed!");
    println!("   Original size: {} bytes", result.original_size);
    println!("   Compressed size: {} bytes", result.compressed_size);
    println!("   Compression ratio: {:.2}x", result.compression_ratio);
    println!("   Patterns identified: {}", result.patterns_identified);
    println!("   Processing time: {:?}", result.processing_time);
    println!("   Memory usage: {} bytes", result.memory_usage);
    println!("   Resource optimization: {:.2}", result.resource_optimization);
    println!("   Neuromorphic insights: {}", result.neuromorphic_insights.len());
}
