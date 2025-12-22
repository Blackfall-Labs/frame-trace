//! # SAM Utils - Shared Utilities for SAM Ecosystem
//!
//! This crate provides shared utilities used across the SAM (Societal Advisory Module) ecosystem.
//!
//! ## Features
//!
//! - **Execution Tracing**: CallGraph tracking for debugging and performance analysis
//!
//! ## Usage
//!
//! ```rust
//! use sam_utils::trace::{ExecutionTrace, TraceStep, StepType};
//!
//! // Create a new trace with an ID
//! let mut trace = ExecutionTrace::new("trace-001");
//!
//! // Add execution steps
//! let step = TraceStep::new(StepType::Retrieval, "search_documents")
//!     .with_duration(42);
//! trace.add_step(step);
//!
//! // Analyze performance
//! let summary = trace.summary();
//! println!("Total steps: {}", summary.total_steps);
//! ```

pub mod trace;

pub use trace::{ExecutionTrace, StepType, TraceStep};
