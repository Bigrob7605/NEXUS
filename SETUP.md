# NEXUS Project Setup Guide 🚀

This guide will help you set up the NEXUS development environment and get the project running on your machine.

## 🎯 **What You'll Get**

After following this guide, you'll have:
- ✅ NEXUS project fully built and running
- ✅ All advanced technology modules integrated
- ✅ Development environment ready for contributions
- ✅ Testing framework operational
- ✅ Documentation system synchronized

## 🔧 **Prerequisites**

### **Required Software**
- **Rust 1.70+**: [Install Rust](https://rustup.rs/)
- **Git**: [Install Git](https://git-scm.com/)
- **Python 3.8+**: [Install Python](https://python.org/)
- **Cargo**: Comes with Rust installation

### **System Requirements**
- **Operating System**: Windows 10+, macOS 10.15+, or Linux
- **Memory**: Minimum 4GB RAM (8GB+ recommended)
- **Storage**: 2GB free space
- **Network**: Internet connection for dependencies

### **Recommended Tools**
- **VS Code**: [Download VS Code](https://code.visualstudio.com/)
- **Rust Extension**: Install "rust-analyzer" extension
- **Python Extension**: Install "Python" extension
- **GitLens**: Enhanced Git integration

## 🚀 **Quick Start (5 minutes)**

### **1. Clone the Repository**
```bash
git clone https://github.com/YOUR_USERNAME/nexus.git
cd nexus
```

### **2. Build the Project**
```bash
cargo build
```

### **3. Verify Installation**
```bash
cargo run --bin nexus
```

**Expected Output:**
```
🚀 NEXUS - The Last Programming Language
AI-Native, Provably Correct, Self-Evolving

✅ Core modules loaded:
  - AST Representation System
  - Parser Infrastructure
  - Γ-AST Foundation
  - Neuromorphic Memory System
  - AI Scheduler

🎯 Advanced technology integration complete!
Ready for enhanced compression algorithms and pattern recognition.
```

**🎉 Congratulations! NEXUS is now running on your machine!**

## 🔍 **Detailed Setup Instructions**

### **Step 1: Install Rust**

#### **Windows**
```bash
# Download and run rustup-init.exe from https://rustup.rs/
# Follow the installation prompts
# Restart your terminal after installation
```

#### **macOS/Linux**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### **Verify Rust Installation**
```bash
rustc --version
cargo --version
```

**Expected Output:**
```
rustc 1.75.0 (2024-01-01)
cargo 1.75.0 (2024-01-01)
```

### **Step 2: Install Python Dependencies**

#### **Install Python Packages**
```bash
# Install required Python packages for MythGraph Ledger
pip install -r requirements.txt
```

#### **Verify Python Installation**
```bash
python --version
pip --version
```

**Expected Output:**
```
Python 3.8.10
pip 21.3.1
```

### **Step 3: Clone and Setup Project**

#### **Fork and Clone**
1. **Fork** the NEXUS repository on GitHub
2. **Clone** your fork:
```bash
git clone https://github.com/YOUR_USERNAME/nexus.git
cd nexus
```

#### **Add Upstream Remote**
```bash
git remote add upstream https://github.com/nexus-lang/nexus.git
```

### **Step 4: Build and Test**

#### **Initial Build**
```bash
# Build the project (this may take a few minutes on first run)
cargo build
```

#### **Run Tests**
```bash
# Run all tests
cargo test

# Run specific test module
cargo test --package nexus --lib parser
```

#### **Check for Issues**
```bash
# Check for compilation issues
cargo check

# Check for linting issues
cargo clippy
```

### **Step 5: Verify Integration**

#### **Run Main Binary**
```bash
cargo run --bin nexus
```

#### **Run Test Binary**
```bash
cargo run --bin run_tests
```

## 🧪 **Testing Your Setup**

### **Comprehensive Test Suite**
```bash
# Run all tests with output
cargo test -- --nocapture

# Run tests with specific pattern
cargo test test_name_pattern

# Run tests with verbose output
cargo test --verbose
```

### **Individual Module Tests**
```bash
# Test AST module
cargo test --package nexus --lib ast

# Test Parser module
cargo test --package nexus --lib parser

# Test Gamma AST module
cargo test --package nexus --lib gamma_ast
```

### **Performance Tests**
```bash
# Run performance benchmarks
cargo bench

# Check memory usage
cargo build --release
```

## 🔧 **Troubleshooting**

### **Common Issues and Solutions**

#### **1. Rust Version Issues**
```bash
# Update Rust to latest version
rustup update

# Check current version
rustup show
```

#### **2. Build Failures**
```bash
# Clean and rebuild
cargo clean
cargo build

# Check for specific errors
cargo check --verbose
```

#### **3. Module Not Found Errors**
```bash
# Ensure all modules are registered in src/main.rs
# Check that all files exist in src/ directory
# Verify module declarations are correct
```

#### **4. Python Import Errors**
```bash
# Install Python dependencies
pip install -r requirements.txt

# Check Python path
python -c "import sys; print(sys.path)"
```

#### **5. Git Issues**
```bash
# Check remote configuration
git remote -v

# Reset to clean state
git reset --hard HEAD
git clean -fd
```

### **Getting Help**

#### **Check Project Status**
```bash
# Verify project structure
ls -la src/
ls -la tests/
ls -la docs/
```

#### **Check Dependencies**
```bash
# Verify Rust dependencies
cargo tree

# Check Python packages
pip list
```

#### **Review Logs**
```bash
# Check recent sync logs
cat logs/recent-sync.md

# Check project status
cat docs/PROJECT_STATUS_SUMMARY.md
```

## 🎯 **Next Steps After Setup**

### **1. Explore the Codebase**
```bash
# Browse source code
code src/

# Read documentation
code README.md
code CORE_DEVELOPMENT_ROADMAP.md
```

### **2. Run Development Tools**
```bash
# Watch for changes and auto-rebuild
cargo install cargo-watch
cargo watch -x check

# Run linter
cargo clippy

# Format code
cargo fmt
```

### **3. Start Contributing**
```bash
# Create feature branch
git checkout -b feature/your-feature-name

# Make changes and test
cargo test

# Commit and push
git add .
git commit -m "feat(scope): description"
git push origin feature/your-feature-name
```

## 📊 **Verification Checklist**

Before you start contributing, ensure:

- [ ] **Rust Installation**: `rustc --version` shows 1.70+
- [ ] **Python Installation**: `python --version` shows 3.8+
- [ ] **Project Builds**: `cargo build` completes successfully
- [ ] **Tests Pass**: `cargo test` shows all tests passing
- [ ] **Main Binary Runs**: `cargo run --bin nexus` displays welcome message
- [ ] **Dependencies Installed**: All Python packages from requirements.txt installed
- [ ] **Git Setup**: Repository cloned with upstream remote configured
- [ ] **Documentation**: README.md and other docs are readable

## 🚀 **Advanced Setup Options**

### **Development Environment**
```bash
# Install additional development tools
cargo install cargo-watch
cargo install cargo-audit
cargo install cargo-tarpaulin

# Setup VS Code extensions
code --install-extension rust-lang.rust-analyzer
code --install-extension ms-python.python
```

### **Performance Profiling**
```bash
# Install profiling tools
cargo install flamegraph
cargo install cargo-profiler

# Run with profiling
cargo profiler callgrind -- cargo run --bin nexus
```

### **Continuous Integration**
```bash
# Setup pre-commit hooks
cargo install cargo-husky
cargo husky install

# Configure CI checks
cargo install cargo-make
```

## 🎉 **You're Ready!**

**Congratulations!** You now have a fully operational NEXUS development environment.

### **What You Can Do Next:**
1. **Explore the codebase** and understand the architecture
2. **Run tests** to verify everything is working
3. **Read documentation** to understand the project goals
4. **Pick an issue** to work on from the roadmap
5. **Start contributing** to the NEXUS revolution!

### **Need Help?**
- **Check the documentation** in the `docs/` folder
- **Review the roadmap** in `CORE_DEVELOPMENT_ROADMAP.md`
- **Check recent logs** in `logs/recent-sync.md`
- **Create an issue** on GitHub if you encounter problems

---

**Welcome to NEXUS: The Last Programming Language!** 🌟

**The future of programming is provable, AI-native, self-evolving, and pain-free.**
**The future is NEXUS.**
