# Code Metrics & Design Patterns Guide

Best practices for modular, compact, testable Rust code. Follow these patterns when implementing features and fixing sw-checklist errors.

## Component Architecture

**Key Principle**: The metrics limit LOC, functions, modules, and crates - but do NOT limit components.

Organize the project into top-level components, each with its own crates structure:

```
./components/
├── macros/                    # Derive macros, proc macros
│   └── crates/macros-core/src/lib.rs
├── integration-tests/         # E2E and integration tests
│   └── crates/e2e-tests/src/lib.rs
├── database/                  # Data persistence layer
│   └── crates/db-core/src/lib.rs
├── rest-server/               # HTTP/REST API
│   └── crates/rest-api/src/lib.rs
├── crud/                      # CRUD operations
│   └── crates/crud-ops/src/lib.rs
├── ui-nodes/                  # Node rendering components
│   └── crates/node-render/src/lib.rs
├── ui-editor/                 # Canvas/editor UI
│   └── crates/canvas-editor/src/lib.rs
└── shared/                    # Shared types across components
    └── crates/shared-types/src/lib.rs
```

**Strict Limits** (leave headroom for future work):
- **Target 3, max 4** crates per component
- **Target 3, max 4** modules per crate
- **Target 3, max 4** functions per module/file
- Files stay small (<350 LOC, prefer <100)
- No limit on number of components

**Benefits**:
- Sparse structure leaves room for growth
- Components are independently testable and deployable
- Easy to navigate and understand
- Forces clean separation of concerns

**When to Create a New Component**:
- Distinct domain responsibility (e.g., "video processing", "authentication")
- Different deployment target (e.g., WASM vs server)
- Separable test suite
- Different dependency graph

## Module Organization

### No Functions in lib.rs or mod.rs

**Strict Rule**: lib.rs and mod.rs files should ONLY contain:
- Module declarations (`mod foo;`, `pub mod bar;`)
- Re-exports (`pub use foo::Bar;`)
- Type aliases
- Module-level documentation

Move all functions, impls, and structs to named modules.

```rust
// BAD: lib.rs with functions
pub fn helper() { ... }
pub mod types;

// GOOD: lib.rs re-exports only
pub mod types;
pub mod helpers;
pub use types::*;
```

### Function Count Per Module (max 7)

Split modules by responsibility when approaching the limit:

```
// Before: one large module
canvas.rs (11 functions)

// After: directory with focused submodules
canvas/
  mod.rs        (5 functions - component & rendering)
  callbacks.rs  (5 functions - event handling)
  connections.rs (4 functions - connection rendering)
```

### Struct Definition Separation

Separate struct definitions from implementations:

```
// models/node.rs - struct definition only
pub struct Node { ... }

// models/node_read.rs - immutable impl methods
impl Node {
    pub fn type_name(&self) -> &'static str { ... }
    pub fn get_connector(&self, id: Uuid) -> Option<&Connector> { ... }
}

// models/node_write.rs - mutable impl methods
impl Node {
    pub fn set_position(&mut self, pos: Position) { ... }
    pub fn add_connector(&mut self, conn: Connector) { ... }
}
```

## Design Patterns

### Builder Pattern

Separate initialization from execution:

```rust
// Config builder for complex initialization
pub struct ServerConfigBuilder {
    port: Option<u16>,
    data_dir: Option<PathBuf>,
}

impl ServerConfigBuilder {
    pub fn new() -> Self { Self { port: None, data_dir: None } }
    pub fn port(mut self, port: u16) -> Self { self.port = Some(port); self }
    pub fn data_dir(mut self, dir: PathBuf) -> Self { self.data_dir = Some(dir); self }
    pub fn build(self) -> Result<ServerConfig, ConfigError> { ... }
}

// Usage: ServerConfigBuilder::new().port(3000).build()?
```

### Pure Functions

Prefer pure functions that take state as input:

```rust
// Bad: modifies external state
fn process_node(nodes: &mut HashMap<Uuid, Node>) { ... }

// Good: pure function, returns new state
fn process_node(nodes: &HashMap<Uuid, Node>) -> HashMap<Uuid, Node> { ... }

// Good: takes state reference, returns derived value
fn calculate_total(state: &AppState) -> f64 { ... }
```

### Separate Readers from Writers

Functions that read state vs modify state:

```rust
// Read-only functions (easier to test, no side effects)
impl State {
    pub fn node_count(&self) -> usize { self.nodes.len() }
    pub fn find_node(&self, id: Uuid) -> Option<&Node> { self.nodes.get(&id) }
    pub fn calculate_bounds(&self) -> Rect { ... }
}

// Write functions (clearly marked as mutating)
impl State {
    pub fn add_node(&mut self, node: Node) { self.nodes.insert(node.id, node); }
    pub fn remove_node(&mut self, id: Uuid) -> Option<Node> { self.nodes.remove(&id) }
}
```

### Config Object Pattern

Pass configuration through a single object:

```rust
pub struct AppConfig {
    pub port: u16,
    pub data_dir: PathBuf,
    pub debug_mode: bool,
    pub max_upload_size: usize,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> { ... }
    pub fn from_args(args: &Args) -> Self { ... }
}

// Functions receive config, not individual params
fn start_server(config: &AppConfig) -> Result<(), Error> { ... }
```

### Chain of Responsibility

Replace switch/if-else with handler chain:

```rust
trait ActionHandler {
    fn handle(&self, state: &mut AppState, action: &AppAction) -> bool;
}

struct CanvasHandler;
impl ActionHandler for CanvasHandler {
    fn handle(&self, state: &mut AppState, action: &AppAction) -> bool {
        match action {
            AppAction::SetPan(pos) => { state.canvas.pan = *pos; true }
            AppAction::SetZoom(z) => { state.canvas.zoom = *z; true }
            _ => false  // Not handled, pass to next
        }
    }
}

struct NodeHandler;
impl ActionHandler for NodeHandler { ... }

// Dispatch through chain
fn dispatch(handlers: &[Box<dyn ActionHandler>], state: &mut AppState, action: AppAction) {
    for handler in handlers {
        if handler.handle(state, &action) { return; }
    }
}
```

### Facade Pattern

Hide complex subsystems behind simple interface:

```rust
// Complex internal modules
mod video_decoder;
mod frame_extractor;
mod thumbnail_generator;

// Simple facade
pub struct VideoProcessor { ... }

impl VideoProcessor {
    pub fn extract_stills(&self, path: &Path, interval: Duration) -> Vec<Still> {
        // Internally uses decoder, extractor, generator
    }
}
```

### Adapter Pattern

Convert interfaces between systems:

```rust
// External API returns different format
struct ExternalVideoInfo { duration_ms: u64, ... }

// Our internal format
struct VideoMetadata { duration: Duration, ... }

// Adapter converts between them
impl From<ExternalVideoInfo> for VideoMetadata {
    fn from(ext: ExternalVideoInfo) -> Self {
        Self { duration: Duration::from_millis(ext.duration_ms), ... }
    }
}
```

## Rust Macros for Boilerplate

### Derive Macros

Use derive macros to reduce repetitive code:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Position { pub x: f64, pub y: f64 }
```

### Custom Macros for Patterns

```rust
// Macro for repetitive callback creation
macro_rules! create_callback {
    ($state:expr, |$e:ident| $body:expr) => {
        {
            let state = $state.clone();
            Callback::from(move |$e| { $body })
        }
    };
}

// Usage
let on_click = create_callback!(state, |e| state.dispatch(AppAction::Click(e)));
```

## Testing Strategy

### Pure Functions Are Easier to Test

```rust
// Easy to test - no setup needed
#[test]
fn test_calculate_bounds() {
    let nodes = vec![node_at(0, 0), node_at(100, 100)];
    assert_eq!(calculate_bounds(&nodes), Rect::new(0, 0, 100, 100));
}
```

### Test Modules in Separate Files

Move tests to integration tests when module function count is exceeded:

```
crates/shared/
  src/models/node.rs      # Struct + impl (max 7 functions)
  tests/node_tests.rs     # All tests for node
```

## Loose Coupling

### Dependency Injection

```rust
// Bad: hard-coded dependency
impl Server {
    fn new() -> Self {
        let store = FileStore::new();  // Tightly coupled
        Self { store }
    }
}

// Good: inject dependency
impl Server {
    fn new(store: Box<dyn DataStore>) -> Self {
        Self { store }
    }
}
```

### Event-Driven Communication

```rust
// Components communicate through events, not direct calls
enum AppEvent {
    NodeCreated(Uuid),
    NodeDeleted(Uuid),
    ConnectionMade { from: Uuid, to: Uuid },
}

// Subscribers react to events independently
fn on_event(event: &AppEvent, state: &mut AppState) {
    match event { ... }
}
```

## Checklist for New Code

- [ ] Functions < 50 lines (< 25 preferred)
- [ ] Modules < 7 functions (split if needed)
- [ ] Pure functions where possible
- [ ] Readers separate from writers
- [ ] Config object for multiple params
- [ ] Derive macros for common traits
- [ ] Tests in separate files if module full
- [ ] Dependencies injected, not hard-coded
