# Contributing to NEXUS 🚀

Thank you for your interest in contributing to NEXUS: The Last Programming Language! This guide will help you get started with contributing to our AI-native, provably correct, self-evolving programming language.

## 🎯 **Our Mission**

NEXUS aims to eliminate ALL programming pain points and deliver ALL developer dreams through:
- **AI-Native Design**: Built for AI-to-AI communication efficiency
- **Provable Correctness**: Every feature includes formal verification
- **Self-Evolution**: Language that gets better over time
- **Universal Bridges**: Seamless integration with any existing language

## 🚫 **Zero Tolerance for Drift**

**IMPORTANT**: NEXUS follows the Kai Master Protocol V5 with strict anti-drift rules:
- ❌ No impossible claims (e.g., "1.56 BILLION x compression")
- ❌ No physics violations
- ❌ No "magic" terminology without implementation
- ❌ No simulations or exaggerations

**All contributions must be reality-based and backed by actual code.**

## 🔧 **Prerequisites**

### **Required Tools**
- **Rust 1.70+**: For the core compiler and runtime
- **Git**: For version control
- **Python 3.8+**: For MythGraph Ledger and testing framework
- **Cargo**: Rust package manager

### **Recommended Tools**
- **VS Code** with Rust extension
- **rust-analyzer**: For better Rust development experience
- **cargo-watch**: For automatic rebuilding during development

## 🚀 **Getting Started**

### **1. Fork and Clone**
```bash
# Fork the repository on GitHub
# Then clone your fork
git clone https://github.com/YOUR_USERNAME/nexus.git
cd nexus

# Add upstream remote
git remote add upstream https://github.com/nexus-lang/nexus.git
```

### **2. Build the Project**
```bash
# Build the project
cargo build

# Run tests
cargo test

# Check for issues
cargo check
```

### **3. Verify Integration**
```bash
# Run the main binary
cargo run --bin nexus

# Should display:
# 🚀 NEXUS - The Last Programming Language
# AI-Native, Provably Correct, Self-Evolving
# ✅ Core modules loaded:
#   - AST Representation System
#   - Parser Infrastructure
#   - Γ-AST Foundation
#   - Neuromorphic Memory System
#   - AI Scheduler
```

## 📋 **Contribution Guidelines**

### **Code Quality Standards**
- **Rust Standards**: Follow Rust best practices and idioms
- **Error Handling**: Comprehensive error handling with proper error types
- **Documentation**: All public APIs must be documented
- **Testing**: Unit tests for all new functionality
- **Performance**: Respect information theory constraints

### **Commit Message Format**
```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```
feat(gamma-ast): add advanced pattern recognition
fix(parser): resolve borrow checker issue in expect method
docs(readme): update with advanced technology integration
```

### **Pull Request Process**
1. **Create Feature Branch**: `git checkout -b feature/your-feature-name`
2. **Make Changes**: Implement your feature with tests
3. **Run Tests**: Ensure all tests pass
4. **Update Documentation**: Update relevant docs and README
5. **Submit PR**: Create pull request with clear description
6. **Code Review**: Address feedback from maintainers
7. **Merge**: Once approved and CI passes

## 🏗️ **Project Structure**

```
nexus/
├── src/                    # Rust source code
│   ├── ast/              # AST representation system
│   ├── parser/           # Parser infrastructure
│   ├── gamma_ast/        # Γ-AST foundation
│   ├── neuromem.rs       # Neuromorphic memory system
│   └── ai_scheduler.rs   # AI scheduler
├── tests/                 # Test files
├── docs/                  # Documentation
├── logs/                  # Project logs
└── Cargo.toml            # Rust dependencies
```

## 🧪 **Testing Strategy**

### **Test Requirements**
- **Unit Tests**: Test each component individually
- **Integration Tests**: Test component interactions
- **Performance Tests**: Validate performance requirements
- **Regression Tests**: Ensure no functionality is lost

### **Running Tests**
```bash
# Run all tests
cargo test

# Run specific test module
cargo test --package nexus --lib parser

# Run tests with output
cargo test -- --nocapture

# Run tests with specific pattern
cargo test test_name_pattern
```

### **Test Coverage**
- **Target**: >95% test coverage
- **All public APIs**: Must have tests
- **Edge cases**: Must be covered
- **Error conditions**: Must be tested

## 🔍 **Code Review Process**

### **What We Look For**
- **Correctness**: Does the code do what it claims?
- **Performance**: Is it efficient and within constraints?
- **Security**: Are there any security vulnerabilities?
- **Maintainability**: Is the code readable and maintainable?
- **Documentation**: Is everything properly documented?

### **Review Checklist**
- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Documentation is updated
- [ ] No performance regressions
- [ ] Follows project coding standards
- [ ] Addresses the issue/feature request

## 🚨 **Common Issues to Avoid**

### **Borrow Checker Issues**
- **Problem**: Common in Rust development
- **Solution**: Use proper ownership patterns, clone when necessary
- **Example**: We fixed this in the parser's `expect` method

### **Unused Code Warnings**
- **Problem**: Rust warns about unused code
- **Solution**: Remove unused code or mark with `#[allow(dead_code)]`
- **Note**: We currently have some warnings that will be addressed

### **Integration Issues**
- **Problem**: New modules not properly integrated
- **Solution**: Ensure all modules are registered in `src/main.rs`
- **Example**: We successfully integrated `neuromem` and `ai_scheduler`

## 📚 **Documentation Standards**

### **Code Documentation**
```rust
/// Brief description of what this function does
/// 
/// # Arguments
/// * `param1` - Description of first parameter
/// * `param2` - Description of second parameter
/// 
/// # Returns
/// Description of return value
/// 
/// # Examples
/// ```
/// let result = function_name(value1, value2);
/// assert_eq!(result, expected_value);
/// ```
pub fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // Implementation
}
```

### **Documentation Updates**
- **README.md**: Update for major changes
- **API docs**: Update for all public API changes
- **Roadmap**: Update progress and next steps
- **Logs**: Document all significant changes

## 🎯 **Current Development Priorities**

### **Phase 2: Enhanced Compression System**
1. **Implement Advanced Compression Algorithms**
   - Leverage neuromorphic pattern recognition
   - Apply temporal and spatial analysis
   - Integrate intelligent resource management

2. **Validate Enhanced Capabilities**
   - Test with comprehensive framework
   - Verify cryptographic signatures
   - Measure performance improvements

### **Areas Needing Contributors**
- **Compression Algorithms**: Advanced pattern recognition implementation
- **Language Bridges**: Python, Rust, JavaScript interoperability
- **Testing Framework**: Enhanced validation and edge case testing
- **Performance Optimization**: GPU utilization and memory management

## 🤝 **Getting Help**

### **Communication Channels**
- **GitHub Issues**: For bug reports and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Pull Requests**: For code contributions and improvements

### **Before Asking for Help**
1. **Check Documentation**: Read README and relevant docs
2. **Search Issues**: Look for similar problems
3. **Reproduce Issue**: Ensure you can reproduce the problem
4. **Provide Context**: Include relevant code and error messages

## 🏆 **Recognition**

### **Contributor Levels**
- **Contributor**: First successful contribution
- **Regular Contributor**: Multiple contributions over time
- **Core Contributor**: Significant contributions and project knowledge
- **Maintainer**: Trusted with merge permissions

### **Hall of Fame**
Contributors will be recognized in:
- **README.md**: List of major contributors
- **CHANGELOG.md**: All contributions documented
- **Project website**: Featured contributor profiles

## 📄 **License**

By contributing to NEXUS, you agree that your contributions will be licensed under the **NPL (Nexus Public License)**:
- Any derivative must be provably memory-safe
- Any derivative must publish its Γ-AST hash chain
- No derivative may emit undefined behavior

## 🚀 **Ready to Contribute?**

1. **Fork the repository**
2. **Set up your development environment**
3. **Pick an issue or feature to work on**
4. **Follow the contribution guidelines**
5. **Submit your pull request**

**Welcome to the NEXUS revolution!** 🌟

---

**Remember**: Every contribution must be provably correct, reality-based, and move us closer to eliminating ALL programming pain while delivering ALL developer dreams.

**The future of programming is provable, AI-native, self-evolving, and pain-free.**
**The future is NEXUS.**
