//! Execution trace and CallGraph tracking
//!
//! Records the execution flow through SAM's pipeline for debugging,
//! transparency, and performance analysis.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Pipeline step type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepType {
    /// Audio capture started
    AudioCapture,
    /// Voice activity detected
    VoiceActivity,
    /// Speech-to-text transcription
    SpeechToText,
    /// Context retrieval from memory
    Retrieval,
    /// LLM generation
    LlmGeneration,
    /// Tool/skill execution
    ToolExecution,
    /// Text-to-speech synthesis
    TextToSpeech,
    /// Audio playback
    AudioPlayback,
    /// Error occurred
    Error,
}

/// A single step in the execution trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceStep {
    /// Step type
    pub step_type: StepType,

    /// Step name/description
    pub name: String,

    /// Unix timestamp when step started (milliseconds)
    pub start_time_ms: u64,

    /// Duration in milliseconds
    pub duration_ms: u64,

    /// Input data (JSON-serializable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,

    /// Output data (JSON-serializable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,

    /// Additional metadata
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub metadata: HashMap<String, String>,

    /// Error message if step failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl TraceStep {
    /// Create a new trace step
    pub fn new(step_type: StepType, name: impl Into<String>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO);

        Self {
            step_type,
            name: name.into(),
            start_time_ms: now.as_millis() as u64,
            duration_ms: 0,
            input: None,
            output: None,
            metadata: HashMap::new(),
            error: None,
        }
    }

    /// Set input data
    pub fn with_input(mut self, input: serde_json::Value) -> Self {
        self.input = Some(input);
        self
    }

    /// Set output data
    pub fn with_output(mut self, output: serde_json::Value) -> Self {
        self.output = Some(output);
        self
    }

    /// Set duration
    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Set error
    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.step_type = StepType::Error;
        self.error = Some(error.into());
        self
    }
}

/// Complete execution trace for a conversation turn
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionTrace {
    /// Unique trace ID
    pub trace_id: String,

    /// Conversation ID this trace belongs to
    pub conversation_id: Option<u64>,

    /// Turn number in conversation
    pub turn_number: Option<u64>,

    /// All steps in execution order
    pub steps: Vec<TraceStep>,

    /// Total execution time (milliseconds)
    pub total_duration_ms: u64,

    /// Trace start timestamp
    pub start_time_ms: u64,
}

impl ExecutionTrace {
    /// Create a new execution trace
    pub fn new(trace_id: impl Into<String>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO);

        Self {
            trace_id: trace_id.into(),
            conversation_id: None,
            turn_number: None,
            steps: Vec::new(),
            total_duration_ms: 0,
            start_time_ms: now.as_millis() as u64,
        }
    }

    /// Set conversation context
    pub fn with_conversation(mut self, conversation_id: u64, turn_number: u64) -> Self {
        self.conversation_id = Some(conversation_id);
        self.turn_number = Some(turn_number);
        self
    }

    /// Add a step to the trace
    pub fn add_step(&mut self, step: TraceStep) {
        self.steps.push(step);
        self.update_total_duration();
    }

    /// Finalize the trace
    pub fn finalize(&mut self) {
        self.update_total_duration();
    }

    /// Update total duration based on steps
    fn update_total_duration(&mut self) {
        if let (Some(first), Some(last)) = (self.steps.first(), self.steps.last()) {
            self.total_duration_ms =
                (last.start_time_ms + last.duration_ms).saturating_sub(first.start_time_ms);
        }
    }

    /// Export trace as JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Export trace as DOT graph format
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph ExecutionTrace {\n");
        dot.push_str("  rankdir=LR;\n");
        dot.push_str("  node [shape=box];\n\n");

        for (i, step) in self.steps.iter().enumerate() {
            let label = format!("{}\\n{}ms", step.name, step.duration_ms);
            let color = match step.step_type {
                StepType::Error => "red",
                StepType::AudioCapture | StepType::VoiceActivity => "lightblue",
                StepType::SpeechToText | StepType::TextToSpeech => "lightgreen",
                StepType::Retrieval => "lightyellow",
                StepType::LlmGeneration => "orange",
                StepType::ToolExecution => "pink",
                StepType::AudioPlayback => "lightgray",
            };

            dot.push_str(&format!(
                "  step{} [label=\"{}\", fillcolor={}, style=filled];\n",
                i, label, color
            ));

            if i > 0 {
                dot.push_str(&format!("  step{} -> step{};\n", i - 1, i));
            }
        }

        dot.push_str("}\n");
        dot
    }

    /// Get performance summary
    pub fn summary(&self) -> TraceSummary {
        let mut summary = TraceSummary {
            total_steps: self.steps.len(),
            total_duration_ms: self.total_duration_ms,
            step_durations: HashMap::new(),
            errors: Vec::new(),
        };

        for step in &self.steps {
            let type_name = format!("{:?}", step.step_type);
            *summary.step_durations.entry(type_name).or_insert(0) += step.duration_ms;

            if let Some(ref error) = step.error {
                summary.errors.push(format!("{}: {}", step.name, error));
            }
        }

        summary
    }
}

/// Performance summary of an execution trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceSummary {
    /// Total number of steps
    pub total_steps: usize,

    /// Total duration (milliseconds)
    pub total_duration_ms: u64,

    /// Duration by step type
    pub step_durations: HashMap<String, u64>,

    /// List of errors that occurred
    pub errors: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_step_creation() {
        let step = TraceStep::new(StepType::SpeechToText, "Transcribe audio");

        assert_eq!(step.step_type, StepType::SpeechToText);
        assert_eq!(step.name, "Transcribe audio");
        assert_eq!(step.duration_ms, 0);
        assert!(step.input.is_none());
        assert!(step.output.is_none());
    }

    #[test]
    fn test_trace_step_with_data() {
        let step = TraceStep::new(StepType::LlmGeneration, "Generate response")
            .with_input(serde_json::json!({"prompt": "Hello"}))
            .with_output(serde_json::json!({"response": "Hi there!"}))
            .with_duration(250)
            .with_metadata("model", "qwen-2.5-3b");

        assert_eq!(step.duration_ms, 250);
        assert!(step.input.is_some());
        assert!(step.output.is_some());
        assert_eq!(step.metadata.get("model").unwrap(), "qwen-2.5-3b");
    }

    #[test]
    fn test_trace_step_with_error() {
        let step =
            TraceStep::new(StepType::ToolExecution, "Call API").with_error("Network timeout");

        assert_eq!(step.step_type, StepType::Error);
        assert!(step.error.is_some());
        assert_eq!(step.error.unwrap(), "Network timeout");
    }

    #[test]
    fn test_execution_trace() {
        let trace = ExecutionTrace::new("trace-001").with_conversation(1, 5);

        assert_eq!(trace.trace_id, "trace-001");
        assert_eq!(trace.conversation_id, Some(1));
        assert_eq!(trace.turn_number, Some(5));
        assert_eq!(trace.steps.len(), 0);
    }

    #[test]
    fn test_execution_trace_add_steps() {
        let mut trace = ExecutionTrace::new("trace-002");

        let step1 = TraceStep::new(StepType::SpeechToText, "STT").with_duration(100);
        let step2 = TraceStep::new(StepType::LlmGeneration, "LLM").with_duration(300);
        let step3 = TraceStep::new(StepType::TextToSpeech, "TTS").with_duration(150);

        trace.add_step(step1);
        trace.add_step(step2);
        trace.add_step(step3);

        assert_eq!(trace.steps.len(), 3);
        assert!(trace.total_duration_ms > 0);
    }

    #[test]
    fn test_trace_json_serialization() {
        let mut trace = ExecutionTrace::new("trace-003");
        trace.add_step(TraceStep::new(StepType::SpeechToText, "STT").with_duration(100));

        let json = trace.to_json().unwrap();
        assert!(json.contains("trace-003"));
        assert!(json.contains("SpeechToText"));
    }

    #[test]
    fn test_trace_dot_format() {
        let mut trace = ExecutionTrace::new("trace-004");
        trace.add_step(TraceStep::new(StepType::SpeechToText, "STT").with_duration(100));
        trace.add_step(TraceStep::new(StepType::LlmGeneration, "LLM").with_duration(300));

        let dot = trace.to_dot();
        assert!(dot.contains("digraph ExecutionTrace"));
        assert!(dot.contains("step0"));
        assert!(dot.contains("step1"));
        assert!(dot.contains("->"));
    }

    #[test]
    fn test_trace_summary() {
        let mut trace = ExecutionTrace::new("trace-005");

        trace.add_step(TraceStep::new(StepType::SpeechToText, "STT").with_duration(100));
        trace.add_step(TraceStep::new(StepType::LlmGeneration, "LLM 1").with_duration(200));
        trace.add_step(TraceStep::new(StepType::LlmGeneration, "LLM 2").with_duration(150));
        trace.add_step(TraceStep::new(StepType::TextToSpeech, "TTS").with_duration(120));

        let summary = trace.summary();
        assert_eq!(summary.total_steps, 4);
        assert_eq!(*summary.step_durations.get("LlmGeneration").unwrap(), 350);
        assert_eq!(summary.errors.len(), 0);
    }

    #[test]
    fn test_trace_summary_with_errors() {
        let mut trace = ExecutionTrace::new("trace-006");

        trace.add_step(TraceStep::new(StepType::SpeechToText, "STT").with_duration(100));
        trace.add_step(
            TraceStep::new(StepType::ToolExecution, "API Call").with_error("Connection refused"),
        );

        let summary = trace.summary();
        assert_eq!(summary.total_steps, 2);
        assert_eq!(summary.errors.len(), 1);
        assert!(summary.errors[0].contains("Connection refused"));
    }
}
