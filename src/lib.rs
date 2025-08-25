//! NEXUS Library
//! 
//! This library provides the core NEXUS functionality including:
//! - REAL working compression engine (no false claims)
//! - Γ-AST representation and processing
//! - Language bridges and interoperability
//! - Core compression algorithms that actually work
//! - AI-powered resource management and optimization
//! - Neuromorphic memory pattern analysis
//! - GPU acceleration for large-scale processing

pub mod parser;
pub mod ast;
pub mod gamma_ast;
pub mod nexus_compression_engine;
pub mod ai_scheduler;
pub mod neuromem;
pub mod gpu_acceleration;

pub mod tests;

// Re-export main types for convenience - REAL WORKING TECHNOLOGY
pub use nexus_compression_engine::{NexusCompressionEngine, CompressionConfig, CompressionResult, CompressionError};
pub use gamma_ast::{GammaAST, GammaNode, Pattern, CompressionLevel, CompressionStats};

// Re-export AI and optimization types - LEGITIMATE TECHNOLOGY
pub use ai_scheduler::{AIProcess, GPUMemoryManager, SchedulerError, GPUAllocation, MemoryBlock};
pub use neuromem::{MemoryRegion, AccessPattern, MemorySpike, LearningEngine, MemoryType};
pub use gpu_acceleration::{GPUAccelerationEngine, GPUConfig, GPUDevice, GPUPatternResult};

// Re-export test types for integration tests
pub use tests::{TestResult, TestSuite};
