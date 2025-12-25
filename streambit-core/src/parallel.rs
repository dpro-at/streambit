//! Parallel processing utilities using Rayon
//!
//! This module provides utilities for parallel processing of data using Rayon.
//! All StreamBit modules use these utilities to maximize CPU utilization.

use rayon::prelude::*;

/// Configuration for parallel processing
///
/// This struct allows you to configure the number of threads used for
/// parallel operations. By default, Rayon uses all available CPU cores.
#[derive(Debug, Clone)]
pub struct ParallelConfig {
    /// Number of threads to use (None = auto-detect all cores)
    pub num_threads: Option<usize>,
}

impl Default for ParallelConfig {
    fn default() -> Self {
        Self { num_threads: None }
    }
}

impl ParallelConfig {
    /// Create a new parallel configuration with auto-detected thread count
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the number of threads to use
    ///
    /// # Examples
    ///
    /// ```rust
    /// use streambit_core::parallel::ParallelConfig;
    ///
    /// let config = ParallelConfig::new().with_threads(4);
    /// ```
    pub fn with_threads(mut self, num_threads: usize) -> Self {
        self.num_threads = Some(num_threads);
        self
    }

    /// Apply the configuration to the Rayon thread pool
    ///
    /// This sets the global thread pool size. Note that this can only be
    /// called once per process.
    pub fn apply(&self) -> rayon::ThreadPoolBuildResult<()> {
        if let Some(num_threads) = self.num_threads {
            rayon::ThreadPoolBuilder::new()
                .num_threads(num_threads)
                .build_global()
        } else {
            Ok(())
        }
    }

    /// Get the number of threads that will be used
    ///
    /// Returns the configured number of threads, or the number of available
    /// CPU cores if not configured.
    pub fn thread_count(&self) -> usize {
        self.num_threads.unwrap_or_else(num_cpus::get)
    }
}

/// Process items in parallel with a given function
///
/// This is a convenience function that processes a vector of items in parallel
/// using Rayon. Each item is processed independently.
///
/// # Examples
///
/// ```rust
/// use streambit_core::parallel::process_parallel;
///
/// let items = vec![1, 2, 3, 4, 5];
/// let results = process_parallel(items, |x| x * 2);
/// assert_eq!(results, vec![2, 4, 6, 8, 10]);
/// ```
pub fn process_parallel<T, R, F>(items: Vec<T>, f: F) -> Vec<R>
where
    T: Send,
    R: Send,
    F: Fn(T) -> R + Sync + Send,
{
    items.into_par_iter().map(f).collect()
}

/// Process items in parallel with error handling
///
/// Similar to `process_parallel`, but allows the processing function to return
/// a Result. If any item fails to process, the entire operation fails.
///
/// # Examples
///
/// ```rust
/// use streambit_core::parallel::try_process_parallel;
///
/// let items = vec![1, 2, 3, 4, 5];
/// let results: Result<Vec<_>, &str> = try_process_parallel(items, |x| {
///     if x > 3 {
///         Err("too large")
///     } else {
///         Ok(x * 2)
///     }
/// });
/// assert!(results.is_err());
/// ```
pub fn try_process_parallel<T, R, E, F>(items: Vec<T>, f: F) -> Result<Vec<R>, E>
where
    T: Send,
    R: Send,
    E: Send,
    F: Fn(T) -> Result<R, E> + Sync + Send,
{
    items.into_par_iter().map(f).collect()
}

/// Process items in parallel with indexed access
///
/// Similar to `process_parallel`, but the processing function receives both
/// the index and the item.
pub fn process_parallel_indexed<T, R, F>(items: Vec<T>, f: F) -> Vec<R>
where
    T: Send,
    R: Send,
    F: Fn(usize, T) -> R + Sync + Send,
{
    items
        .into_par_iter()
        .enumerate()
        .map(|(i, item)| f(i, item))
        .collect()
}

// Note: num_cpus is not in workspace dependencies, so we'll use rayon's thread count
fn num_cpus_get() -> usize {
    rayon::current_num_threads()
}

// Helper to get CPU count
mod num_cpus {
    pub fn get() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_processing() {
        let items = vec![1, 2, 3, 4, 5];
        let results = process_parallel(items, |x| x * 2);
        assert_eq!(results, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_parallel_with_errors() {
        let items = vec![1, 2, 3, 4, 5];
        let results: Result<Vec<_>, &str> = try_process_parallel(items, |x| {
            if x == 3 {
                Err("Error at 3")
            } else {
                Ok(x * 2)
            }
        });
        assert!(results.is_err());
    }

    #[test]
    fn test_parallel_indexed() {
        let items = vec!["a", "b", "c"];
        let results = process_parallel_indexed(items, |i, s| format!("{}{}", i, s));
        assert_eq!(results, vec!["0a", "1b", "2c"]);
    }

    #[test]
    fn test_config() {
        let config = ParallelConfig::new().with_threads(4);
        assert_eq!(config.num_threads, Some(4));
    }
}
