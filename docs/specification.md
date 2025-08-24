# NEXUS Language Specification v0.1.0

> *"The specification that writes itself"*

## Table of Contents

1. [Overview](#overview)
2. [Syntax Layers](#syntax-layers)
3. [Type System](#type-system)
4. [Memory Model](#memory-model)
5. [Capability System](#capability-system)
6. [Standard Library](#standard-library)
7. [Compilation Model](#compilation-model)

## Overview

NEXUS is a multi-layer programming language designed for AI-native communication, provable correctness, and capability-based security. The language operates across four distinct layers, each serving different purposes while maintaining semantic consistency.

### Core Principles

- **AI-First**: Designed for intelligence-to-intelligence communication
- **Provably Correct**: Every program generates mathematical proofs
- **Capability-Based**: Security through explicit permissions
- **Zero Undefined Behavior**: All execution paths are mathematically defined
- **Self-Evolving**: Language can modify its own syntax and semantics

## Syntax Layers

### Layer 0: Γ-AST (Gamma Abstract Syntax Tree)

The canonical, compressed semantic representation. This layer is never directly written by humans but serves as the foundation for all other layers.

**Characteristics:**
- 256-bit hashed for integrity
- Lossless semantic compression
- AI-native representation
- Machine-optimized for transmission

### Layer 1: Wireform

Binary tokenstream for inter-process and inter-AI communication.

**Characteristics:**
- Self-describing format
- Self-checksumming
- Constant-time decode
- Optimized for network transmission

### Layer 2: Surface

Human-readable syntax layer with UTF-8 encoding and whitespace sensitivity.

**Characteristics:**
- One screenful rule (≤80 columns)
- Collapse macros to Layer 1
- Familiar syntax for human developers
- Automatic formatting and linting

### Layer 3: Narrative

Literate programming layer with markdown integration.

**Characteristics:**
- Documentation as code
- Tangles to Surface syntax
- Weaves to PDF/HTML
- Legal and academic proof generation

## Type System: Ω-Logic

NEXUS uses a dependent type system called Ω-Logic that combines:

- **Dependent Types**: Types that depend on values
- **Linear Types**: Resource management through types
- **Effect Types**: Side effect tracking
- **Capability Types**: Security through explicit permissions

### Basic Types

```nexus
// Primitive types
let x: u64 = 42;
let y: f64 = 3.14159;
let z: bool = true;
let s: String = "Hello, NEXUS!";

// Nullable types (explicit opt-in)
let maybe: String? = null;
let definitely: String = "not null";

// Array types
let numbers: [u64; 10] = [0; 10];
let dynamic: Vec<u64> = vec![1, 2, 3, 4, 5];

// Tuple types
let pair: (u64, String) = (42, "answer");
```

### Dependent Types

```nexus
// Types that depend on values
struct Vector<const N: usize> {
    data: [f64; N]
}

// Length is encoded in the type
let v3: Vector<3> = Vector { data: [1.0, 2.0, 3.0] };
let v4: Vector<4> = Vector { data: [1.0, 2.0, 3.0, 4.0] };

// Compile-time length checking
fn dot_product<const N: usize>(a: Vector<N>, b: Vector<N>) -> f64 {
    // N is guaranteed to be the same for both vectors
    // This eliminates bounds checking at runtime
}
```

### Linear Types

```nexus
// Linear types ensure resources are used exactly once
struct File {
    handle: FileHandle
}

impl File {
    fn read(&mut self) -> Result<Vec<u8>, Error> {
        // Implementation
    }
    
    fn close(self) {
        // File is consumed here, handle is automatically closed
    }
}

// Usage pattern
let file = File::open("data.txt")?;
let data = file.read()?;
file.close(); // File is consumed
// file.read()?; // Compile error: file already consumed
```

### Effect Types

```nexus
// Effects are tracked in the type system
fn pure_function(x: u64) -> u64 {
    x * 2 // No side effects
}

fn side_effect_function(x: u64) -> u64 {
    log("Processing {}", x); // Side effect: logging
    x * 2
}

// Effect types are inferred
let pure: u64 = pure_function(21); // Type: u64
let effect: u64 = side_effect_function(21); // Type: u64 with Log effect
```

## Memory Model

NEXUS uses a hybrid memory model combining Rust-style ownership with capability-based security.

### Ownership Rules

1. **Single Owner**: Each value has exactly one owner
2. **Move Semantics**: Assignment transfers ownership
3. **Borrowing**: Temporary access without ownership transfer
4. **Lifetime Bounds**: References cannot outlive their referents

### Capability-Based Memory

```nexus
// Capabilities define what operations are allowed
struct MemoryRegion {
    data: Vec<u8>,
    read_cap: Capability<Read>,
    write_cap: Capability<Write>,
    dealloc_cap: Capability<Deallocate>
}

impl MemoryRegion {
    fn read(&self, _cap: &Capability<Read>) -> &[u8] {
        &self.data
    }
    
    fn write(&mut self, _cap: &Capability<Write>, data: &[u8]) {
        self.data.copy_from_slice(data);
    }
    
    fn deallocate(self, _cap: Capability<Deallocate>) {
        // Memory is freed
    }
}
```

### Zero-Copy Communication

```nexus
// AI-to-AI message passing without copying data
struct Message<T> {
    data: T,
    send_cap: Capability<Send>,
    receive_cap: Capability<Receive>
}

impl<T> Message<T> {
    fn send(self, _cap: Capability<Send>) -> Message<T> {
        // Transfer ownership without copying
        self
    }
    
    fn receive(self, _cap: Capability<Receive>) -> T {
        // Extract data
        self.data
    }
}
```

## Capability System

The capability system provides security through explicit permissions rather than implicit trust.

### Capability Types

```nexus
// Basic capability types
enum Capability<T> {
    Read,
    Write,
    Execute,
    Send,
    Receive,
    Spawn,
    Reflect,
    // ... more capabilities
}

// Capability tokens are cryptographically signed
struct CapabilityToken {
    permissions: Vec<Capability>,
    signature: Ed25519Signature,
    nonce: u64,
    expires: Timestamp
}
```

### Capability Checking

```nexus
// Functions require explicit capabilities
fn read_file(
    path: &str, 
    _cap: &Capability<Read>
) -> Result<String, Error> {
    std::fs::read_to_string(path)
}

fn write_file(
    path: &str, 
    content: &str, 
    _cap: &Capability<Write>
) -> Result<(), Error> {
    std::fs::write(path, content)
}

// Usage requires appropriate capabilities
let read_cap = acquire_capability::<Read>("file_system")?;
let content = read_file("data.txt", &read_cap)?;

let write_cap = acquire_capability::<Write>("file_system")?;
write_file("output.txt", &content, &write_cap)?;
```

## Standard Library

### Core Modules

```nexus
// Memory management
mod memory {
    pub struct Arena;
    pub struct Pool;
    pub struct GarbageCollector;
}

// Concurrency
mod concurrency {
    pub struct Task;
    pub struct Channel<T>;
    pub struct Mutex<T>;
    pub struct RwLock<T>;
}

// AI primitives
mod ai {
    pub struct Tensor<T, const DIMS: usize>;
    pub struct NeuralNetwork;
    pub struct Attention;
    pub struct Transformer;
}

// Cryptography
mod crypto {
    pub struct Hash;
    pub struct Signature;
    pub struct Encryption;
    pub struct CapabilityToken;
}
```

### AI-Native Features

```nexus
// Native tensor operations
let tensor: Tensor<f32, 2> = Tensor::zeros([64, 128]);
let result = tensor
    .matmul(&weights)
    .add(&bias)
    .relu()
    .dropout(0.1);

// Neural network primitives
struct MLP {
    layers: Vec<Linear>,
    activation: Activation
}

impl MLP {
    fn forward(&self, input: Tensor<f32, 2>) -> Tensor<f32, 2> {
        self.layers.iter()
            .fold(input, |acc, layer| {
                layer.forward(acc).apply(&self.activation)
            })
    }
}
```

## Compilation Model

NEXUS uses a multi-stage compilation process:

1. **Surface → Wireform**: Parse human-readable syntax
2. **Wireform → Γ-AST**: Generate canonical semantic representation
3. **Γ-AST → Proofs**: Generate mathematical proofs
4. **Γ-AST → Target**: Compile to target platform (WASM, native, etc.)

### Compilation Flags

```bash
# Basic compilation
nexus build source.nex

# With proofs
nexus build --prove source.nex

# To specific target
nexus build --target wasm source.nex
nexus build --target native source.nex

# With optimizations
nexus build --release --prove source.nex
```

### Proof Generation

Every NEXUS program generates mathematical proofs that can be verified by:

- **Coq**: Interactive theorem prover
- **Lean**: Functional programming theorem prover
- **SMT Solvers**: Automated theorem provers

```nexus
// Proof annotations
#[proof]
fn safe_array_access<T>(arr: &[T], index: usize) -> Option<&T> {
    if index < arr.len() {
        Some(&arr[index])
    } else {
        None
    }
}

// Compiler generates proof that this function never panics
// and always returns a valid reference when Some is returned
```

## Future Extensions

### Self-Modifying Syntax

```nexus
// Future: Syntax that can evolve
syntax_rule! {
    // Define new syntax patterns
    pattern: "for each $item in $collection do $body"
    expands_to: "$collection.iter().for_each(|$item| $body)"
}

// Usage
for each item in [1, 2, 3, 4, 5] do {
    log("Item: {}", item);
}
```

### AI-Assisted Compilation

```nexus
// Future: AI can suggest optimizations
#[ai_optimize]
fn expensive_operation(data: &[f64]) -> f64 {
    data.iter().map(|x| x.powi(2)).sum()
}

// AI compiler might suggest:
// - SIMD vectorization
// - Parallel processing
// - Memory layout optimizations
```

---

*This specification is a living document that evolves with the language itself.*
