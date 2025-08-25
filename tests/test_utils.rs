//! Test Utilities
//! 
//! This module provides common testing functionality and helpers:
//! - Test data generation
//! - Assertion helpers
//! - Performance measurement
//! - Common test patterns

use nexus::TestResult;
use std::time::{Duration, Instant};
use std::hash::Hasher;

/// Assert that a condition is true
pub fn assert_test(condition: bool, message: &str) -> Result<(), String> {
    if condition {
        Ok(())
    } else {
        Err(message.to_string())
    }
}

/// Assert that two values are equal
pub fn assert_eq_test<T: PartialEq + std::fmt::Debug>(actual: T, expected: T, message: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!("{}: expected {:?}, got {:?}", message, expected, actual))
    }
}

/// Assert that a value is within a range
pub fn assert_in_range_test(value: f64, min: f64, max: f64, message: &str) -> Result<(), String> {
    if value >= min && value <= max {
        Ok(())
    } else {
        Err(format!("{}: value {} not in range [{}, {}]", message, value, min, max))
    }
}

/// Measure execution time of a function
pub fn measure_time<F, T>(func: F) -> (T, Duration)
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = func();
    let duration = start.elapsed();
    (result, duration)
}

/// Measure execution time and return result with timing
pub fn measure_time_with_result<F, T>(func: F, test_name: &str) -> TestResult
where
    F: FnOnce() -> Result<(), String>,
{
    let (result, duration) = measure_time(func);
    
    match result {
        Ok(()) => TestResult {
            test_name: test_name.to_string(),
            passed: true,
            duration_ms: duration.as_millis() as u64,
            error_message: None,
        },
        Err(error) => TestResult {
            test_name: test_name.to_string(),
            passed: false,
            duration_ms: duration.as_millis() as u64,
            error_message: Some(error),
        },
    }
}

/// Generate test data of specified size
pub fn generate_test_data(size: usize) -> Vec<i32> {
    (0..size).map(|x| x as i32).collect()
}

/// Generate random test data
pub fn generate_random_test_data(size: usize, min: i32, max: i32) -> Vec<i32> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    size.hash(&mut hasher);
    let seed = hasher.finish();
    
    (0..size)
        .map(|i| {
            let mut hasher = DefaultHasher::new();
            (seed + i as u64).hash(&mut hasher);
            let hash = hasher.finish();
            (hash % (max - min + 1) as u64 + min as u64) as i32
        })
        .collect()
}

/// Generate test strings with different characteristics
pub fn generate_test_strings() -> Vec<String> {
    vec![
        "".to_string(),                                    // Empty
        "a".to_string(),                                   // Single character
        "hello world".to_string(),                         // Short string
        "the quick brown fox jumps over the lazy dog".to_string(), // Medium string
        "a".repeat(1000),                                  // Long repetitive
        "abcdefghijklmnopqrstuvwxyz".repeat(100),          // Long pattern
        "fn test() { ".repeat(500) + "}",                  // Long function
    ]
}

/// Generate test functions with different complexity
pub fn generate_test_functions() -> Vec<String> {
    vec![
        "fn add(a: i32, b: i32) -> i32 { a + b }".to_string(),
        r#"
        fn complex_function(data: &[f64]) -> Vec<f64> {
            let mut result = Vec::new();
            for &value in data {
                if value > 0.0 {
                    result.push(value.sqrt());
                }
            }
            result
        }
        "#.to_string(),
        r#"
        fn very_complex_function<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
            if arr.len() <= 1 {
                return arr.to_vec();
            }
            let pivot = &arr[arr.len() / 2];
            let mut left = Vec::new();
            let mut right = Vec::new();
            let mut equal = Vec::new();
            
            for item in arr {
                match item.cmp(pivot) {
                    std::cmp::Ordering::Less => left.push(item.clone()),
                    std::cmp::Ordering::Equal => equal.push(item.clone()),
                    std::cmp::Ordering::Greater => right.push(item.clone()),
                }
            }
            
            let mut result = very_complex_function(&left);
            result.extend(equal);
            result.extend(very_complex_function(&right));
            result
        }
        "#.to_string(),
    ]
}

/// Performance benchmark helper
pub struct Benchmark {
    name: String,
    iterations: usize,
    warmup_iterations: usize,
}

impl Benchmark {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            iterations: 1000,
            warmup_iterations: 100,
        }
    }
    
    pub fn iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }
    
    pub fn warmup_iterations(mut self, warmup: usize) -> Self {
        self.warmup_iterations = warmup;
        self
    }
    
    pub fn run<F>(&self, mut func: F) -> BenchmarkResult
    where
        F: FnMut() -> (),
    {
        // Warmup
        for _ in 0..self.warmup_iterations {
            func();
        }
        
        // Actual benchmark
        let mut times = Vec::new();
        for _ in 0..self.iterations {
            let (_, duration) = measure_time(|| func());
            times.push(duration);
        }
        
        // Calculate statistics
        let total_time: Duration = times.iter().sum();
        let avg_time = total_time / self.iterations as u32;
        let min_time = times.iter().min().unwrap();
        let max_time = times.iter().max().unwrap();
        
        BenchmarkResult {
            name: self.name.clone(),
            iterations: self.iterations,
            total_time,
            avg_time,
            min_time: *min_time,
            max_time: *max_time,
            times,
        }
    }
}

/// Benchmark result with detailed timing information
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: usize,
    pub total_time: Duration,
    pub avg_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub times: Vec<Duration>,
}

impl BenchmarkResult {
    pub fn print_summary(&self) {
        println!("Benchmark: {}", self.name);
        println!("  Iterations: {}", self.iterations);
        println!("  Total time: {:?}", self.total_time);
        println!("  Average time: {:?}", self.avg_time);
        println!("  Min time: {:?}", self.min_time);
        println!("  Max time: {:?}", self.max_time);
        println!("  Throughput: {:.2} ops/sec", 
            self.iterations as f64 / self.total_time.as_secs_f64());
    }
    
    pub fn assert_performance(&self, max_avg_time: Duration, message: &str) -> Result<(), String> {
        if self.avg_time <= max_avg_time {
            Ok(())
        } else {
            Err(format!("{}: average time {:?} exceeds limit {:?}", 
                message, self.avg_time, max_avg_time))
        }
    }
}

/// Test data comparison helper
pub struct DataComparator<T> {
    actual: Vec<T>,
    expected: Vec<T>,
}

impl<T: PartialEq + std::fmt::Debug> DataComparator<T> {
    pub fn new(actual: Vec<T>, expected: Vec<T>) -> Self {
        Self { actual, expected }
    }
    
    pub fn compare(&self) -> Result<(), String> {
        if self.actual.len() != self.expected.len() {
            return Err(format!("Length mismatch: expected {}, got {}", 
                self.expected.len(), self.actual.len()));
        }
        
        for (i, (actual, expected)) in self.actual.iter().zip(self.expected.iter()).enumerate() {
            if actual != expected {
                return Err(format!("Mismatch at index {}: expected {:?}, got {:?}", 
                    i, expected, actual));
            }
        }
        
        Ok(())
    }
    
    pub fn assert_equal(&self, message: &str) -> Result<(), String> {
        self.compare().map_err(|e| format!("{}: {}", message, e))
    }
}

/// Memory usage tracker
pub struct MemoryTracker {
    start_memory: Option<usize>,
}

impl MemoryTracker {
    pub fn new() -> Self {
        Self { start_memory: None }
    }
    
    pub fn start(&mut self) {
        self.start_memory = Some(simulate_memory_usage());
    }
    
    pub fn end(&self) -> Option<usize> {
        self.start_memory.map(|start| {
            let end = simulate_memory_usage();
            if end >= start {
                end - start
            } else {
                0 // Memory decreased
            }
        })
    }
    
    pub fn track<F, T>(&mut self, func: F) -> (T, Option<usize>)
    where
        F: FnOnce() -> T,
    {
        self.start();
        let result = func();
        let memory_used = self.end();
        (result, memory_used)
    }
}

// Simulation functions (replace with actual implementations later)

fn simulate_memory_usage() -> usize {
    // Simulate memory usage measurement
    // In a real implementation, this would use system calls or memory profiling
    std::collections::hash_map::DefaultHasher::new().finish() as usize % 1000000
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_assert_test() {
        assert!(assert_test(true, "should pass").is_ok());
        assert!(assert_test(false, "should fail").is_err());
    }
    
    #[test]
    fn test_assert_eq_test() {
        assert!(assert_eq_test(1, 1, "equal").is_ok());
        assert!(assert_eq_test(1, 2, "not equal").is_err());
    }
    
    #[test]
    fn test_assert_in_range_test() {
        assert!(assert_in_range_test(5.0, 1.0, 10.0, "in range").is_ok());
        assert!(assert_in_range_test(15.0, 1.0, 10.0, "out of range").is_err());
    }
    
    #[test]
    fn test_measure_time() {
        let (_, duration) = measure_time(|| {
            std::thread::sleep(Duration::from_millis(1));
        });
        assert!(duration >= Duration::from_millis(1));
    }
    
    #[test]
    fn test_generate_test_data() {
        let data = generate_test_data(5);
        assert_eq!(data, vec![0, 1, 2, 3, 4]);
    }
    
    #[test]
    fn test_generate_random_test_data() {
        let data = generate_random_test_data(10, 1, 100);
        assert_eq!(data.len(), 10);
        for &value in &data {
            assert!(value >= 1 && value <= 100);
        }
    }
    
    #[test]
    fn test_benchmark() {
        let benchmark = Benchmark::new("test").iterations(100).warmup_iterations(10);
        let result = benchmark.run(|| {
            std::thread::sleep(Duration::from_micros(100));
        });
        
        assert_eq!(result.iterations, 100);
        assert!(result.avg_time >= Duration::from_micros(100));
    }
    
    #[test]
    fn test_data_comparator() {
        let actual = vec![1, 2, 3];
        let expected = vec![1, 2, 3];
        let comparator = DataComparator::new(actual, expected);
        assert!(comparator.compare().is_ok());
    }
    
    #[test]
    fn test_memory_tracker() {
        let mut tracker = MemoryTracker::new();
        let (_, memory_used) = tracker.track(|| {
            // Simulate some work
            vec![0u8; 1000]
        });
        
        assert!(memory_used.is_some());
    }
}
