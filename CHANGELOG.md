# Changelog

## [0.1.0] - 2025-12-21

### Added
- Initial release extracted from Frame project
- **Execution Tracing**: CallGraph tracking for debugging and performance analysis
- `ExecutionTrace` struct for managing pipeline traces
- `StepType` enum for common pipeline stages
- `TraceStep` struct for individual execution steps
- JSON serialization support via serde
- Performance metrics and analysis capabilities

### Features
- Track execution flow through multi-stage pipelines
- Measure duration of each step with microsecond precision
- Attach input/output data to steps for debugging
- Export traces as JSON for analysis
- Minimal overhead (~1-2μs per step)

### Dependencies
- `serde` (serialization)
- `serde_json` (JSON export)

### Notes
- Extracted from [Frame](https://github.com/Blackfall-Labs/sam)
- Platform-independent, minimal dependencies
- Production-ready with minimal overhead
