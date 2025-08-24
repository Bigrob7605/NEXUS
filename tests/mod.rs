//! NEXUS Testing Framework
//! 
//! This module provides comprehensive testing for all NEXUS components:
//! - Theory validation tests (mathematical proofs)
//! - Unit tests (individual components)
//! - Integration tests (multi-component workflows)
//! - Performance tests (benchmarks and validation)

pub mod theory_tests;
pub mod unit_tests;
pub mod integration_tests;
pub mod performance_tests;
pub mod test_data;
pub mod test_utils;

use std::time::Instant;

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
            let start = Instant::now();
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

/// Run all test suites
pub fn run_all_tests() -> Vec<TestResult> {
    println!("🚀 Starting NEXUS Test Suite");
    println!("{}", "=".repeat(60));
    
    let mut all_results = Vec::new();
    
    // Run theory tests
    let mut theory_suite = TestSuite::new("Theory Validation");
    theory_tests::register_tests(&mut theory_suite);
    all_results.extend(theory_suite.run_all());
    
    // Run unit tests
    let mut unit_suite = TestSuite::new("Unit Tests");
    unit_tests::register_tests(&mut unit_suite);
    all_results.extend(unit_suite.run_all());
    
    // Run integration tests
    let mut integration_suite = TestSuite::new("Integration Tests");
    integration_tests::register_tests(&mut integration_suite);
    all_results.extend(integration_suite.run_all());
    
    // Run performance tests
    let mut performance_suite = TestSuite::new("Performance Tests");
    performance_tests::register_tests(&mut performance_suite);
    all_results.extend(performance_suite.run_all());
    
    // Summary
    let total_tests = all_results.len();
    let passed_tests = all_results.iter().filter(|r| r.passed).count();
    let failed_tests = total_tests - passed_tests;
    
    println!("{}", "=".repeat(60));
    println!("🏁 Test Suite Complete");
    println!("📊 Total: {}, Passed: {}, Failed: {}", total_tests, passed_tests, failed_tests);
    
    if failed_tests == 0 {
        println!("🎉 SUCCESS: All tests passed! Ready to proceed with implementation.");
    } else {
        println!("❌ FAILURE: {} tests failed. Must fix all failures before proceeding.", failed_tests);
    }
    
    all_results
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_test_suite_creation() {
        let mut suite = TestSuite::new("Test Suite");
        assert_eq!(suite.name, "Test Suite");
        assert_eq!(suite.tests.len(), 0);
    }
    
    #[test]
    fn test_test_suite_add_test() {
        let mut suite = TestSuite::new("Test Suite");
        suite.add_test(|| TestResult {
            test_name: "Test 1".to_string(),
            passed: true,
            duration_ms: 0,
            error_message: None,
        });
        assert_eq!(suite.tests.len(), 1);
    }
    
    #[test]
    fn test_test_result_creation() {
        let result = TestResult {
            test_name: "Test".to_string(),
            passed: true,
            duration_ms: 100,
            error_message: None,
        };
        
        assert_eq!(result.test_name, "Test");
        assert!(result.passed);
        assert_eq!(result.duration_ms, 100);
        assert!(result.error_message.is_none());
    }
}
