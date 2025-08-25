//! Tests module for integration tests
//! 
//! This module provides test types and utilities for integration tests

/// Test result with timing information
#[derive(Debug, Clone)]
pub struct TestResult {
    pub test_name: String,
    pub passed: bool,
    pub duration_ms: u64,
    pub error_message: Option<String>,
}

/// Test suite runner
pub struct TestSuite {
    pub name: String,
    pub tests: Vec<Box<dyn Fn() -> TestResult>>,
}

impl TestSuite {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tests: Vec::new(),
        }
    }
    
    pub fn add_test<F>(&mut self, test: F)
    where
        F: Fn() -> TestResult + 'static,
    {
        self.tests.push(Box::new(test));
    }
    
    pub fn run_all(&self) -> Vec<TestResult> {
        println!("🧪 Running test suite: {}", self.name);
        println!("{}", "=".repeat(50));
        
        let mut results = Vec::new();
        let mut passed = 0;
        let mut failed = 0;
        
        for test in &self.tests {
            let start = std::time::Instant::now();
            let result = test();
            let duration = start.elapsed();
            
            let result = TestResult {
                duration_ms: duration.as_millis() as u64,
                ..result
            };
            
            if result.passed {
                println!("✅ {} - {}ms", result.test_name, result.duration_ms);
                passed += 1;
            } else {
                println!("❌ {} - {}ms - {}", 
                    result.test_name, 
                    result.duration_ms,
                    result.error_message.as_deref().unwrap_or("Unknown error")
                );
                failed += 1;
            }
            
            results.push(result);
        }
        
        println!("{}", "=".repeat(50));
        println!("📊 Results: {} passed, {} failed", passed, failed);
        
        if failed == 0 {
            println!("🎉 All tests passed!");
        } else {
            println!("⚠️  {} tests failed!", failed);
        }
        
        results
    }
}
