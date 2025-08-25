//! Test Data Generation
//! 
//! This module generates comprehensive test data for all NEXUS components:
//! - Simple functions for basic testing
//! - Complex algorithms for advanced testing
//! - Large systems for performance testing
//! - Edge cases for robustness testing

use std::collections::HashMap;

/// Test data for compression testing
pub struct CompressionTestData {
    pub simple_functions: Vec<String>,
    pub complex_algorithms: Vec<String>,
    pub large_systems: Vec<String>,
    pub edge_cases: Vec<String>,
}

/// Test data for bridge testing
pub struct BridgeTestData {
    pub python_functions: Vec<String>,
    pub rust_functions: Vec<String>,
    pub javascript_functions: Vec<String>,
    pub cpp_functions: Vec<String>,
    pub go_functions: Vec<String>,
}

/// Test data for performance testing
pub struct PerformanceTestData {
    pub small_datasets: Vec<Vec<i32>>,
    pub medium_datasets: Vec<Vec<i32>>,
    pub large_datasets: Vec<Vec<i32>>,
    pub mixed_datasets: Vec<Vec<f64>>,
}

impl CompressionTestData {
    pub fn new() -> Self {
        Self {
            simple_functions: generate_simple_functions(),
            complex_algorithms: generate_complex_algorithms(),
            large_systems: generate_large_systems(),
            edge_cases: generate_edge_cases(),
        }
    }
    
    /// Get all test data for comprehensive testing
    pub fn all_data(&self) -> Vec<String> {
        let mut all = Vec::new();
        all.extend(self.simple_functions.clone());
        all.extend(self.complex_algorithms.clone());
        all.extend(self.large_systems.clone());
        all.extend(self.edge_cases.clone());
        all
    }
    
    /// Get test data by complexity level
    pub fn by_complexity(&self, level: &str) -> Vec<String> {
        match level {
            "simple" => self.simple_functions.clone(),
            "complex" => self.complex_algorithms.clone(),
            "large" => self.large_systems.clone(),
            "edge" => self.edge_cases.clone(),
            _ => Vec::new(),
        }
    }
}

impl BridgeTestData {
    pub fn new() -> Self {
        Self {
            python_functions: generate_python_functions(),
            rust_functions: generate_rust_functions(),
            javascript_functions: generate_javascript_functions(),
            cpp_functions: generate_cpp_functions(),
            go_functions: generate_go_functions(),
        }
    }
    
    /// Get all functions for a specific language
    pub fn for_language(&self, language: &str) -> Vec<String> {
        match language {
            "python" => self.python_functions.clone(),
            "rust" => self.rust_functions.clone(),
            "javascript" => self.javascript_functions.clone(),
            "cpp" => self.cpp_functions.clone(),
            "go" => self.go_functions.clone(),
            _ => Vec::new(),
        }
    }
    
    /// Get equivalent functions across languages
    pub fn equivalent_functions(&self) -> HashMap<String, Vec<String>> {
        let mut equivalents = HashMap::new();
        
        // Add function
        equivalents.insert("add".to_string(), vec![
            self.python_functions[0].clone(),
            self.rust_functions[0].clone(),
            self.javascript_functions[0].clone(),
            self.cpp_functions[0].clone(),
            self.go_functions[0].clone(),
        ]);
        
        // Fibonacci function
        equivalents.insert("fibonacci".to_string(), vec![
            self.python_functions[1].clone(),
            self.rust_functions[1].clone(),
            self.javascript_functions[1].clone(),
            self.cpp_functions[1].clone(),
            self.go_functions[1].clone(),
        ]);
        
        // Sort function
        equivalents.insert("sort".to_string(), vec![
            self.python_functions[2].clone(),
            self.rust_functions[2].clone(),
            self.javascript_functions[2].clone(),
            self.cpp_functions[2].clone(),
            self.go_functions[2].clone(),
        ]);
        
        equivalents
    }
}

impl PerformanceTestData {
    pub fn new() -> Self {
        Self {
            small_datasets: generate_small_datasets(),
            medium_datasets: generate_medium_datasets(),
            large_datasets: generate_large_datasets(),
            mixed_datasets: generate_mixed_datasets(),
        }
    }
    
    /// Get dataset by size
    pub fn by_size(&self, size: &str) -> Vec<Vec<i32>> {
        match size {
            "small" => self.small_datasets.clone(),
            "medium" => self.medium_datasets.clone(),
            "large" => self.large_datasets.clone(),
            _ => Vec::new(),
        }
    }
    
    /// Get mixed datasets for complex testing
    pub fn mixed(&self) -> Vec<Vec<f64>> {
        self.mixed_datasets.clone()
    }
}

// Data generation functions

fn generate_simple_functions() -> Vec<String> {
    vec![
        "fn add(a: i32, b: i32) -> i32 { a + b }".to_string(),
        "fn multiply(a: i32, b: i32) -> i32 { a * b }".to_string(),
        "fn divide(a: f64, b: f64) -> f64 { a / b }".to_string(),
        "fn is_even(n: i32) -> bool { n % 2 == 0 }".to_string(),
        "fn max(a: i32, b: i32) -> i32 { if a > b { a } else { b } }".to_string(),
    ]
}

fn generate_complex_algorithms() -> Vec<String> {
    vec![
        r#"
        fn quicksort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
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
            
            let mut result = quicksort(&left);
            result.extend(equal);
            result.extend(quicksort(&right));
            result
        }
        "#.to_string(),
        
        r#"
        fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
            let mut left = 0;
            let mut right = arr.len();
            
            while left < right {
                let mid = left + (right - left) / 2;
                match arr[mid].cmp(target) {
                    std::cmp::Ordering::Equal => return Some(mid),
                    std::cmp::Ordering::Less => left = mid + 1,
                    std::cmp::Ordering::Greater => right = mid,
                }
            }
            None
        }
        "#.to_string(),
        
        r#"
        fn fibonacci_dynamic(n: u64) -> u64 {
            if n <= 1 {
                return n;
            }
            let mut a = 0;
            let mut b = 1;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
        "#.to_string(),
    ]
}

fn generate_large_systems() -> Vec<String> {
    vec![
        r#"
        struct WebFramework {
            routes: HashMap<String, Box<dyn Fn(HttpRequest) -> HttpResponse>>,
            middleware: Vec<Box<dyn Fn(HttpRequest, Next) -> HttpResponse>>,
            database: DatabaseConnection,
            cache: Cache,
            logger: Logger,
        }
        
        impl WebFramework {
            pub fn new() -> Self {
                Self {
                    routes: HashMap::new(),
                    middleware: Vec::new(),
                    database: DatabaseConnection::new(),
                    cache: Cache::new(),
                    logger: Logger::new(),
                }
            }
            
            pub fn add_route<F>(&mut self, path: String, handler: F)
            where
                F: Fn(HttpRequest) -> HttpResponse + 'static,
            {
                self.routes.insert(path, Box::new(handler));
            }
            
            pub fn add_middleware<F>(&mut self, middleware: F)
            where
                F: Fn(HttpRequest, Next) -> HttpResponse + 'static,
            {
                self.middleware.push(Box::new(middleware));
            }
            
            pub fn handle_request(&self, request: HttpRequest) -> HttpResponse {
                let mut next = Next::new(&self.middleware);
                next.call(request)
            }
        }
        "#.to_string(),
        
        r#"
        struct MachineLearningPipeline {
            data_preprocessor: DataPreprocessor,
            feature_extractor: FeatureExtractor,
            model: Box<dyn Model>,
            evaluator: ModelEvaluator,
            hyperparameter_tuner: HyperparameterTuner,
        }
        
        impl MachineLearningPipeline {
            pub fn new() -> Self {
                Self {
                    data_preprocessor: DataPreprocessor::new(),
                    feature_extractor: FeatureExtractor::new(),
                    model: Box::new(LinearRegression::new()),
                    evaluator: ModelEvaluator::new(),
                    hyperparameter_tuner: HyperparameterTuner::new(),
                }
            }
            
            pub fn train(&mut self, data: Dataset) -> TrainingResult {
                let preprocessed_data = self.data_preprocessor.preprocess(data);
                let features = self.feature_extractor.extract(preprocessed_data);
                let training_result = self.model.train(features);
                let evaluation = self.evaluator.evaluate(self.model.as_ref());
                
                TrainingResult {
                    model_performance: evaluation,
                    training_metrics: training_result,
                }
            }
        }
        "#.to_string(),
    ]
}

fn generate_edge_cases() -> Vec<String> {
    vec![
        "".to_string(), // Empty string
        "a".to_string(), // Single character
        "a".repeat(10000), // Very long repetitive string
        "fn test() { ".repeat(1000) + "}", // Very long function
        ("fn test() { /* ".to_string() + &"comment ".repeat(1000) + "*/ }"), // Long comments
        r#"
        fn edge_case() {
            // Function with many nested blocks
            if true {
                if true {
                    if true {
                        if true {
                            if true {
                                if true {
                                    if true {
                                        if true {
                                            if true {
                                                if true {
                                                    println!("deep");
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        "#.to_string(),
    ]
}

fn generate_python_functions() -> Vec<String> {
    vec![
        "def add(a, b): return a + b".to_string(),
        "def fibonacci(n): return n if n <= 1 else fibonacci(n-1) + fibonacci(n-2)".to_string(),
        "def sort_list(lst): return sorted(lst)".to_string(),
        "def filter_even(lst): return [x for x in lst if x % 2 == 0]".to_string(),
        "def map_square(lst): return [x**2 for x in lst]".to_string(),
    ]
}

fn generate_rust_functions() -> Vec<String> {
    vec![
        "fn add(a: i32, b: i32) -> i32 { a + b }".to_string(),
        "fn fibonacci(n: u64) -> u64 { if n <= 1 { n } else { fibonacci(n-1) + fibonacci(n-2) } }".to_string(),
        "fn sort_list(mut lst: Vec<i32>) -> Vec<i32> { lst.sort(); lst }".to_string(),
        "fn filter_even(lst: &[i32]) -> Vec<i32> { lst.iter().filter(|&&x| x % 2 == 0).cloned().collect() }".to_string(),
        "fn map_square(lst: &[i32]) -> Vec<i32> { lst.iter().map(|&x| x * x).collect() }".to_string(),
    ]
}

fn generate_javascript_functions() -> Vec<String> {
    vec![
        "function add(a, b) { return a + b; }".to_string(),
        "function fibonacci(n) { return n <= 1 ? n : fibonacci(n-1) + fibonacci(n-2); }".to_string(),
        "function sortList(lst) { return [...lst].sort((a, b) => a - b); }".to_string(),
        "function filterEven(lst) { return lst.filter(x => x % 2 === 0); }".to_string(),
        "function mapSquare(lst) { return lst.map(x => x * x); }".to_string(),
    ]
}

fn generate_cpp_functions() -> Vec<String> {
    vec![
        "int add(int a, int b) { return a + b; }".to_string(),
        "int fibonacci(int n) { return n <= 1 ? n : fibonacci(n-1) + fibonacci(n-2); }".to_string(),
        "std::vector<int> sortList(std::vector<int> lst) { std::sort(lst.begin(), lst.end()); return lst; }".to_string(),
        "std::vector<int> filterEven(const std::vector<int>& lst) { std::vector<int> result; for(int x : lst) if(x % 2 == 0) result.push_back(x); return result; }".to_string(),
        "std::vector<int> mapSquare(const std::vector<int>& lst) { std::vector<int> result; for(int x : lst) result.push_back(x * x); return result; }".to_string(),
    ]
}

fn generate_go_functions() -> Vec<String> {
    vec![
        "func add(a, b int) int { return a + b }".to_string(),
        "func fibonacci(n int) int { if n <= 1 { return n }; return fibonacci(n-1) + fibonacci(n-2) }".to_string(),
        "func sortList(lst []int) []int { sort.Ints(lst); return lst }".to_string(),
        "func filterEven(lst []int) []int { var result []int; for _, x := range lst { if x%2 == 0 { result = append(result, x) } }; return result }".to_string(),
        "func mapSquare(lst []int) []int { result := make([]int, len(lst)); for i, x := range lst { result[i] = x * x }; return result }".to_string(),
    ]
}

fn generate_small_datasets() -> Vec<Vec<i32>> {
    vec![
        vec![1, 2, 3, 4, 5],
        vec![5, 4, 3, 2, 1],
        vec![1, 1, 1, 1, 1],
        vec![1, 3, 5, 7, 9],
        vec![2, 4, 6, 8, 10],
    ]
}

fn generate_medium_datasets() -> Vec<Vec<i32>> {
    vec![
        (1..=100).collect(),
        (100..=1).collect(),
        vec![1; 100],
        (1..=100).filter(|x| x % 2 == 0).collect(),
        (1..=100).filter(|x| x % 2 == 1).collect(),
    ]
}

fn generate_large_datasets() -> Vec<Vec<i32>> {
    vec![
        (1..=10000).collect(),
        (10000..=1).collect(),
        vec![1; 10000],
        (1..=10000).filter(|x| x % 2 == 0).collect(),
        (1..=10000).filter(|x| x % 2 == 1).collect(),
    ]
}

fn generate_mixed_datasets() -> Vec<Vec<f64>> {
    vec![
        (1..=100).map(|x| x as f64).collect(),
        (1..=100).map(|x| (x as f64).sqrt()).collect(),
        (1..=100).map(|x| (x as f64).powi(2)).collect(),
        (1..=100).map(|x| (x as f64).sin()).collect(),
        (1..=100).map(|x| (x as f64).cos()).collect(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compression_test_data_creation() {
        let data = CompressionTestData::new();
        assert!(!data.simple_functions.is_empty());
        assert!(!data.complex_algorithms.is_empty());
        assert!(!data.large_systems.is_empty());
        assert!(!data.edge_cases.is_empty());
    }
    
    #[test]
    fn test_bridge_test_data_creation() {
        let data = BridgeTestData::new();
        assert!(!data.python_functions.is_empty());
        assert!(!data.rust_functions.is_empty());
        assert!(!data.javascript_functions.is_empty());
        assert!(!data.cpp_functions.is_empty());
        assert!(!data.go_functions.is_empty());
    }
    
    #[test]
    fn test_performance_test_data_creation() {
        let data = PerformanceTestData::new();
        assert!(!data.small_datasets.is_empty());
        assert!(!data.medium_datasets.is_empty());
        assert!(!data.large_datasets.is_empty());
        assert!(!data.mixed_datasets.is_empty());
    }
    
    #[test]
    fn test_equivalent_functions() {
        let data = BridgeTestData::new();
        let equivalents = data.equivalent_functions();
        
        assert!(equivalents.contains_key("add"));
        assert!(equivalents.contains_key("fibonacci"));
        assert!(equivalents.contains_key("sort"));
        
        for (_, functions) in equivalents {
            assert_eq!(functions.len(), 5); // 5 languages
        }
    }
}
