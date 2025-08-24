//! Abstract Syntax Tree (AST) Module
//! 
//! This module provides the core data structures for representing code in a universal format.
//! The AST is designed to be language-agnostic and support efficient compression and conversion.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a node in the Abstract Syntax Tree
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    /// The type of this node
    pub node_type: NodeType,
    /// The value or content of this node
    pub value: String,
    /// Child nodes (for composite structures)
    pub children: Vec<Node>,
    /// Metadata associated with this node
    pub metadata: HashMap<String, String>,
    /// Source location information
    pub location: Option<Location>,
}

/// Represents the type of an AST node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeType {
    // Expression nodes
    Literal,
    Variable,
    BinaryOp,
    UnaryOp,
    FunctionCall,
    MethodCall,
    
    // Statement nodes
    Expression,
    Assignment,
    Declaration,
    Return,
    If,
    While,
    For,
    Block,
    
    // Declaration nodes
    Function,
    Class,
    Module,
    Import,
    
    // Type nodes
    TypeAnnotation,
    GenericType,
    UnionType,
    
    // Special nodes
    Comment,
    Whitespace,
    Error,
}

/// Represents a location in source code
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
    /// File path or identifier
    pub file: Option<String>,
}

/// Represents a universal type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Type {
    /// The base type name
    pub name: String,
    /// Generic type parameters
    pub generics: Vec<Type>,
    /// Whether this type is nullable
    pub nullable: bool,
    /// Type constraints or bounds
    pub constraints: Vec<TypeConstraint>,
}

/// Represents a type constraint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeConstraint {
    /// The constraint type (e.g., "implements", "extends")
    pub constraint_type: String,
    /// The target type
    pub target: Type,
}

/// Represents the complete AST for a program or module
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AST {
    /// Root nodes of the AST
    pub roots: Vec<Node>,
    /// Global metadata
    pub metadata: HashMap<String, String>,
    /// Source language information
    pub source_language: Option<String>,
    /// Target language information
    pub target_language: Option<String>,
}

impl Node {
    /// Create a new node with the given type and value
    pub fn new(node_type: NodeType, value: String) -> Self {
        Self {
            node_type,
            value,
            children: Vec::new(),
            metadata: HashMap::new(),
            location: None,
        }
    }
    
    /// Add a child node
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
    
    /// Add metadata
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
    
    /// Set the location
    pub fn set_location(&mut self, location: Location) {
        self.location = Some(location);
    }
    
    /// Check if this node is a leaf (no children)
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
    
    /// Get the number of children
    pub fn child_count(&self) -> usize {
        self.children.len()
    }
    
    /// Find a child node by type
    pub fn find_child_by_type(&self, node_type: &NodeType) -> Option<&Node> {
        self.children.iter().find(|child| &child.node_type == node_type)
    }
    
    /// Find all child nodes by type
    pub fn find_children_by_type(&self, node_type: &NodeType) -> Vec<&Node> {
        self.children.iter().filter(|child| &child.node_type == node_type).collect()
    }
}

impl Type {
    /// Create a new type with the given name
    pub fn new(name: String) -> Self {
        Self {
            name,
            generics: Vec::new(),
            nullable: false,
            constraints: Vec::new(),
        }
    }
    
    /// Add a generic type parameter
    pub fn add_generic(&mut self, generic: Type) {
        self.generics.push(generic);
    }
    
    /// Add a type constraint
    pub fn add_constraint(&mut self, constraint: TypeConstraint) {
        self.constraints.push(constraint);
    }
    
    /// Make this type nullable
    pub fn make_nullable(&mut self) {
        self.nullable = true;
    }
    
    /// Check if this type has generic parameters
    pub fn has_generics(&self) -> bool {
        !self.generics.is_empty()
    }
    
    /// Get the full type name including generics
    pub fn full_name(&self) -> String {
        if self.generics.is_empty() {
            self.name.clone()
        } else {
            let generic_names: Vec<String> = self.generics.iter().map(|g| g.full_name()).collect();
            format!("{}<{}>", self.name, generic_names.join(", "))
        }
    }
}

impl AST {
    /// Create a new AST
    pub fn new() -> Self {
        Self {
            roots: Vec::new(),
            metadata: HashMap::new(),
            source_language: None,
            target_language: None,
        }
    }
    
    /// Add a root node
    pub fn add_root(&mut self, root: Node) {
        self.roots.push(root);
    }
    
    /// Add metadata
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
    
    /// Set the source language
    pub fn set_source_language(&mut self, language: String) {
        self.source_language = Some(language);
    }
    
    /// Set the target language
    pub fn set_target_language(&mut self, language: String) {
        self.target_language = Some(language);
    }
    
    /// Get the total number of nodes in the AST
    pub fn node_count(&self) -> usize {
        let mut count = 0;
        for root in &self.roots {
            count += count_nodes(root);
        }
        count
    }
    
    /// Find all nodes of a specific type
    pub fn find_nodes_by_type(&self, node_type: &NodeType) -> Vec<&Node> {
        let mut nodes = Vec::new();
        for root in &self.roots {
            collect_nodes_by_type(root, node_type, &mut nodes);
        }
        nodes
    }
}

/// Count the total number of nodes in a tree
fn count_nodes(node: &Node) -> usize {
    let mut count = 1; // Count this node
    for child in &node.children {
        count += count_nodes(child);
    }
    count
}

/// Collect all nodes of a specific type
fn collect_nodes_by_type<'a>(node: &'a Node, target_type: &NodeType, nodes: &mut Vec<&'a Node>) {
    if &node.node_type == target_type {
        nodes.push(node);
    }
    for child in &node.children {
        collect_nodes_by_type(child, target_type, nodes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_node_creation() {
        let mut node = Node::new(NodeType::Literal, "42".to_string());
        assert_eq!(node.node_type, NodeType::Literal);
        assert_eq!(node.value, "42");
        assert!(node.children.is_empty());
        assert!(node.is_leaf());
    }
    
    #[test]
    fn test_node_children() {
        let mut parent = Node::new(NodeType::BinaryOp, "+".to_string());
        let left = Node::new(NodeType::Literal, "1".to_string());
        let right = Node::new(NodeType::Literal, "2".to_string());
        
        parent.add_child(left);
        parent.add_child(right);
        
        assert_eq!(parent.child_count(), 2);
        assert!(!parent.is_leaf());
    }
    
    #[test]
    fn test_type_creation() {
        let mut type_info = Type::new("List".to_string());
        let generic = Type::new("String".to_string());
        
        type_info.add_generic(generic);
        assert!(type_info.has_generics());
        assert_eq!(type_info.full_name(), "List<String>");
    }
    
    #[test]
    fn test_ast_creation() {
        let mut ast = AST::new();
        let root = Node::new(NodeType::Module, "main".to_string());
        
        ast.add_root(root);
        ast.set_source_language("python".to_string());
        ast.set_target_language("rust".to_string());
        
        assert_eq!(ast.roots.len(), 1);
        assert_eq!(ast.source_language, Some("python".to_string()));
        assert_eq!(ast.target_language, Some("rust".to_string()));
    }
}
