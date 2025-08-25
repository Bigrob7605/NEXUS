use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::error::Error;
use std::fmt;

/// Represents an AI/ML process with specific resource requirements
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AIProcess {
    pub pid: u32,
    pub priority: u32,
    pub gpu_requirements: Vec<u32>,
    pub memory_requirements: u64,
    pub estimated_runtime: Duration,
    pub created_at: Instant,
    pub model_type: String,
    pub batch_size: u32,
}

/// Custom ordering for AIProcess based on priority and resource efficiency
impl Ord for AIProcess {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Higher priority first (BinaryHeap is max-heap, so higher priority should be "greater")
        match self.priority.cmp(&other.priority) {
            std::cmp::Ordering::Equal => {
                // If same priority, prefer shorter runtime for better resource utilization
                other.estimated_runtime.cmp(&self.estimated_runtime) // Shorter runtime is "greater"
            }
            other => other,
        }
    }
}

impl PartialOrd for AIProcess {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// GPU resource allocation information
#[derive(Debug, Clone)]
pub struct GPUAllocation {
    pub gpu_id: u32,
    pub memory_allocated: u64,
    pub compute_utilization: f32,
    pub memory_utilization: f32,
    pub process_id: Option<u32>,
}

/// Memory block allocation
#[derive(Debug, Clone)]
pub struct MemoryBlock {
    pub address: u64,
    pub size: u64,
    pub process_id: u32,
    pub allocated_at: Instant,
}

/// GPU memory manager for efficient allocation
pub struct GPUMemoryManager {
    gpus: Vec<GPUAllocation>,
    total_gpu_memory: u64,
}

impl GPUMemoryManager {
    pub fn new(gpu_count: u32, memory_per_gpu: u64) -> Self {
        let mut gpus = Vec::new();
        for i in 0..gpu_count {
            gpus.push(GPUAllocation {
                gpu_id: i,
                memory_allocated: 0,
                compute_utilization: 0.0,
                memory_utilization: 0.0,
                process_id: None,
            });
        }
        
        Self {
            gpus,
            total_gpu_memory: gpu_count as u64 * memory_per_gpu,
        }
    }
    
    pub fn can_allocate_gpu(&self, gpu_id: u32, memory_needed: u64) -> bool {
        if let Some(gpu) = self.gpus.get(gpu_id as usize) {
            gpu.memory_allocated + memory_needed <= self.total_gpu_memory / self.gpus.len() as u64
        } else {
            false
        }
    }
    
    pub fn allocate_gpu(&mut self, gpu_id: u32, memory_needed: u64, process_id: u32) -> Result<(), SchedulerError> {
        // Calculate available memory before getting mutable borrow
        let available_memory = self.total_gpu_memory / self.gpus.len() as u64;
        
        if let Some(gpu) = self.gpus.get_mut(gpu_id as usize) {
            let can_allocate = gpu.memory_allocated + memory_needed <= available_memory;
            
            if can_allocate {
                gpu.memory_allocated += memory_needed;
                gpu.process_id = Some(process_id);
                Ok(())
            } else {
                Err(SchedulerError::InsufficientGPUResources)
            }
        } else {
            Err(SchedulerError::InvalidGPUId)
        }
    }
    
    pub fn free_gpu(&mut self, gpu_id: u32, memory_freed: u64) -> Result<(), SchedulerError> {
        if let Some(gpu) = self.gpus.get_mut(gpu_id as usize) {
            if gpu.memory_allocated >= memory_freed {
                gpu.memory_allocated -= memory_freed;
                gpu.process_id = None;
                Ok(())
            } else {
                Err(SchedulerError::InvalidMemoryFree)
            }
        } else {
            Err(SchedulerError::InvalidGPUId)
        }
    }
    
    /// Release GPU resources for a specific process
    pub fn release_gpu(&mut self, gpu_id: u32, process_id: u32) -> Result<(), SchedulerError> {
        if let Some(gpu) = self.gpus.get_mut(gpu_id as usize) {
            if gpu.process_id == Some(process_id) {
                let memory_freed = gpu.memory_allocated;
                gpu.memory_allocated = 0;
                gpu.process_id = None;
                Ok(())
            } else {
                Err(SchedulerError::InvalidProcessId)
            }
        } else {
            Err(SchedulerError::InvalidGPUId)
        }
    }
}

/// System memory manager
pub struct MemoryManager {
    total_memory: u64,
    allocated_memory: u64,
    memory_blocks: Vec<MemoryBlock>,
}

impl MemoryManager {
    pub fn new(total_memory: u64) -> Self {
        Self {
            total_memory,
            allocated_memory: 0,
            memory_blocks: Vec::new(),
        }
    }
    
    pub fn can_allocate(&self, size: u64) -> bool {
        self.allocated_memory + size <= self.total_memory
    }
    
    pub fn allocate(&mut self, size: u64, process_id: u32) -> Result<u64, SchedulerError> {
        if !self.can_allocate(size) {
            return Err(SchedulerError::InsufficientMemory);
        }
        
        // Simple allocation strategy - can be improved with proper memory management
        let address = self.allocated_memory;
        self.allocated_memory += size;
        
        self.memory_blocks.push(MemoryBlock {
            address,
            size,
            process_id,
            allocated_at: Instant::now(),
        });
        
        Ok(address)
    }
    
    pub fn free(&mut self, process_id: u32) -> Result<u64, SchedulerError> {
        let mut freed_memory = 0;
        self.memory_blocks.retain(|block| {
            if block.process_id == process_id {
                freed_memory += block.size;
                false
            } else {
                true
            }
        });
        
        self.allocated_memory -= freed_memory;
        Ok(freed_memory)
    }
}

/// Custom error types for the scheduler
#[derive(Debug)]
pub enum SchedulerError {
    InsufficientGPUResources,
    InsufficientMemory,
    InvalidGPUId,
    InvalidMemoryFree,
    ProcessNotFound,
    ResourceAllocationFailed,
    InvalidProcessId,
}

impl fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SchedulerError::InsufficientGPUResources => write!(f, "Insufficient GPU resources"),
            SchedulerError::InsufficientMemory => write!(f, "Insufficient system memory"),
            SchedulerError::InvalidGPUId => write!(f, "Invalid GPU ID"),
            SchedulerError::InvalidMemoryFree => write!(f, "Invalid memory free operation"),
            SchedulerError::ProcessNotFound => write!(f, "Process not found"),
            SchedulerError::ResourceAllocationFailed => write!(f, "Resource allocation failed"),
            SchedulerError::InvalidProcessId => write!(f, "Invalid process ID"),
        }
    }
}

impl Error for SchedulerError {}

/// Main AI process scheduler
pub struct AIScheduler {
    processes: Arc<Mutex<BinaryHeap<AIProcess>>>,
    running_processes: Arc<Mutex<Vec<AIProcess>>>,
    gpu_manager: Arc<Mutex<GPUMemoryManager>>,
    memory_manager: Arc<Mutex<MemoryManager>>,
    stats: Arc<Mutex<SchedulerStats>>,
}

/// Scheduler performance statistics
#[derive(Debug, Default, Clone)]
pub struct SchedulerStats {
    pub total_processes_scheduled: u64,
    pub total_gpu_utilization: f32,
    pub total_memory_utilization: f32,
    pub average_scheduling_time: Duration,
    pub failed_allocations: u64,
}

impl AIScheduler {
    pub fn new(gpu_count: u32, gpu_memory: u64, system_memory: u64) -> Self {
        Self {
            processes: Arc::new(Mutex::new(BinaryHeap::new())),
            running_processes: Arc::new(Mutex::new(Vec::new())),
            gpu_manager: Arc::new(Mutex::new(GPUMemoryManager::new(gpu_count, gpu_memory))),
            memory_manager: Arc::new(Mutex::new(MemoryManager::new(system_memory))),
            stats: Arc::new(Mutex::new(SchedulerStats::default())),
        }
    }
    
    /// Add a new AI process to the scheduling queue
    pub fn add_process(&self, process: AIProcess) -> Result<(), SchedulerError> {
        let mut processes = self.processes.lock().unwrap();
        processes.push(process);
        Ok(())
    }
    
    /// Schedule processes based on available resources
    pub fn schedule(&self) -> Result<Vec<AIProcess>, SchedulerError> {
        let start_time = Instant::now();
        let mut processes = self.processes.lock().unwrap();
        let mut gpu_manager = self.gpu_manager.lock().unwrap();
        let mut memory_manager = self.memory_manager.lock().unwrap();
        let mut running = self.running_processes.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        let mut scheduled = Vec::new();
        let mut failed = Vec::new();
        
        while let Some(process) = processes.pop() {
            if self.can_allocate_resources(&process, &gpu_manager, &memory_manager) {
                // Allocate GPU resources
                for &gpu_id in &process.gpu_requirements {
                    if let Err(_e) = gpu_manager.allocate_gpu(gpu_id, process.memory_requirements, process.pid) {
                        failed.push(process.clone());
                        stats.failed_allocations += 1;
                        continue;
                    }
                }
                
                // Allocate system memory
                if let Err(_e) = memory_manager.allocate(process.memory_requirements, process.pid) {
                    // Rollback GPU allocation
                    for &gpu_id in &process.gpu_requirements {
                        let _ = gpu_manager.free_gpu(gpu_id, process.memory_requirements);
                    }
                    failed.push(process.clone());
                    stats.failed_allocations += 1;
                    continue;
                }
                
                scheduled.push(process.clone());
                running.push(process);
                stats.total_processes_scheduled += 1;
            } else {
                failed.push(process);
                stats.failed_allocations += 1;
            }
        }
        
        // Put failed processes back in queue
        for process in failed {
            processes.push(process);
        }
        
        // Update statistics
        stats.average_scheduling_time = start_time.elapsed();
        
        Ok(scheduled)
    }
    
    /// Check if resources can be allocated for a process
    fn can_allocate_resources(
        &self,
        process: &AIProcess,
        gpu_manager: &GPUMemoryManager,
        memory_manager: &MemoryManager,
    ) -> bool {
        // Check GPU availability
        for &gpu_id in &process.gpu_requirements {
            if !gpu_manager.can_allocate_gpu(gpu_id, process.memory_requirements) {
                return false;
            }
        }
        
        // Check system memory
        if !memory_manager.can_allocate(process.memory_requirements) {
            return false;
        }
        
        true
    }
    
    /// Get current scheduler statistics
    pub fn get_stats(&self) -> SchedulerStats {
        self.stats.lock().unwrap().clone()
    }
    
    /// Get GPU utilization information
    pub fn get_gpu_utilization(&self) -> Vec<GPUAllocation> {
        self.gpu_manager.lock().unwrap().gpus.clone()
    }
    
    /// Get memory utilization information
    pub fn get_memory_utilization(&self) -> (u64, u64) {
        let memory_manager = self.memory_manager.lock().unwrap();
        (memory_manager.allocated_memory, memory_manager.total_memory)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_process_priority_ordering() {
        let mut processes = BinaryHeap::new();
        
        let low_priority = AIProcess {
            pid: 1,
            priority: 1,
            gpu_requirements: vec![0],
            memory_requirements: 1024,
            estimated_runtime: Duration::from_secs(10),
            created_at: Instant::now(),
            model_type: "test".to_string(),
            batch_size: 32,
        };
        
        let high_priority = AIProcess {
            pid: 2,
            priority: 10,
            gpu_requirements: vec![0],
            memory_requirements: 1024,
            estimated_runtime: Duration::from_secs(5),
            created_at: Instant::now(),
            model_type: "test".to_string(),
            batch_size: 32,
        };
        
        processes.push(low_priority);
        processes.push(high_priority);
        
        // High priority should come first
        let first = processes.pop().unwrap();
        assert_eq!(first.pid, 2);
        assert_eq!(first.priority, 10);
    }
    
    #[test]
    fn test_gpu_memory_allocation() {
        let mut gpu_manager = GPUMemoryManager::new(2, 8192);
        
        // Allocate memory on GPU 0
        assert!(gpu_manager.allocate_gpu(0, 1024, 1).is_ok());
        
        // Try to allocate more than available
        assert!(gpu_manager.allocate_gpu(0, 8000, 2).is_err());
        
        // Free memory
        assert!(gpu_manager.free_gpu(0, 1024).is_ok());
    }
    
    #[test]
    fn test_scheduler_creation() {
        let scheduler = AIScheduler::new(4, 8192, 16384);
        let stats = scheduler.get_stats();
        assert_eq!(stats.total_processes_scheduled, 0);
    }
}
