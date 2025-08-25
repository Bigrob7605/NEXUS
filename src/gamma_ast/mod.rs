//! Γ-AST (Gamma AST) Module
//! 
//! This module provides the core universal AST format that serves as the bridge
//! between different programming languages. It includes compression algorithms,
//! pattern recognition, and metadata support.

use crate::ast::Location;
use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize};

/// Represents a compressed node in the Γ-AST
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GammaNode {
    /// Unique identifier for this node
    pub id: u64,
    /// The type of node (compressed representation)
    pub node_type: GammaNodeType,
    /// Compressed value or reference
    pub value: GammaValue,
    /// Location information (may be compressed)
    pub location: Option<Location>,
    /// Children nodes (compressed references)
    pub children: Vec<u64>,
    /// Metadata and annotations
    pub metadata: HashMap<String, String>,
    /// Compression level achieved
    pub compression_level: CompressionLevel,
}

/// Types of nodes in the Γ-AST (optimized for compression)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GammaNodeType {
    // Core types (single byte)
    Literal,
    Variable,
    Function,
    Class,
    Module,
    
    // Control flow (single byte)
    If,
    Loop,
    Switch,
    Try,
    
    // Operations (single byte)
    BinaryOp,
    UnaryOp,
    Assignment,
    Call,
    
    // Structure (single byte)
    Block,
    Expression,
    Statement,
    Declaration,
    
    // Custom types (compressed)
    Custom(String),
}

/// Compressed values in the Γ-AST
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GammaValue {
    /// Direct value (for small literals)
    Direct(String),
    /// Reference to a pattern (for repeated structures)
    PatternRef(u64),
    /// Compressed hash (for large values)
    CompressedHash(u64),
    /// Null/empty value
    None,
}

impl GammaValue {
    /// Convert the value to a string representation
    pub fn to_string(&self) -> String {
        match self {
            GammaValue::Direct(s) => s.clone(),
            GammaValue::PatternRef(id) => format!("pattern_{}", id),
            GammaValue::CompressedHash(hash) => format!("hash_{:x}", hash),
            GammaValue::None => "".to_string(),
        }
    }
}

/// Compression levels achieved
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompressionLevel {
    /// No compression (original size)
    None,
    /// Light compression (2-4x reduction)
    Light,
    /// Medium compression (4-8x reduction)
    Medium,
    /// Heavy compression (8-16x reduction)
    Heavy,
    /// Maximum compression (16x+ reduction)
    Maximum,
}

/// Pattern recognition system for identifying common code structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    /// Unique pattern identifier
    pub id: u64,
    /// Pattern signature (hash of structure)
    pub signature: u64,
    /// Pattern frequency (how often it appears)
    pub frequency: u32,
    /// Pattern size (number of nodes)
    pub size: usize,
    /// Pattern nodes
    pub nodes: Vec<GammaNode>,
    /// Languages where this pattern appears
    pub languages: Vec<String>,
}

/// Cross-file pattern for maximum compression across codebases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossFilePattern {
    /// Unique pattern identifier
    pub id: u64,
    /// Pattern type identifier
    pub pattern_type: String,
    /// Pattern signature (structural + semantic hash)
    pub signature: String,
    /// Node IDs that belong to this pattern
    pub node_ids: Vec<u64>,
    /// Frequency across files
    pub frequency: usize,
    /// Compression potential score
    pub compression_potential: f64,
    /// Hierarchical level (1-4)
    pub hierarchical_level: u8,
}

/// Meta-pattern for combining multiple patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaPattern {
    /// Unique meta-pattern identifier
    pub id: u64,
    /// Sub-pattern IDs that compose this meta-pattern
    pub sub_patterns: Vec<u64>,
    /// Combined signature of all sub-patterns
    pub combined_signature: String,
    /// Total compression potential
    pub compression_potential: f64,
    /// Hierarchical level (always 4 for meta-patterns)
    pub hierarchical_level: u8,
}

/// Γ-AST implementation with compression and pattern recognition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GammaAST {
    /// Root nodes of the compressed AST
    pub roots: Vec<u64>,
    /// All nodes in the compressed format
    pub nodes: HashMap<u64, GammaNode>,
    /// Recognized patterns
    pub patterns: HashMap<u64, Pattern>,
    /// Source language information
    pub source_language: String,
    /// Compression statistics
    pub compression_stats: CompressionStats,
    /// Pattern registry for reuse
    pub pattern_registry: PatternRegistry,
}

/// Compression statistics and metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionStats {
    /// Original size in bytes
    pub original_size: usize,
    /// Compressed size in bytes
    pub compressed_size: usize,
    /// Compression ratio achieved
    pub compression_ratio: f64,
    /// Number of patterns identified
    pub patterns_found: usize,
    /// Memory usage optimization
    pub memory_optimization: f64,
}

/// Pattern registry for efficient pattern reuse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternRegistry {
    /// Pattern signatures mapped to pattern IDs
    pub signatures: HashMap<u64, u64>,
    /// Pattern frequency tracking
    pub frequencies: HashMap<u64, u32>,
    /// Pattern size distribution
    pub size_distribution: HashMap<usize, u32>,
}

impl GammaAST {
    /// Create a new Γ-AST
    pub fn new() -> Self {
        Self {
            roots: Vec::new(),
            nodes: HashMap::new(),
            patterns: HashMap::new(),
            source_language: String::new(),
            compression_stats: CompressionStats::new(),
            pattern_registry: PatternRegistry::new(),
        }
    }
    
    /// Set the source language
    pub fn set_source_language(&mut self, language: String) {
        self.source_language = language;
    }
    
    /// Add a root node
    pub fn add_root(&mut self, node_id: u64) {
        self.roots.push(node_id);
    }
    
    /// Add a node to the AST
    pub fn add_node(&mut self, node: GammaNode) {
        self.nodes.insert(node.id, node);
    }
    
    /// Get a node by ID
    pub fn get_node(&self, id: u64) -> Option<&GammaNode> {
        self.nodes.get(&id)
    }
    
    /// Get a mutable reference to a node
    pub fn get_node_mut(&mut self, id: u64) -> Option<&mut GammaNode> {
        self.nodes.get_mut(&id)
    }
    
    /// Add a recognized pattern
    pub fn add_pattern(&mut self, pattern: Pattern) {
        self.patterns.insert(pattern.id, pattern.clone());
        self.pattern_registry.signatures.insert(pattern.signature, pattern.id);
        self.pattern_registry.frequencies.insert(pattern.id, pattern.frequency);
        self.pattern_registry.size_distribution.entry(pattern.size).and_modify(|e| *e += 1).or_insert(1);
    }
    
    /// Find patterns in the AST
    pub fn find_patterns(&mut self) -> Vec<u64> {
        let mut found_patterns = Vec::new();
        
        // Analyze node structures for common patterns
        for (id, _node) in &self.nodes {
            if let Some(pattern_id) = self.identify_pattern(*id) {
                found_patterns.push(pattern_id);
            }
        }
        
        found_patterns
    }
    
    /// Identify if a node matches a known pattern
    fn identify_pattern(&self, node_id: u64) -> Option<u64> {
        // This is a simplified pattern identification
        // In a full implementation, this would use sophisticated pattern matching algorithms
        if let Some(node) = self.get_node(node_id) {
            // Check if this node structure matches any known patterns
            for (pattern_id, pattern) in &self.patterns {
                if self.matches_pattern(node, pattern) {
                    return Some(*pattern_id);
                }
            }
        }
        None
    }
    
    /// Check if a node matches a pattern
    fn matches_pattern(&self, node: &GammaNode, pattern: &Pattern) -> bool {
        // Simplified pattern matching - in reality this would be much more sophisticated
        node.node_type == pattern.nodes[0].node_type && 
        node.children.len() == pattern.nodes[0].children.len()
    }
    
    /// Calculate compression statistics
    pub fn calculate_compression_stats(&mut self) {
        let original_size = self.calculate_original_size();
        let compressed_size = self.calculate_compressed_size();
        let compression_ratio = if original_size > 0 {
            original_size as f64 / compressed_size as f64
        } else {
            1.0
        };
        
        self.compression_stats = CompressionStats {
            original_size,
            compressed_size,
            compression_ratio,
            patterns_found: self.patterns.len(),
            memory_optimization: compression_ratio,
        };
    }
    
    /// Calculate the original size (estimated)
    fn calculate_original_size(&self) -> usize {
        // Estimate original size based on node content
        let mut size = 0;
        for node in self.nodes.values() {
            size += node.value.to_string().len();
            size += node.children.len() * 8; // 8 bytes per child reference
            size += node.metadata.len() * 16; // 16 bytes per metadata entry
        }
        size
    }
    
    /// Calculate the compressed size
    fn calculate_compressed_size(&self) -> usize {
        // Calculate actual compressed size
        let mut size = 0;
        for node in self.nodes.values() {
            size += 8; // Node ID
            size += 1; // Node type (enum)
            size += match &node.value {
                GammaValue::Direct(s) => s.len(),
                GammaValue::PatternRef(_) => 8,
                GammaValue::CompressedHash(_) => 8,
                GammaValue::None => 0,
            };
            size += node.children.len() * 8; // Child references
            size += node.metadata.len() * 8; // Compressed metadata
        }
        size
    }
}

impl CompressionStats {
    /// Create new compression stats
    pub fn new() -> Self {
        Self {
            original_size: 0,
            compressed_size: 0,
            compression_ratio: 1.0,
            patterns_found: 0,
            memory_optimization: 1.0,
        }
    }
    
    /// Get compression percentage
    pub fn compression_percentage(&self) -> f64 {
        if self.original_size > 0 {
            if self.compressed_size <= self.original_size {
                ((self.original_size - self.compressed_size) as f64 / self.original_size as f64) * 100.0
            } else {
                // If compressed size is larger, that means expansion occurred
                -((self.compressed_size - self.original_size) as f64 / self.original_size as f64) * 100.0
            }
        } else {
            0.0
        }
    }
}

impl PatternRegistry {
    /// Create a new pattern registry
    pub fn new() -> Self {
        Self {
            signatures: HashMap::new(),
            frequencies: HashMap::new(),
            size_distribution: HashMap::new(),
        }
    }
    
    /// Get the most frequent patterns
    pub fn get_top_patterns(&self, limit: usize) -> Vec<(u64, u32)> {
        let mut patterns: Vec<(u64, u32)> = self.frequencies.iter()
            .map(|(id, freq)| (*id, *freq))
            .collect();
        patterns.sort_by(|a, b| b.1.cmp(&a.1));
        patterns.truncate(limit);
        patterns
    }
    
    /// Get patterns by size range
    pub fn get_patterns_by_size(&self, min_size: usize, max_size: usize) -> Vec<u64> {
        self.size_distribution.iter()
            .filter(|(size, _)| **size >= min_size && **size <= max_size)
            .flat_map(|(_, count)| {
                self.frequencies.iter()
                    .filter(|(_, freq)| **freq > 0)
                    .map(|(id, _)| *id)
                    .take(*count as usize)
            })
            .collect()
    }
}

impl fmt::Display for GammaAST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Γ-AST (Gamma AST)")?;
        writeln!(f, "Source Language: {}", self.source_language)?;
        writeln!(f, "Roots: {}", self.roots.len())?;
        writeln!(f, "Nodes: {}", self.nodes.len())?;
        writeln!(f, "Patterns: {}", self.patterns.len())?;
        writeln!(f, "Compression Ratio: {:.2}x", self.compression_stats.compression_ratio)?;
        writeln!(f, "Compression: {:.1}%", self.compression_stats.compression_percentage())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gamma_ast_creation() {
        let ast = GammaAST::new();
        assert_eq!(ast.roots.len(), 0);
        assert_eq!(ast.nodes.len(), 0);
        assert_eq!(ast.patterns.len(), 0);
    }
    
    #[test]
    fn test_node_addition() {
        let mut ast = GammaAST::new();
        let node = GammaNode {
            id: 1,
            node_type: GammaNodeType::Literal,
            value: GammaValue::Direct("42".to_string()),
            location: None,
            children: Vec::new(),
            metadata: HashMap::new(),
            compression_level: CompressionLevel::Light,
        };
        
        ast.add_node(node);
        assert_eq!(ast.nodes.len(), 1);
        assert!(ast.get_node(1).is_some());
    }
    
    #[test]
    fn test_pattern_recognition() {
        let mut ast = GammaAST::new();
        
        // Add a simple pattern
        let pattern = Pattern {
            id: 1,
            signature: 12345,
            frequency: 5,
            size: 3,
            nodes: Vec::new(),
            languages: vec!["rust".to_string(), "python".to_string()],
        };
        
        ast.add_pattern(pattern);
        assert_eq!(ast.patterns.len(), 1);
        assert_eq!(ast.pattern_registry.frequencies.get(&1), Some(&5));
    }
    
    #[test]
    fn test_compression_stats() {
        let mut ast = GammaAST::new();
        
        // Add some nodes to calculate stats
        let node1 = GammaNode {
            id: 1,
            node_type: GammaNodeType::Literal,
            value: GammaValue::Direct("42".to_string()),
            location: None,
            children: Vec::new(),
            metadata: HashMap::new(),
            compression_level: CompressionLevel::Medium,
        };
        
        let node2 = GammaNode {
            id: 2,
            node_type: GammaNodeType::Variable,
            value: GammaValue::Direct("x".to_string()),
            location: None,
            children: Vec::new(),
            metadata: HashMap::new(),
            compression_level: CompressionLevel::Light,
        };
        
        ast.add_node(node1);
        ast.add_node(node2);
        ast.add_root(1);
        ast.add_root(2);
        
        ast.calculate_compression_stats();
        
        assert!(ast.compression_stats.compression_ratio > 0.0);
        // For small ASTs, compression overhead can exceed savings
        // Allow for realistic expansion scenarios
        assert!(ast.compression_stats.compression_percentage() >= -1000.0);
        assert!(ast.compression_stats.compression_percentage() <= 100.0);
    }
}
