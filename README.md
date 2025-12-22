# Frame Trace - Shared Utilities for SAM Ecosystem

**Shared debugging and utility functions for the Frame ecosystem.**

## Features

### 🔍 Execution Tracing

CallGraph tracking for debugging, transparency, and performance analysis.

- **Pipeline visualization**: Track execution flow through complex systems
- **Performance profiling**: Measure duration of each step
- **Debugging**: Understand call chains and data flow
- **Transparency**: Export execution traces for analysis

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
frame-trace = "0.1.0"
```

## Quick Start

### Basic Execution Tracing

```rust
use sam_utils::trace::{ExecutionTrace, StepType};

fn main() {
    let mut trace = ExecutionTrace::new();

    // Start a step
    trace.start_step(StepType::Retrieval, "search_documents");

    // Do work...
    let results = search_documents("Rust async");

    // End the step
    trace.end_step();

    // Add another step
    trace.start_step(StepType::LlmGeneration, "generate_response");
    let response = generate_response(&results);
    trace.end_step();

    // Analyze performance
    println!("Total execution time: {}ms", trace.total_duration_ms());
    println!("Step count: {}", trace.steps().len());

    // Export as JSON
    let json = serde_json::to_string_pretty(&trace).unwrap();
    println!("{}", json);
}
```

### With Input/Output Data

```rust
use sam_utils::trace::{ExecutionTrace, StepType};
use serde_json::json;

fn process_query(query: &str) -> String {
    let mut trace = ExecutionTrace::new();

    // Step 1: Retrieval with input data
    trace.start_step_with_data(
        StepType::Retrieval,
        "search",
        Some(json!({"query": query}))
    );
    let docs = vec!["doc1", "doc2"];
    trace.end_step_with_data(Some(json!({"count": docs.len()})));

    // Step 2: LLM generation
    trace.start_step(StepType::LlmGeneration, "generate");
    let response = "Generated response...";
    trace.end_step();

    // Export trace
    let trace_json = serde_json::to_string(&trace).unwrap();
    eprintln!("Trace: {}", trace_json);

    response.to_string()
}
```

### Performance Analysis

```rust
use sam_utils::trace::ExecutionTrace;

fn analyze_trace(trace: &ExecutionTrace) {
    println!("Performance Analysis");
    println!("===================");
    println!("Total duration: {}ms", trace.total_duration_ms());
    println!("Steps: {}", trace.steps().len());

    // Find slowest step
    if let Some(slowest) = trace.steps().iter().max_by_key(|s| s.duration_ms) {
        println!(
            "Slowest step: {} ({}ms)",
            slowest.name, slowest.duration_ms
        );
    }

    // Group by step type
    let mut by_type = std::collections::HashMap::new();
    for step in trace.steps() {
        *by_type.entry(step.step_type).or_insert(0) += step.duration_ms;
    }

    println!("\nTime by step type:");
    for (step_type, duration) in by_type {
        println!("  {:?}: {}ms", step_type, duration);
    }
}
```

## Step Types

The `StepType` enum defines common pipeline stages:

- `AudioCapture` - Audio input capture
- `VoiceActivity` - Voice activity detection
- `SpeechToText` - Speech-to-text transcription
- `Retrieval` - Knowledge/context retrieval
- `LlmGeneration` - LLM response generation
- `ToolExecution` - Tool/skill execution
- `TextToSpeech` - Text-to-speech synthesis
- `AudioPlayback` - Audio output playback
- `Error` - Error condition

## Use Cases

### 1. Debugging Complex Pipelines

Track execution flow through multi-stage AI pipelines:

```rust
// Voice assistant pipeline
trace.start_step(StepType::AudioCapture, "capture");
let audio = capture_audio();
trace.end_step();

trace.start_step(StepType::SpeechToText, "transcribe");
let text = transcribe(audio);
trace.end_step();

trace.start_step(StepType::Retrieval, "retrieve_context");
let context = retrieve_context(&text);
trace.end_step();

trace.start_step(StepType::LlmGeneration, "generate");
let response = llm.generate(&context);
trace.end_step();
```

### 2. Performance Profiling

Identify bottlenecks in your application:

```rust
for step in trace.steps() {
    if step.duration_ms > 1000 {
        eprintln!("SLOW: {} took {}ms", step.name, step.duration_ms);
    }
}
```

### 3. Transparency & Auditability

Export execution traces for review:

```json
{
  "steps": [
    {
      "step_type": "Retrieval",
      "name": "search_documents",
      "start_time_ms": 1703001234567,
      "duration_ms": 42,
      "input": {"query": "How do I use async Rust?"},
      "output": {"count": 3}
    },
    {
      "step_type": "LlmGeneration",
      "name": "generate_response",
      "start_time_ms": 1703001234609,
      "duration_ms": 1523
    }
  ]
}
```

### 4. Distributed Tracing

Pass traces between services for end-to-end visibility:

```rust
// Service A
let trace = execute_service_a();
let trace_json = serde_json::to_string(&trace)?;
send_to_service_b(trace_json);

// Service B
let mut trace: ExecutionTrace = serde_json::from_str(&trace_json)?;
trace.start_step(StepType::ToolExecution, "service_b_work");
// ... continue trace ...
```

## API Reference

### `ExecutionTrace`

Main trace container.

**Methods:**
- `new()` - Create new trace
- `start_step(step_type, name)` - Start a new step
- `start_step_with_data(step_type, name, input)` - Start with input data
- `end_step()` - End current step
- `end_step_with_data(output)` - End with output data
- `steps()` - Get all steps
- `total_duration_ms()` - Total execution time
- `current_step_mut()` - Get mutable reference to current step

### `TraceStep`

Individual step in execution trace.

**Fields:**
- `step_type: StepType` - Type of step
- `name: String` - Step description
- `start_time_ms: u64` - Unix timestamp (ms)
- `duration_ms: u64` - Duration in milliseconds
- `input: Option<Value>` - Input data (JSON)
- `output: Option<Value>` - Output data (JSON)
- `error: Option<String>` - Error message if failed

### `StepType`

Enum of common pipeline step types.

See [Step Types](#step-types) section above.

## Performance

- **Overhead**: ~1-2 microseconds per step start/end
- **Memory**: ~200 bytes per step
- **Serialization**: ~5-10ms for 100 steps to JSON

Minimal overhead suitable for production use.

## Compatibility

- **Rust Edition**: 2021
- **MSRV**: 1.70+
- **Platforms**: All (platform-independent)

## History

Extracted from the [Frame](https://github.com/Blackfall-Labs/sam) project, where it provides execution tracing for the AI assistant pipeline.

## License

MIT - See [LICENSE](LICENSE) for details.

## Author

Magnus Trent <magnus@blackfall.dev>

## Links

- **GitHub:** https://github.com/Blackfall-Labs/frame-trace-rs
- **Docs:** https://docs.rs/frame-trace
- **Crates.io:** https://crates.io/crates/frame-trace
- **SAM Project:** https://github.com/Blackfall-Labs/sam
