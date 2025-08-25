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
pub mod test_enhanced_compression;

use std::time::Instant;

// Re-export test types from the main crate
pub use nexus::{TestResult, TestSuite};

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
