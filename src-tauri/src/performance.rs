use crate::error::Result;
use std::collections::HashMap;
use std::time::Instant;
use serde::{Deserialize, Serialize};
use log::{info, warn};

pub const MAX_OPERATION_TIME_MS: u64 = 100;
pub const MAX_MEMORY_USAGE_MB: usize = 50;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub operation_times: HashMap<String, f64>,
    pub memory_usage: HashMap<String, usize>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            operation_times: HashMap::new(),
            memory_usage: HashMap::new(),
            last_updated: chrono::Utc::now(),
        }
    }
}

#[derive(Debug)]
pub struct PerformanceMonitor {
    metrics: PerformanceMetrics,
    alerts: Vec<String>,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            metrics: PerformanceMetrics::default(),
            alerts: Vec::new(),
        }
    }
    
    pub fn measure_operation<F, R>(&mut self, name: &str, operation: F) -> R 
    where F: FnOnce() -> R {
        let start = Instant::now();
        let result = operation();
        let duration = start.elapsed().as_millis() as f64;
        
        self.metrics.operation_times.insert(name.to_string(), duration);
        self.metrics.last_updated = chrono::Utc::now();
        
        // Alert if exceeds threshold
        if duration > MAX_OPERATION_TIME_MS as f64 {
            let alert = format!("Operation '{}' exceeded threshold: {}ms", name, duration);
            warn!("{}", alert);
            self.alerts.push(alert);
        }
        
        info!("Operation '{}' completed in {}ms", name, duration);
        result
    }
    
    pub fn measure_memory_usage(&mut self, context: &str) {
        if let Ok(memory) = get_memory_usage() {
            self.metrics.memory_usage.insert(context.to_string(), memory);
            
            if memory > MAX_MEMORY_USAGE_MB {
                let alert = format!("Memory usage in '{}' exceeded threshold: {}MB", context, memory);
                warn!("{}", alert);
                self.alerts.push(alert);
            }
        }
    }
    
    pub fn get_metrics(&self) -> &PerformanceMetrics {
        &self.metrics
    }
    
    pub fn get_alerts(&self) -> &[String] {
        &self.alerts
    }
    
    pub fn clear_alerts(&mut self) {
        self.alerts.clear();
    }
    
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Performance Report ===\n");
        report.push_str(&format!("Generated: {}\n\n", self.metrics.last_updated));
        
        report.push_str("Operation Times:\n");
        for (op, time) in &self.metrics.operation_times {
            report.push_str(&format!("  {}: {:.2}ms\n", op, time));
        }
        
        report.push_str("\nMemory Usage:\n");
        for (context, memory) in &self.metrics.memory_usage {
            report.push_str(&format!("  {}: {}MB\n", context, memory));
        }
        
        if !self.alerts.is_empty() {
            report.push_str("\nAlerts:\n");
            for alert in &self.alerts {
                report.push_str(&format!("  WARNING: {}\n", alert));
            }
        }
        
        report
    }
}

pub fn get_memory_usage() -> Result<usize> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        
        let output = Command::new("ps")
            .args(&["-p", &std::process::id().to_string(), "-o", "rss="])
            .output()?;
        
        let rss_str = String::from_utf8(output.stdout)?;
        let rss_kb = rss_str.trim().parse::<usize>()?;
        Ok(rss_kb / 1024) // Convert to MB
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        // Fallback for other platforms
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_performance_monitor() {
        let mut monitor = PerformanceMonitor::new();
        
        // Test operation measurement
        let result = monitor.measure_operation("test_operation", || {
            std::thread::sleep(Duration::from_millis(10));
            42
        });
        
        assert_eq!(result, 42);
        assert!(monitor.get_metrics().operation_times.contains_key("test_operation"));
    }
    
    #[test]
    fn test_memory_usage() {
        let mut monitor = PerformanceMonitor::new();
        monitor.measure_memory_usage("test_context");
        
        // Should not panic
        assert!(monitor.get_metrics().memory_usage.contains_key("test_context"));
    }
}