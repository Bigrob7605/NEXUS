//! Parser Module
//! 
//! This module provides the core parsing infrastructure for converting source code
//! into our universal AST format. It includes lexing, parsing, and error handling.

use crate::ast::{AST, Node, NodeType, Location};
use std::collections::HashMap;
use std::fmt;

/// Represents a token in the source code
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The type of token
    pub token_type: TokenType,
    /// The actual text value
    pub value: String,
    /// Source location
    pub location: Location,
}

/// Represents different types of tokens
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Integer,
    Float,
    String,
    Boolean,
    Null,
    
    // Identifiers
    Identifier,
    
    // Keywords
    Function,
    If,
    Else,
    While,
    For,
    Return,
    Let,
    Const,
    Class,
    Import,
    Export,
    
    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Assign,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,
    Colon,
    
    // Special
    Comment,
    Whitespace,
    Newline,
    EOF,
}

/// Represents a parsing error
#[derive(Debug, Clone)]
pub struct ParseError {
    /// Error message
    pub message: String,
    /// Location where the error occurred
    pub location: Location,
    /// Error severity
    pub severity: ErrorSeverity,
}

/// Error severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    Warning,
    Error,
    Fatal,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} at line {}:{} - {}", 
            self.severity, 
            self.location.line, 
            self.location.column, 
            self.message
        )
    }
}

impl std::error::Error for ParseError {}

/// Result type for parsing operations
pub type ParseResult<T> = Result<T, ParseError>;

/// Trait for language-specific parsers
pub trait Parser {
    /// Parse source code into an AST
    fn parse(&mut self, source: &str) -> ParseResult<AST>;
    
    /// Get the language this parser supports
    fn language(&self) -> &str;
    
    /// Check if the parser can handle the given source
    fn can_parse(&self, source: &str) -> bool;
}

/// Lexer trait for token generation
pub trait Lexer {
    /// Tokenize source code into a stream of tokens
    fn tokenize(&mut self, source: &str) -> ParseResult<Vec<Token>>;
    
    /// Get the next token from the stream
    fn next_token(&mut self) -> ParseResult<Option<Token>>;
    
    /// Peek at the next token without consuming it
    fn peek_token(&self) -> ParseResult<Option<&Token>>;
}

/// Basic lexer implementation
pub struct BasicLexer {
    /// Source code to tokenize
    source: String,
    /// Current position in the source
    position: usize,
    /// Current line number
    line: usize,
    /// Current column number
    column: usize,
    /// Tokens that have been generated
    tokens: Vec<Token>,
    /// Current token being built
    current_token: Option<Token>,
}

impl BasicLexer {
    /// Create a new lexer
    pub fn new() -> Self {
        Self {
            source: String::new(),
            position: 0,
            line: 1,
            column: 1,
            tokens: Vec::new(),
            current_token: None,
        }
    }
    
    /// Set the source code to tokenize
    pub fn set_source(&mut self, source: String) {
        self.source = source;
        self.position = 0;
        self.line = 1;
        self.column = 1;
        self.tokens.clear();
        self.current_token = None;
    }
    
    /// Check if we've reached the end of the source
    fn is_eof(&self) -> bool {
        self.position >= self.source.len()
    }
    
    /// Get the current character
    fn current_char(&self) -> Option<char> {
        self.source.chars().nth(self.position)
    }
    
    /// Get the next character without advancing
    fn peek_char(&self) -> Option<char> {
        self.source.chars().nth(self.position + 1)
    }
    
    /// Advance to the next character
    fn advance(&mut self) {
        if let Some(ch) = self.current_char() {
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        self.position += 1;
    }
    
    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() && ch != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    /// Read an identifier or keyword
    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        let start_column = self.column;
        
        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        identifier
    }
    
    /// Read a number (integer or float)
    fn read_number(&mut self) -> String {
        let mut number = String::new();
        let mut has_decimal = false;
        
        while let Some(ch) = self.current_char() {
            if ch.is_digit(10) {
                number.push(ch);
                self.advance();
            } else if ch == '.' && !has_decimal {
                number.push(ch);
                has_decimal = true;
                self.advance();
            } else {
                break;
            }
        }
        
        number
    }
    
    /// Read a string literal
    fn read_string(&mut self) -> ParseResult<String> {
        let mut string = String::new();
        let quote_char = self.current_char().ok_or_else(|| ParseError {
            message: "Expected quote character".to_string(),
            location: Location { line: self.line, column: self.column, file: None },
            severity: ErrorSeverity::Error,
        })?;
        
        self.advance(); // Skip opening quote
        
        while let Some(ch) = self.current_char() {
            if ch == quote_char {
                self.advance(); // Skip closing quote
                break;
            } else if ch == '\\' {
                // Handle escape sequences
                self.advance();
                if let Some(escaped) = self.current_char() {
                    match escaped {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        'r' => string.push('\r'),
                        '\\' => string.push('\\'),
                        '"' => string.push('"'),
                        '\'' => string.push('\''),
                        _ => return Err(ParseError {
                            message: format!("Invalid escape sequence \\{}", escaped),
                            location: Location { line: self.line, column: self.column, file: None },
                            severity: ErrorSeverity::Error,
                        }),
                    }
                    self.advance();
                }
            } else {
                string.push(ch);
                self.advance();
            }
        }
        
        Ok(string)
    }
    
    /// Create a token at the current position
    fn create_token(&self, token_type: TokenType, value: String) -> Token {
        Token {
            token_type,
            value,
            location: Location {
                line: self.line,
                column: self.column,
                file: None,
            },
        }
    }
}

impl Lexer for BasicLexer {
    fn tokenize(&mut self, source: &str) -> ParseResult<Vec<Token>> {
        self.set_source(source.to_string());
        self.tokens.clear();
        
        while !self.is_eof() {
            self.skip_whitespace();
            
            if self.is_eof() {
                break;
            }
            
            if let Some(ch) = self.current_char() {
                let token = match ch {
                    // Identifiers and keywords
                    'a'..='z' | 'A'..='Z' | '_' => {
                        let identifier = self.read_identifier();
                        let token_type = match identifier.as_str() {
                            "fn" | "function" => TokenType::Function,
                            "if" => TokenType::If,
                            "else" => TokenType::Else,
                            "while" => TokenType::While,
                            "for" => TokenType::For,
                            "return" => TokenType::Return,
                            "let" => TokenType::Let,
                            "const" => TokenType::Const,
                            "class" => TokenType::Class,
                            "import" => TokenType::Import,
                            "export" => TokenType::Export,
                            "true" | "false" => TokenType::Boolean,
                            "null" => TokenType::Null,
                            _ => TokenType::Identifier,
                        };
                        self.create_token(token_type, identifier)
                    }
                    
                    // Numbers
                    '0'..='9' => {
                        let number = self.read_number();
                        let token_type = if number.contains('.') {
                            TokenType::Float
                        } else {
                            TokenType::Integer
                        };
                        self.create_token(token_type, number)
                    }
                    
                    // Strings
                    '"' | '\'' => {
                        let string = self.read_string()?;
                        self.create_token(TokenType::String, string)
                    }
                    
                    // Operators and delimiters
                    '+' => { self.advance(); self.create_token(TokenType::Plus, "+".to_string()) }
                    '-' => { self.advance(); self.create_token(TokenType::Minus, "-".to_string()) }
                    '*' => { self.advance(); self.create_token(TokenType::Multiply, "*".to_string()) }
                    '/' => { self.advance(); self.create_token(TokenType::Divide, "/".to_string()) }
                    '%' => { self.advance(); self.create_token(TokenType::Modulo, "%".to_string()) }
                    '=' => {
                        self.advance();
                        if let Some('=') = self.peek_char() {
                            self.advance();
                            self.create_token(TokenType::Equal, "==".to_string())
                        } else {
                            self.create_token(TokenType::Assign, "=".to_string())
                        }
                    }
                    '<' => {
                        self.advance();
                        if let Some('=') = self.peek_char() {
                            self.advance();
                            self.create_token(TokenType::LessEqual, "<=".to_string())
                        } else {
                            self.create_token(TokenType::LessThan, "<".to_string())
                        }
                    }
                    '>' => {
                        self.advance();
                        if let Some('=') = self.peek_char() {
                            self.advance();
                            self.create_token(TokenType::GreaterEqual, ">=".to_string())
                        } else {
                            self.create_token(TokenType::GreaterThan, ">".to_string())
                        }
                    }
                    '!' => {
                        self.advance();
                        if let Some('=') = self.peek_char() {
                            self.advance();
                            self.create_token(TokenType::NotEqual, "!=".to_string())
                        } else {
                            self.create_token(TokenType::Not, "!".to_string())
                        }
                    }
                    '&' => {
                        self.advance();
                        if let Some('&') = self.peek_char() {
                            self.advance();
                            self.create_token(TokenType::And, "&&".to_string())
                        } else {
                            return Err(ParseError {
                                message: "Expected '&' after '&'".to_string(),
                                location: Location { line: self.line, column: self.column, file: None },
                                severity: ErrorSeverity::Error,
                            });
                        }
                    }
                    '|' => {
                        self.advance();
                        if let Some('|') = self.peek_char() {
                            self.advance();
                            self.create_token(TokenType::Or, "||".to_string())
                        } else {
                            return Err(ParseError {
                                message: "Expected '|' after '|'".to_string(),
                                location: Location { line: self.line, column: self.column, file: None },
                                severity: ErrorSeverity::Error,
                            });
                        }
                    }
                    '(' => { self.advance(); self.create_token(TokenType::LeftParen, "(".to_string()) }
                    ')' => { self.advance(); self.create_token(TokenType::RightParen, ")".to_string()) }
                    '{' => { self.advance(); self.create_token(TokenType::LeftBrace, "{".to_string()) }
                    '}' => { self.advance(); self.create_token(TokenType::RightBrace, "}".to_string()) }
                    '[' => { self.advance(); self.create_token(TokenType::LeftBracket, "[".to_string()) }
                    ']' => { self.advance(); self.create_token(TokenType::RightBracket, "]".to_string()) }
                    ';' => { self.advance(); self.create_token(TokenType::Semicolon, ";".to_string()) }
                    ',' => { self.advance(); self.create_token(TokenType::Comma, ",".to_string()) }
                    '.' => { self.advance(); self.create_token(TokenType::Dot, ".".to_string()) }
                    ':' => { self.advance(); self.create_token(TokenType::Colon, ":".to_string()) }
                    
                    // Comments
                    '/' => {
                        if let Some('/') = self.peek_char() {
                            // Single line comment
                            self.advance(); // Skip first '/'
                            self.advance(); // Skip second '/'
                            let mut comment = String::new();
                            while let Some(ch) = self.current_char() {
                                if ch == '\n' {
                                    break;
                                }
                                comment.push(ch);
                                self.advance();
                            }
                            self.create_token(TokenType::Comment, comment)
                        } else {
                            self.create_token(TokenType::Divide, "/".to_string())
                        }
                    }
                    
                    // Newlines
                    '\n' => {
                        self.advance();
                        self.create_token(TokenType::Newline, "\n".to_string())
                    }
                    
                    // Unknown character
                    _ => {
                        return Err(ParseError {
                            message: format!("Unexpected character: {}", ch),
                            location: Location { line: self.line, column: self.column, file: None },
                            severity: ErrorSeverity::Error,
                        });
                    }
                };
                
                self.tokens.push(token);
            }
        }
        
        // Add EOF token
        self.tokens.push(self.create_token(TokenType::EOF, "".to_string()));
        
        Ok(self.tokens.clone())
    }
    
    fn next_token(&mut self) -> ParseResult<Option<Token>> {
        if self.tokens.is_empty() {
            Ok(None)
        } else {
            Ok(Some(self.tokens.remove(0)))
        }
    }
    
    fn peek_token(&self) -> ParseResult<Option<&Token>> {
        Ok(self.tokens.first())
    }
}

/// Basic parser implementation
pub struct BasicParser {
    /// The lexer for token generation
    lexer: BasicLexer,
    /// Current tokens being parsed
    tokens: Vec<Token>,
    /// Current position in the token stream
    position: usize,
}

impl BasicParser {
    /// Create a new parser
    pub fn new() -> Self {
        Self {
            lexer: BasicLexer::new(),
            tokens: Vec::new(),
            position: 0,
        }
    }
    
    /// Get the current token
    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }
    
    /// Get the next token without advancing
    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.position + 1)
    }
    
    /// Advance to the next token
    fn advance(&mut self) {
        self.position += 1;
    }
    
    /// Check if we've reached the end of tokens
    fn is_eof(&self) -> bool {
        self.position >= self.tokens.len()
    }
    
    /// Expect a specific token type
    fn expect(&mut self, expected_type: TokenType) -> ParseResult<Token> {
        // Check bounds first
        if self.position >= self.tokens.len() {
            return Err(ParseError {
                message: format!("Expected {:?}, but reached end of input", expected_type),
                location: Location { line: 1, column: 1, file: None },
                severity: ErrorSeverity::Fatal,
            });
        }
        
        // Get token data first, then advance
        let token_data = {
            let token = &self.tokens[self.position];
            (token.token_type.clone(), token.value.clone(), token.location.clone())
        };
        
        let (token_type, value, location) = token_data;
        
        if token_type == expected_type {
            // Advance position
            self.position += 1;
            
            // Return a new token with the data we collected
            Ok(Token {
                token_type,
                value,
                location,
            })
        } else {
            Err(ParseError {
                message: format!("Expected {:?}, got {:?}", expected_type, token_type),
                location,
                severity: ErrorSeverity::Error,
            })
        }
    }
    
    /// Parse a simple expression
    fn parse_expression(&mut self) -> ParseResult<Node> {
        // Check bounds first
        if self.position >= self.tokens.len() {
            return Err(ParseError {
                message: "Unexpected end of input in expression".to_string(),
                location: Location { line: 1, column: 1, file: None },
                severity: ErrorSeverity::Fatal,
            });
        }
        
        // Use the existing advance method to avoid borrow checker issues
        let token = self.current_token().unwrap();
        let token_type = token.token_type.clone();
        let value = token.value.clone();
        let location = token.location.clone();
        
        // Advance using the existing method
        self.advance();
        
        let node = match token_type {
            TokenType::Integer | TokenType::Float => {
                Node::new(NodeType::Literal, value)
            }
            TokenType::String => {
                Node::new(NodeType::Literal, value)
            }
            TokenType::Boolean => {
                Node::new(NodeType::Literal, value)
            }
            TokenType::Identifier => {
                Node::new(NodeType::Variable, value)
            }
            _ => {
                return Err(ParseError {
                    message: format!("Unexpected token in expression: {:?}", token_type),
                    location,
                    severity: ErrorSeverity::Error,
                });
            }
        };
        
        Ok(node)
    }
}

impl Parser for BasicParser {
    fn parse(&mut self, source: &str) -> ParseResult<AST> {
        // First, tokenize the source
        self.tokens = self.lexer.tokenize(source)?;
        self.position = 0;
        
        // Create a new AST
        let mut ast = AST::new();
        ast.set_source_language("unknown".to_string());
        
        // For now, just parse the first expression we find
        if !self.is_eof() {
            let expression = self.parse_expression()?;
            ast.add_root(expression);
        }
        
        Ok(ast)
    }
    
    fn language(&self) -> &str {
        "basic"
    }
    
    fn can_parse(&self, _source: &str) -> bool {
        // Basic parser can handle simple expressions
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lexer_creation() {
        let lexer = BasicLexer::new();
        assert_eq!(lexer.position, 0);
        assert_eq!(lexer.line, 1);
        assert_eq!(lexer.column, 1);
    }
    
    #[test]
    fn test_basic_tokenization() {
        let mut lexer = BasicLexer::new();
        let tokens = lexer.tokenize("42 + 3").unwrap();
        
        assert_eq!(tokens.len(), 4); // 42, +, 3, EOF
        assert_eq!(tokens[0].token_type, TokenType::Integer);
        assert_eq!(tokens[0].value, "42");
        assert_eq!(tokens[1].token_type, TokenType::Plus);
        assert_eq!(tokens[1].value, "+");
        assert_eq!(tokens[2].token_type, TokenType::Integer);
        assert_eq!(tokens[2].value, "3");
    }
    
    #[test]
    fn test_parser_creation() {
        let parser = BasicParser::new();
        assert_eq!(parser.language(), "basic");
        assert!(parser.can_parse("simple expression"));
    }
    
    #[test]
    fn test_basic_parsing() {
        let mut parser = BasicParser::new();
        let ast = parser.parse("42").unwrap();
        
        assert_eq!(ast.roots.len(), 1);
        assert_eq!(ast.roots[0].node_type, NodeType::Literal);
        assert_eq!(ast.roots[0].value, "42");
    }
}
