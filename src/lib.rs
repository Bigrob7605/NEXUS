//! NEXUS Library
//! 
//! This library provides the core NEXUS functionality including:
//! - Enhanced compression engine with AI optimization
//! - Neuromorphic memory system
//! - AI scheduler for resource management
//! - Γ-AST representation and processing
//! - Language bridges and interoperability
//! - GPU acceleration for universal information folding

pub mod parser;
pub mod ast;
pub mod gamma_ast;
pub mod neuromem;
pub mod ai_scheduler;
pub mod enhanced_compression;
pub mod demo_enhanced_compression;
pub mod gpu_acceleration;

pub mod tests;

// Re-export main types for convenience
pub use enhanced_compression::{EnhancedCompressionEngine, EnhancedCompressionConfig, CompressionResult};
pub use gamma_ast::{GammaAST, GammaNode, Pattern, CompressionLevel, CompressionStats};
pub use neuromem::{Neuromem, MemoryRegion, AccessPattern, MemorySpike, LearningEngine};
pub use ai_scheduler::{AIScheduler, AIProcess, GPUMemoryManager, SchedulerError};
pub use gpu_acceleration::{GPUAccelerationEngine, GPUConfig, UniversalPattern, GPUPatternResult, GPUDevice};

// Re-export test types for integration tests
pub use tests::{TestResult, TestSuite};
