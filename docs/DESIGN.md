# Design Document

## Visual Design

### Color Palette

```
Background:     #1e1e2e (dark canvas)
Grid:           #2a2a3e (subtle grid lines)
Node Background:#2d2d3d (card background)
Node Border:    #3d3d4d (default) / #6c6cff (selected)
Connector:      #4a9eff (output) / #ff6b6b (input)
Connection:     #4a9eff (line) / #6c6cff (hover)
Text Primary:   #e0e0e0
Text Secondary: #a0a0a0
Accent:         #6c6cff (buttons, highlights)
Success:        #4ade80
Warning:        #fbbf24
Error:          #f87171
```

### Typography

```
Font Family:    Inter, system-ui, sans-serif
Node Title:     14px, semi-bold
Node Content:   12px, regular
Connector Label:10px, regular
Toolbox:        13px, medium
```

### Node Design

```
┌─────────────────────────────────────┐
│ ◉ Video Input                    ✕ │  <- Header (drag handle, title, delete)
├─────────────────────────────────────┤
│                                     │
│     [Browse Video...]               │  <- Content area
│                                     │
│  or                                 │
│                                     │
│     my_video.mp4                    │
│     Duration: 5:32                  │
│                                     │
├─────────────────────────────────────┤
│                              [●]────│  <- Output connector
└─────────────────────────────────────┘

Width: 200px (min)
Header Height: 32px
Border Radius: 8px
Shadow: 0 4px 6px rgba(0,0,0,0.3)
```

### Still Sampler Node

```
┌─────────────────────────────────────┐
│ ◉ Still Sampler                  ✕ │
├─────────────────────────────────────┤
│────[●]  Input                       │  <- Input connector
│                                     │
│  Interval: [30    ▼] seconds        │  <- Dropdown/spinner
│                                     │
│  ┌─────┐ ┌─────┐ ┌─────┐           │
│  │ 0:00│ │ 0:30│ │ 1:00│           │  <- Thumbnails
│  └─────┘ └─────┘ └─────┘           │
│     │       │       │               │
│  ┌─────┐ ┌─────┐ ┌─────┐           │
│  │ 1:30│ │ 2:00│ │ ... │           │
│  └─────┘ └─────┘ └─────┘           │
│                                     │
├─────────────────────────────────────┤
│                         [●]─ 0:00   │  <- Output connectors
│                         [●]─ 0:30   │     (one per still)
│                         [●]─ 1:00   │
│                         [●]─ ...    │
└─────────────────────────────────────┘

Height: Dynamic (based on stills count)
```

### Connector Design

```
Input Connector:          Output Connector:
     ┌───┐                     ┌───┐
 ────│ ● │                     │ ● │────
     └───┘                     └───┘

Size: 12px diameter
Hover: 14px diameter + glow
Active (during connection): pulsing animation
```

### Connection (Bezier Curve)

```
     ●────────╮
              │
              ╰──────────●

Start Point: Output connector center
End Point: Input connector center
Control Points: Automatically calculated, user-adjustable

Default control point calculation:
  CP1 = start + (dx * 0.5, 0)
  CP2 = end - (dx * 0.5, 0)

Where dx = end.x - start.x
```

### Toolbox Design

```
┌─────────────────────┐
│ ◀ Nodes             │  <- Collapse toggle
├─────────────────────┤
│                     │
│  ┌───────────────┐  │
│  │ 🎬            │  │  <- Draggable item
│  │ Video Input   │  │
│  │ Load a video  │  │
│  └───────────────┘  │
│                     │
│  ┌───────────────┐  │
│  │ 🖼️            │  │
│  │ Still Sampler │  │
│  │ Extract frames│  │
│  └───────────────┘  │
│                     │
└─────────────────────┘

Width: 220px (expanded) / 48px (collapsed)
Transition: 200ms ease
```

## Component Specifications

### Workspace Component

**Responsibilities:**
- Apply pan/zoom transforms to canvas
- Handle wheel events for zoom
- Handle middle-mouse drag for pan
- Calculate world coordinates from screen coordinates

**Props:**
```rust
pub struct WorkspaceProps {
    pub children: Children,
}
```

**State:**
```rust
pub struct WorkspaceState {
    pub zoom: f64,          // 0.25 to 4.0
    pub pan: Position,      // World offset
    pub is_panning: bool,
}
```

### Canvas Component

**Responsibilities:**
- Render SVG container
- Handle drop events for node creation
- Manage canvas coordinate system

**Events:**
- `ondrop` - Create node at drop position
- `ondragover` - Allow drop
- `onclick` - Deselect all

### BaseNode Component

**Props:**
```rust
pub struct BaseNodeProps {
    pub node: Node,
    pub selected: bool,
    pub on_move: Callback<(Uuid, Position)>,
    pub on_delete: Callback<Uuid>,
    pub on_select: Callback<Uuid>,
}
```

**Interaction:**
- Drag header to move
- Click to select
- Click delete button to remove

### Connector Component

**Props:**
```rust
pub struct ConnectorProps {
    pub connector: Connector,
    pub node_id: Uuid,
    pub position: Position,  // Absolute position
    pub is_input: bool,
    pub is_connected: bool,
    pub is_valid_target: bool,
    pub on_start_connection: Callback<(Uuid, Uuid)>,
    pub on_complete_connection: Callback<(Uuid, Uuid)>,
}
```

### BezierConnection Component

**Props:**
```rust
pub struct BezierConnectionProps {
    pub connection: Connection,
    pub start: Position,
    pub end: Position,
    pub selected: bool,
    pub on_select: Callback<Uuid>,
    pub on_control_point_change: Callback<(Uuid, BezierControlPoints)>,
}
```

**SVG Path:**
```rust
fn bezier_path(start: Position, end: Position, cp: Option<BezierControlPoints>) -> String {
    let (cp1, cp2) = cp.unwrap_or_else(|| calculate_default_control_points(start, end));
    format!(
        "M {},{} C {},{} {},{} {},{}",
        start.x, start.y,
        cp1.x, cp1.y,
        cp2.x, cp2.y,
        end.x, end.y
    )
}
```

## Interaction Flows

### Drag Node from Toolbox

```
1. mousedown on toolbox item
2. Create drag preview (ghost node)
3. mousemove updates preview position
4. Enter canvas: show drop indicator
5. mouseup on canvas:
   - Calculate canvas coordinates
   - Dispatch CreateNode action
   - API POST /nodes
6. Cancel: mouseup outside canvas or Escape key
```

### Create Connection

```
1. mousedown on output connector
2. Create PendingConnection in state
3. Render temporary bezier from connector to mouse
4. mousemove updates pending end position
5. Hover over input connector:
   - Validate compatibility
   - Highlight if valid
6. mouseup on valid input:
   - Dispatch CompleteConnection
   - API POST /connections
7. Cancel: mouseup elsewhere or Escape key
```

### Move Node

```
1. mousedown on node header
2. Store initial position and mouse offset
3. mousemove:
   - Calculate new position
   - Update node position (local state)
   - Debounce API updates
4. mouseup:
   - Final position update
   - API PATCH /nodes/:id/position
```

### Adjust Bezier Handle

```
1. mousedown on control handle
2. Store which handle (CP1 or CP2)
3. mousemove:
   - Update control point position
   - Re-render bezier path
4. mouseup:
   - Save control points
   - API PUT /connections/:id
```

## API Design

### Endpoints

```
GET    /api/v1/health                      Health check

GET    /api/v1/projects                    List projects
POST   /api/v1/projects                    Create project
GET    /api/v1/projects/:id                Get project
PUT    /api/v1/projects/:id                Update project
DELETE /api/v1/projects/:id                Delete project

GET    /api/v1/projects/:id/nodes          List nodes
POST   /api/v1/projects/:id/nodes          Create node
GET    /api/v1/projects/:id/nodes/:nid     Get node
PUT    /api/v1/projects/:id/nodes/:nid     Update node
DELETE /api/v1/projects/:id/nodes/:nid     Delete node
PATCH  /api/v1/projects/:id/nodes/:nid/position  Move node

GET    /api/v1/projects/:id/connections    List connections
POST   /api/v1/projects/:id/connections    Create connection
DELETE /api/v1/projects/:id/connections/:cid  Delete connection
PUT    /api/v1/projects/:id/connections/:cid  Update control points

POST   /api/v1/files/upload                Upload file
GET    /api/v1/files/:fid                  Get file
GET    /api/v1/files/:fid/info             Get file metadata
DELETE /api/v1/files/:fid                  Delete file

POST   /api/v1/processing/extract-stills   Start extraction job
GET    /api/v1/processing/:jid/status      Get job status
GET    /api/v1/stills/:sid/thumbnail       Get thumbnail
```

### Request/Response Examples

**Create Node:**
```json
// POST /api/v1/projects/:id/nodes
// Request
{
  "type": "VideoInput",
  "position": { "x": 100, "y": 200 }
}

// Response 201
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "position": { "x": 100, "y": 200 },
  "size": { "width": 200, "height": 150 },
  "data": {
    "type": "VideoInput",
    "file_id": null,
    "file_name": null,
    "duration_seconds": null,
    "upload_status": "None"
  },
  "inputs": [],
  "outputs": [{
    "id": "660e8400-e29b-41d4-a716-446655440001",
    "name": "video_out",
    "connector_type": "Output",
    "position": { "Right": 75 }
  }],
  "z_index": 1
}
```

**Create Connection:**
```json
// POST /api/v1/projects/:id/connections
// Request
{
  "from_node": "550e8400-e29b-41d4-a716-446655440000",
  "from_connector": "660e8400-e29b-41d4-a716-446655440001",
  "to_node": "770e8400-e29b-41d4-a716-446655440000",
  "to_connector": "880e8400-e29b-41d4-a716-446655440001"
}

// Response 201
{
  "id": "990e8400-e29b-41d4-a716-446655440000",
  "from_node": "...",
  "from_connector": "...",
  "to_node": "...",
  "to_connector": "...",
  "control_points": null
}
```

**Extract Stills:**
```json
// POST /api/v1/processing/extract-stills
// Request
{
  "file_id": "550e8400-e29b-41d4-a716-446655440000",
  "interval_seconds": 30
}

// Response 202
{
  "job_id": "aa0e8400-e29b-41d4-a716-446655440000",
  "status": "Processing",
  "progress": 0
}

// GET /api/v1/processing/:jid/status (polling)
// Response 200
{
  "job_id": "aa0e8400-e29b-41d4-a716-446655440000",
  "status": "Complete",
  "progress": 100,
  "stills": [
    { "id": "...", "timestamp_seconds": 0, "thumbnail_url": "/api/v1/stills/.../thumbnail" },
    { "id": "...", "timestamp_seconds": 30, "thumbnail_url": "..." },
    // ...
  ]
}
```

## Error Handling

### Frontend Errors

| Error | Display | Recovery |
|-------|---------|----------|
| Network failure | Toast notification | Retry button, offline indicator |
| Invalid drop | Shake animation | Allow retry |
| Upload failure | Error in node | Delete and retry |
| Processing failure | Error in node | Retry or delete |

### Backend Errors

| Code | Meaning | Response |
|------|---------|----------|
| 400 | Invalid request | `{ "error": "message" }` |
| 404 | Not found | `{ "error": "Resource not found" }` |
| 409 | Conflict | `{ "error": "Connection already exists" }` |
| 413 | File too large | `{ "error": "File exceeds limit" }` |
| 422 | Invalid connection | `{ "error": "Incompatible connector types" }` |
| 500 | Server error | `{ "error": "Internal error" }` |
