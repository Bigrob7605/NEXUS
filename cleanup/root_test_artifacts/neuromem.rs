//! Simplified Neuromorphic-inspired memory manager for KAI-OS
//! Lightweight, concurrency-safe structure for recording access patterns

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// Timestamp as milliseconds since epoch.
pub type Timestamp = u64;

/// Simple helper to get the current timestamp (ms).
pub fn now_ms() -> Timestamp {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as Timestamp
}

/// Patterns of memory access used for classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessPattern {
    Sequential,
    Random,
    Strided,
    Clustered,
    Temporal,
    Spatial,
}

/// A recorded access event ("spike") used for temporal analysis.
#[derive(Debug, Clone)]
pub struct MemorySpike {
    pub timestamp: Timestamp,
    pub region_id: u64,
    pub strength: f32,
    pub pattern: AccessPattern,
    pub plasticity: f32,
}

/// A memory region managed by the system.
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub region_id: u64,
    pub size: usize,
    pub synaptic_strength: f32,
    pub access_frequency: u64,
    pub last_access: Timestamp,
    pub memory_type: MemoryType,
    pub pathway: Vec<f64>,
    pub plasticity: f32,
}

/// Types of memory regions (informational).
#[derive(Debug, Clone, Copy)]
pub enum MemoryType {
    Code,
    Data,
    Cache,
    ModelWeights,
    IOBuffer,
}

/// Learning event recorded for diagnostics.
#[derive(Debug, Clone)]
pub struct LearningEvent {
    pub timestamp: Timestamp,
    pub change: f32,
    pub description: String,
}

/// Minimal learning engine storing parameters and history.
#[derive(Debug, Clone)]
pub struct LearningEngine {
    pub learning_rate: f32,
    pub momentum: f32,
    pub decay: f32,
    pub threshold: f32,
    pub history: Vec<LearningEvent>,
}

impl LearningEngine {
    pub fn new() -> Self {
        Self {
            learning_rate: 0.1,
            momentum: 0.9,
            decay: 0.001,
            threshold: 0.01,
            history: Vec::new(),
        }
    }

    /// Record a learning event for diagnostics.
    pub fn record_event(&mut self, change: f32, description: impl Into<String>) {
        let ev = LearningEvent {
            timestamp: now_ms(),
            change,
            description: description.into(),
        };
        self.history.push(ev);
    }

    /// Adjust learning rate based on improvement signal.
    pub fn adapt(&mut self, improvement: f32) {
        if improvement > self.threshold {
            self.learning_rate = (self.learning_rate * 1.05).min(0.5);
        } else {
            self.learning_rate = (self.learning_rate * 0.95).max(0.01);
        }
        // apply decay
        self.learning_rate *= 1.0 - self.decay;
    }

    pub fn stats(&self) -> (usize, f32) {
        let total = self.history.len();
        let avg = if total > 0 {
            self.history.iter().map(|e| e.change).sum::<f32>() / total as f32
        } else {
            0.0
        };
        (total, avg)
    }
}

/// Aggregate statistics returned by the manager.
#[derive(Debug, Clone, Default)]
pub struct MemStats {
    pub total_allocated: usize,
    pub region_count: usize,
    pub total_accesses: usize,
    pub avg_spike_strength: f32,
    pub synaptic_efficiency: f32,
    pub temporal_coherence: f32,
}

/// Core manager struct.
/// Thread-safe via Arc<Mutex<...>> so it can be shared across threads.
#[derive(Clone, Debug)]
pub struct Neuromem {
    pub synaptic_weights: Arc<Mutex<Vec<f32>>>,
    pub regions: Arc<Mutex<HashMap<u64, MemoryRegion>>>,
    pub spike_history: Arc<Mutex<VecDeque<MemorySpike>>>,
    pub engine: Arc<Mutex<LearningEngine>>,
    pub max_history: usize,
}

impl Neuromem {
    /// Create a new manager with N synaptic weights.
    pub fn new(weights: usize, max_history: usize) -> Self {
        let init_weights = (0..weights)
            .map(|_| 0.5f32)
            .collect();

        Self {
            synaptic_weights: Arc::new(Mutex::new(init_weights)),
            regions: Arc::new(Mutex::new(HashMap::new())),
            spike_history: Arc::new(Mutex::new(VecDeque::with_capacity(max_history))),
            engine: Arc::new(Mutex::new(LearningEngine::new())),
            max_history,
        }
    }

    /// Create and register a new memory region, returns its id.
    pub fn create_region(&self, size: usize, mem_type: MemoryType) -> Result<u64, String> {
        let region_id = now_ms(); // simple unique-ish id
        let region = MemoryRegion {
            region_id,
            size,
            synaptic_strength: 0.5,
            access_frequency: 0,
            last_access: now_ms(),
            memory_type: mem_type,
            pathway: vec![0.1, 0.2],
            plasticity: 0.1,
        };

        let mut regions = self.regions.lock()
            .map_err(|e| format!("Failed to lock regions: {}", e))?;
        regions.insert(region_id, region);
        Ok(region_id)
    }

    /// Record an access to a region, update spike history, and apply plasticity.
    pub fn record_access(&self, region_id: u64, pattern: AccessPattern) -> Result<(), String> {
        let ts = now_ms();
        // Create spike
        let spike = MemorySpike {
            timestamp: ts,
            region_id,
            strength: 1.0,
            pattern,
            plasticity: 0.1,
        };

        // Append to history
        {
            let mut hist = self.spike_history.lock()
                .map_err(|e| format!("Failed to lock spike history: {}", e))?;
            hist.push_back(spike);
            if hist.len() > self.max_history {
                hist.pop_front();
            }
        }

        // Update region metadata
        {
            let mut regs = self.regions.lock()
                .map_err(|e| format!("Failed to lock regions: {}", e))?;
            if let Some(r) = regs.get_mut(&region_id) {
                r.access_frequency = r.access_frequency.saturating_add(1);
                r.last_access = ts;
            }
        }

        // Apply synaptic update
        self.apply_plasticity(region_id, pattern)?;
        Ok(())
    }

    /// Internal synaptic plasticity update.
    fn apply_plasticity(&self, region_id: u64, pattern: AccessPattern) -> Result<(), String> {
        let mut weights = self.synaptic_weights.lock()
            .map_err(|e| format!("Failed to lock synaptic weights: {}", e))?;
        let idx = (region_id as usize) % weights.len();
        let pathway_strength = match pattern {
            AccessPattern::Sequential => 0.8,
            AccessPattern::Random => 0.3,
            AccessPattern::Strided => 0.6,
            AccessPattern::Clustered => 0.7,
            AccessPattern::Temporal => 0.9,
            AccessPattern::Spatial => 0.5,
        };

        // simple exponential moving step toward pathway_strength
        weights[idx] = weights[idx] * 0.85 + 0.15 * pathway_strength as f32;
        Ok(())
    }

    /// Optimize memory layout: compute per-region "optimal" strength and update weights.
    /// Returns average absolute change applied.
    pub fn optimize_layout(&self) -> Result<f32, String> {
        let mut total_change = 0.0f32;
        let mut count = 0usize;

        let regs = self.regions.lock()
            .map_err(|e| format!("Failed to lock regions: {}", e))?;
        let mut weights = self.synaptic_weights.lock()
            .map_err(|e| format!("Failed to lock synaptic weights: {}", e))?;

        for r in regs.values() {
            let idx = (r.region_id as usize) % weights.len();
            let current = weights[idx];
            let optimal = self.calculate_optimal_strength(r);
            let change = (optimal - current).abs();
            weights[idx] = current * 0.8 + optimal * 0.2;
            total_change += change;
            count += 1;
        }

        let avg = if count > 0 { total_change / count as f32 } else { 0.0 };

        // Record learning event
        {
            let mut en = self.engine.lock()
                .map_err(|e| format!("Failed to lock learning engine: {}", e))?;
            en.record_event(avg, "optimize_layout");
            en.adapt(avg);
        }

        Ok(avg)
    }

    /// Heuristic for per-region optimal strength.
    fn calculate_optimal_strength(&self, region: &MemoryRegion) -> f32 {
        let freq = region.access_frequency as f32;
        let base = (freq / 1000.0).min(1.0);
        let plastic_boost = region.plasticity * 0.3;
        (base + plastic_boost).min(1.0).max(0.0)
    }

    /// Return some aggregate statistics.
    pub fn stats(&self) -> Result<MemStats, String> {
        let regs = self.regions.lock()
            .map_err(|e| format!("Failed to lock regions: {}", e))?;
        let hist = self.spike_history.lock()
            .map_err(|e| format!("Failed to lock spike history: {}", e))?;
        let weights = self.synaptic_weights.lock()
            .map_err(|e| format!("Failed to lock synaptic weights: {}", e))?;
        let en = self.engine.lock()
            .map_err(|e| format!("Failed to lock learning engine: {}", e))?;

        let total_allocated = regs.values().map(|r| r.size).sum();
        let region_count = regs.len();
        let total_accesses = hist.len();
        let avg_spike_strength = if total_accesses > 0 {
            hist.iter().map(|s| s.strength).sum::<f32>() / total_accesses as f32
        } else {
            0.0
        };
        let syn_eff = if weights.is_empty() {
            0.0
        } else {
            weights.iter().copied().sum::<f32>() / weights.len() as f32
        };
        // crude temporal coherence: inverse of average delta between spikes
        let temporal_coherence = {
            let mut prev = None;
            let mut diffs = Vec::new();
            for s in hist.iter() {
                if let Some(p) = prev {
                    let d = (s.timestamp as i128 - p as i128).abs() as f32;
                    diffs.push(d);
                }
                prev = Some(s.timestamp);
            }
            if diffs.is_empty() {
                0.0
            } else {
                let avg = diffs.iter().sum::<f32>() / diffs.len() as f32;
                1.0 / (1.0 + avg / 1000.0)
            }
        };

        Ok(MemStats {
            total_allocated,
            region_count,
            total_accesses,
            avg_spike_strength,
            synaptic_efficiency: syn_eff,
            temporal_coherence,
        })
    }

    /// Get a snapshot clone of a region (if present).
    pub fn get_region(&self, region_id: u64) -> Result<Option<MemoryRegion>, String> {
        let regs = self.regions.lock()
            .map_err(|e| format!("Failed to lock regions: {}", e))?;
        Ok(regs.get(&region_id).cloned())
    }

    /// Get a snapshot clone of a region (if present).
    pub fn get_region_snapshot(&self, region_id: u64) -> Option<MemoryRegion> {
        let regions = self.regions.lock().ok()?;
        regions.get(&region_id).cloned()
    }

    /// Create a snapshot of the current memory state
    pub fn snapshot(&self) -> Result<MemorySnapshot, String> {
        let regions = self.regions.lock()
            .map_err(|e| format!("Failed to lock regions: {}", e))?;
        
        let regions_count = regions.len();
        let total_memory: usize = regions.values().map(|r| r.size).sum();
        
        Ok(MemorySnapshot {
            timestamp: now_ms(),
            regions_count,
            total_memory,
        })
    }

    /// Remove a region (useful for lifecycle management).
    pub fn remove_region(&self, region_id: u64) -> Result<bool, String> {
        let mut regs = self.regions.lock()
            .map_err(|e| format!("Failed to lock regions: {}", e))?;
        Ok(regs.remove(&region_id).is_some())
    }
}

impl Default for Neuromem {
    fn default() -> Self {
        Self::new(1024, 1000)
    }
}

/// Memory snapshot for external use
#[derive(Debug, Clone)]
pub struct MemorySnapshot {
    pub timestamp: u64,
    pub regions_count: usize,
    pub total_memory: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_record_optimize() {
        let nm = Neuromem::new(128, 500);
        let rid = nm.create_region(4096, MemoryType::Data).unwrap();
        for _ in 0..10 {
            nm.record_access(rid, AccessPattern::Sequential).unwrap();
        }
        let before = nm.get_region(rid).unwrap().unwrap();
        assert!(before.access_frequency >= 10);
        let avg_change = nm.optimize_layout().unwrap();
        assert!(avg_change >= 0.0);
        let s = nm.stats().unwrap();
        assert_eq!(s.region_count, 1);
    }
}
