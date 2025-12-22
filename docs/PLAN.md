# Implementation Plan

## Phase 0: Documentation
- [x] Create docs folder
- [x] ARCHITECTURE.md
- [x] PRD.md
- [x] DESIGN.md
- [x] PLAN.md (this file)
- [x] STATUS.md

## Phase 1: Foundation

### 1.1 Workspace Setup
- [ ] Convert to Cargo workspace
- [ ] Create `crates/shared` crate
- [ ] Create `crates/backend` crate
- [ ] Create `crates/frontend` crate
- [ ] Configure workspace dependencies

### 1.2 Shared Models
- [ ] `Position`, `Size` structs
- [ ] `Connector`, `ConnectorType` structs
- [ ] `Node`, `NodeData` enum (VideoInput, StillSampler)
- [ ] `Connection`, `BezierControlPoints` structs
- [ ] `CanvasState`, `Viewport` structs
- [ ] `Project` struct
- [ ] API request/response DTOs

### 1.3 Backend Skeleton
- [ ] Axum app setup
- [ ] CLI argument parsing (clap)
- [ ] Server configuration struct
- [ ] Static file serving (tower-http)
- [ ] CORS configuration
- [ ] Health check endpoint
- [ ] Error types

### 1.4 Frontend Skeleton
- [ ] Yew app entry point (`main.rs`)
- [ ] Trunk configuration (`index.html`)
- [ ] Root `App` component
- [ ] Basic CSS/SCSS setup
- [ ] API client service (gloo-net)

## Phase 2: Canvas Infrastructure

### 2.1 Canvas Component
- [ ] SVG container component
- [ ] ViewBox management
- [ ] Coordinate transform utilities
- [ ] Screen-to-canvas coordinate conversion
- [ ] Canvas-to-screen coordinate conversion

### 2.2 Workspace (Pan/Zoom)
- [ ] Zoom state (wheel events)
- [ ] Pan state (middle-mouse drag)
- [ ] Transform group rendering
- [ ] Zoom limits (0.25x to 4x)
- [ ] Keyboard shortcuts (reset zoom)

### 2.3 Viewport & Scrollbars
- [ ] Viewport bounds calculation
- [ ] Virtual scrollbar components
- [ ] Scroll position sync
- [ ] Content bounds from nodes

### 2.4 Background Grid
- [ ] SVG pattern definition
- [ ] Grid scaling with zoom
- [ ] Grid offset with pan

## Phase 3: Node System

### 3.1 State Management
- [ ] `AppState` struct
- [ ] `AppAction` enum
- [ ] Reducer function
- [ ] Context provider setup

### 3.2 Base Node Component
- [ ] Node container (foreignObject in SVG)
- [ ] Drag handle (header)
- [ ] Position updates during drag
- [ ] Selection state
- [ ] Delete button
- [ ] Z-index management

### 3.3 Connector Components
- [ ] Output connector (right side)
- [ ] Input connector (left side)
- [ ] Hover states
- [ ] Click handler (start connection)
- [ ] Drop target (complete connection)

### 3.4 Video Input Node
- [ ] Empty state (upload button)
- [ ] File loaded state (filename display)
- [ ] File input element
- [ ] Upload progress bar
- [ ] Delete file button
- [ ] Output connector

### 3.5 Toolbox
- [ ] Sidebar component
- [ ] Collapse/expand toggle
- [ ] Node palette items
- [ ] Drag preview (ghost)
- [ ] Drop zone on canvas

## Phase 4: Connection System

### 4.1 Bezier Curve Rendering
- [ ] SVG path component
- [ ] Cubic bezier path string
- [ ] Default control point calculation
- [ ] Line styling (stroke, color)
- [ ] Hover state

### 4.2 Connection Interaction
- [ ] `PendingConnection` state
- [ ] Start connection (click output)
- [ ] Update pending (mouse move)
- [ ] Complete connection (drop on input)
- [ ] Cancel connection (escape/click away)
- [ ] Validate connections (type checking)

### 4.3 Bezier Handles
- [ ] Control point visualization
- [ ] Drag handles to adjust
- [ ] Update curve in real-time
- [ ] Persist control points

### 4.4 Connection Management
- [ ] Delete connection (click + delete)
- [ ] Selection state
- [ ] Re-route when nodes move

## Phase 5: Backend API

### 5.1 In-Memory Storage
- [ ] `AppState` with RwLock
- [ ] Projects HashMap
- [ ] Files HashMap
- [ ] Processing jobs HashMap

### 5.2 Project Endpoints
- [ ] GET /projects
- [ ] POST /projects
- [ ] GET /projects/:id
- [ ] PUT /projects/:id
- [ ] DELETE /projects/:id

### 5.3 Node Endpoints
- [ ] GET /projects/:id/nodes
- [ ] POST /projects/:id/nodes
- [ ] GET /projects/:id/nodes/:nid
- [ ] PUT /projects/:id/nodes/:nid
- [ ] DELETE /projects/:id/nodes/:nid
- [ ] PATCH /projects/:id/nodes/:nid/position

### 5.4 Connection Endpoints
- [ ] GET /projects/:id/connections
- [ ] POST /projects/:id/connections
- [ ] DELETE /projects/:id/connections/:cid
- [ ] PUT /projects/:id/connections/:cid

### 5.5 File Handling
- [ ] POST /files/upload (multipart)
- [ ] GET /files/:fid (stream)
- [ ] GET /files/:fid/info
- [ ] DELETE /files/:fid
- [ ] Storage service

### 5.6 Video Processing
- [ ] ffmpeg-sidecar setup
- [ ] POST /processing/extract-stills
- [ ] Background job execution
- [ ] GET /processing/:jid/status
- [ ] GET /stills/:sid/thumbnail

## Phase 6: Still Sampler Node

### 6.1 Node UI
- [ ] Input connector
- [ ] Interval input widget
- [ ] Dropdown/spinner component
- [ ] Idle state display

### 6.2 Processing Integration
- [ ] Trigger extraction on connection
- [ ] Poll job status
- [ ] Progress indicator
- [ ] Error state handling

### 6.3 Dynamic Outputs
- [ ] Generate output connectors from stills
- [ ] Thumbnail grid display
- [ ] Connector labels (timestamps)
- [ ] Node height expansion

## Phase 7: Polish & Integration

### 7.1 Error Handling
- [ ] Toast notification system
- [ ] Network error recovery
- [ ] Offline indicator
- [ ] Retry mechanisms

### 7.2 Persistence
- [ ] Auto-save (debounced)
- [ ] Load project on startup
- [ ] File-based project storage
- [ ] Undo/redo (optional)

### 7.3 Performance
- [ ] Virtual rendering for large canvases
- [ ] Debounced API calls
- [ ] Optimized re-renders
- [ ] Connection layer optimization

### 7.4 Build & Distribution
- [ ] Production Trunk build
- [ ] Embed WASM in backend (memory-serve)
- [ ] Single binary build
- [ ] Release configuration

## Milestones

| Milestone | Target | Deliverable |
|-----------|--------|-------------|
| M1 | Phase 1 complete | Project compiles, serves empty page |
| M2 | Phase 2 complete | Infinite canvas with pan/zoom |
| M3 | Phase 3 complete | Nodes visible, draggable, toolbox works |
| M4 | Phase 4 complete | Connections working with bezier curves |
| M5 | Phase 5 complete | Full REST API, file uploads working |
| M6 | Phase 6 complete | Still extraction end-to-end |
| M7 | Phase 7 complete | Production-ready release |

## Technical Debt Tracking

| Item | Priority | Notes |
|------|----------|-------|
| - | - | (None yet) |

## Open Questions

1. **Node ID generation**: UUID v4 on client or server?
   - Decision: Server generates IDs for consistency

2. **State sync strategy**: Optimistic updates or wait for server?
   - Decision: Optimistic for position, wait for create/delete

3. **File storage location**: Configurable via CLI?
   - Decision: Yes, `--data-dir` argument

4. **Max file size**: Limit for uploads?
   - Decision: 1GB default, configurable

5. **Project persistence**: JSON files or SQLite?
   - Decision: JSON files initially, simpler
