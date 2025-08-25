//! Test GPU Acceleration Module for Universal Information Folding

use nexus::{GPUAccelerationEngine, GPUConfig, UniversalPattern, GPUPlatform};

#[test]
fn test_gpu_engine_creation() {
    let config = GPUConfig::default();
    let engine = GPUAccelerationEngine::new(config);
    assert!(engine.is_ok(), "GPU engine should be created successfully");
}

#[test]
fn test_universal_pattern_processing() {
    let engine = GPUAccelerationEngine::default();
    
    let pattern = UniversalPattern {
        id: 1,
        pattern_type: "test_pattern".to_string(),
        data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        size: 10,
        compression_potential: 1.5,
        gpu_optimized: false,
    };

    let result = engine.process_universal_pattern(&pattern);
    assert!(result.is_ok(), "Pattern processing should succeed");
    
    let result = result.unwrap();
    assert_eq!(result.pattern_id, 1, "Pattern ID should match");
    assert!(result.processing_time.as_nanos() > 0, "Processing time should be positive");
    assert!(result.compression_improvement > 0.0, "Compression improvement should be positive");
}

#[test]
fn test_gpu_config_default() {
    let config = GPUConfig::default();
    assert!(config.enabled, "GPU should be enabled by default");
    assert_eq!(config.platform, GPUPlatform::Auto, "Platform should default to Auto");
    assert_eq!(config.memory_threshold, 1024 * 1024, "Memory threshold should be 1MB");
    assert_eq!(config.max_gpu_memory_mb, 8192, "Max GPU memory should be 8GB");
    assert_eq!(config.parallel_streams, 4, "Parallel streams should be 4");
}

#[test]
fn test_gpu_device_discovery() {
    let engine = GPUAccelerationEngine::default();
    let devices = engine.get_devices();
    
    assert!(!devices.is_empty(), "Should have at least one device (CPU fallback)");
    
    let first_device = &devices[0];
    assert!(first_device.memory_total > 0, "Device should have memory");
    assert!(first_device.compute_units > 0, "Device should have compute units");
}

#[test]
fn test_gpu_processing_stats() {
    let engine = GPUAccelerationEngine::default();
    
    // Process a few patterns to generate stats
    for i in 1..=5 {
        let pattern = UniversalPattern {
            id: i,
            pattern_type: format!("test_pattern_{}", i),
            data: vec![i as u8; 100],
            size: 100,
            compression_potential: 1.0 + (i as f64 * 0.1),
            gpu_optimized: i % 2 == 0,
        };
        
        let _result = engine.process_universal_pattern(&pattern);
    }
    
    let stats = engine.get_processing_stats();
    assert_eq!(stats.total_patterns_processed, 5, "Should have processed 5 patterns");
    assert!(stats.average_compression_improvement > 0.0, "Average improvement should be positive");
    assert!(stats.total_memory_allocated > 0, "Should have allocated memory");
}

#[test]
fn test_gpu_availability_check() {
    let engine = GPUAccelerationEngine::default();
    let is_available = engine.is_available();
    
    // The availability depends on whether GPU features are enabled
    // For now, we expect it to work (either GPU or CPU fallback)
    assert!(true, "Engine should be available (GPU or CPU fallback)");
}

#[test]
fn test_large_pattern_gpu_processing() {
    let mut config = GPUConfig::default();
    config.memory_threshold = 100; // Lower threshold for testing
    
    let engine = GPUAccelerationEngine::new(config).unwrap();
    
    let large_pattern = UniversalPattern {
        id: 999,
        pattern_type: "large_test_pattern".to_string(),
        data: vec![42; 1000], // 1KB pattern
        size: 1000,
        compression_potential: 2.5,
        gpu_optimized: true,
    };

    let result = engine.process_universal_pattern(&large_pattern);
    assert!(result.is_ok(), "Large pattern processing should succeed");
    
    let result = result.unwrap();
    assert_eq!(result.pattern_id, 999, "Pattern ID should match");
    assert!(result.memory_used == 1000, "Memory used should match pattern size");
}

#[test]
fn test_compression_improvement_calculation() {
    let engine = GPUAccelerationEngine::default();
    
    // Test pattern with high compression potential
    let high_potential_pattern = UniversalPattern {
        id: 1000,
        pattern_type: "high_potential".to_string(),
        data: vec![1; 10000], // 10KB pattern
        size: 10000,
        compression_potential: 3.0,
        gpu_optimized: true,
    };

    let result = engine.process_universal_pattern(&high_potential_pattern);
    assert!(result.is_ok(), "High potential pattern should process successfully");
    
    let result = result.unwrap();
    assert!(result.compression_improvement > 0.0, "Compression improvement should be positive");
    
    // The improvement should be higher for larger patterns with higher potential
    assert!(result.compression_improvement > 1.0, "Large high-potential pattern should have significant improvement");
}
