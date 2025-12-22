# Product Requirements Document (PRD)

## Product Overview

**Product Name:** yt-rs Node Editor

**Purpose:** A visual node-based editor for video processing workflows, enabling users to create pipelines for extracting and processing video content through an intuitive drag-and-drop interface.

**Target Users:** Content creators, video editors, and developers who need to process videos and extract frames/stills programmatically.

## Problem Statement

Users need a way to visually design video processing workflows without writing code. Current solutions are either:
- Command-line tools (ffmpeg) requiring technical expertise
- Heavy desktop applications with steep learning curves
- Cloud services with privacy/cost concerns

## Solution

A lightweight, web-based node editor that:
- Runs locally (privacy-preserving)
- Uses visual programming paradigm (accessible)
- Built entirely in Rust (performant, safe)
- Single binary deployment (easy installation)

## Functional Requirements

### FR-1: Canvas Workspace

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-1.1 | Infinite canvas with pan (drag) and zoom (scroll wheel) | Must Have |
| FR-1.2 | Horizontal and vertical scrollbars | Must Have |
| FR-1.3 | Background grid that scales with zoom | Should Have |
| FR-1.4 | Minimap for navigation (large projects) | Could Have |

### FR-2: Toolbox

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-2.1 | Collapsible sidebar containing node types | Must Have |
| FR-2.2 | Drag nodes from toolbox to canvas | Must Have |
| FR-2.3 | Node type icons and descriptions | Should Have |
| FR-2.4 | Search/filter nodes | Could Have |

### FR-3: Video Input Node

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-3.1 | Display video filename when loaded | Must Have |
| FR-3.2 | Browse/upload button when empty | Must Have |
| FR-3.3 | Delete button to remove video | Must Have |
| FR-3.4 | Single output connector | Must Have |
| FR-3.5 | Upload progress indicator | Should Have |
| FR-3.6 | Video thumbnail preview | Could Have |
| FR-3.7 | Video duration display | Could Have |

### FR-4: Still Sampler Node

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-4.1 | Single input connector | Must Have |
| FR-4.2 | Time interval field (default: 30 seconds) | Must Have |
| FR-4.3 | Dynamic output connectors (one per extracted still) | Must Have |
| FR-4.4 | Interval input as dropdown/spinner/numeric entry | Must Have |
| FR-4.5 | Processing status indicator | Should Have |
| FR-4.6 | Still thumbnail previews | Should Have |
| FR-4.7 | Manual timestamp entry for specific frames | Could Have |

### FR-5: Connections

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-5.1 | Click output connector to start connection | Must Have |
| FR-5.2 | Bezier curve follows mouse during drag | Must Have |
| FR-5.3 | Drop on input connector to complete | Must Have |
| FR-5.4 | Visual feedback on valid drop targets | Must Have |
| FR-5.5 | Bezier curve handles for shape adjustment | Should Have |
| FR-5.6 | Connection routing around nodes | Could Have |
| FR-5.7 | Delete connection (click + delete key or context menu) | Must Have |

### FR-6: Node Interaction

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-6.1 | Drag nodes to reposition | Must Have |
| FR-6.2 | Select nodes (click) | Must Have |
| FR-6.3 | Delete nodes (delete key or button) | Must Have |
| FR-6.4 | Multi-select (shift+click or marquee) | Could Have |
| FR-6.5 | Copy/paste nodes | Could Have |

### FR-7: Backend Server

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-7.1 | CLI with configurable port and data directory | Must Have |
| FR-7.2 | Serve static WASM frontend files | Must Have |
| FR-7.3 | REST API for CRUD operations | Must Have |
| FR-7.4 | File upload endpoint | Must Have |
| FR-7.5 | Video processing (still extraction via ffmpeg) | Must Have |
| FR-7.6 | Project save/load | Should Have |

## Non-Functional Requirements

### NFR-1: Performance

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-1.1 | Initial page load | < 3 seconds |
| NFR-1.2 | Node drag responsiveness | 60 FPS |
| NFR-1.3 | Support nodes on canvas | 100+ nodes |
| NFR-1.4 | File upload (100MB video) | < 30 seconds |

### NFR-2: Usability

| ID | Requirement |
|----|-------------|
| NFR-2.1 | No JavaScript knowledge required for end users |
| NFR-2.2 | Intuitive drag-and-drop interaction |
| NFR-2.3 | Clear visual feedback for all actions |
| NFR-2.4 | Keyboard shortcuts for common actions |

### NFR-3: Compatibility

| ID | Requirement |
|----|-------------|
| NFR-3.1 | Chrome 90+, Firefox 90+, Safari 14+, Edge 90+ |
| NFR-3.2 | Desktop screen sizes (1280x720 minimum) |
| NFR-3.3 | macOS, Linux, Windows server support |

### NFR-4: Reliability

| ID | Requirement |
|----|-------------|
| NFR-4.1 | Graceful handling of server disconnection |
| NFR-4.2 | Auto-save project state periodically |
| NFR-4.3 | Recovery from browser refresh |

## User Stories

### US-1: First-time User
```
As a new user,
I want to see an empty canvas with a visible toolbox,
So that I understand how to start building a workflow.
```

### US-2: Adding a Video
```
As a user,
I want to drag a Video Input node onto the canvas and upload a video,
So that I can use that video as a source in my workflow.
```

### US-3: Extracting Stills
```
As a user,
I want to connect a Video Input to a Still Sampler and configure the interval,
So that I can extract frames at regular intervals from my video.
```

### US-4: Adjusting Connections
```
As a user,
I want to drag the bezier curve handles,
So that I can route connections around nodes for clarity.
```

### US-5: Modifying Workflow
```
As a user,
I want to delete nodes and connections,
So that I can iterate on my workflow design.
```

## Out of Scope (v1.0)

- Real-time video preview
- Audio extraction
- Video output/encoding
- Collaboration features
- Cloud storage integration
- Mobile/tablet support
- Node scripting/custom nodes

## Success Metrics

| Metric | Target |
|--------|--------|
| Single binary size | < 50 MB |
| Memory usage (idle) | < 100 MB |
| Time to extract 10 stills from 5-min video | < 30 seconds |
| Lines of JavaScript | 0 (all Rust/WASM) |

## Dependencies

- FFmpeg installed on server system
- Modern web browser with WASM support
- Rust toolchain for building

## Glossary

| Term | Definition |
|------|------------|
| Node | A visual block representing a processing step |
| Connector | Input or output point on a node for connections |
| Connection | A link between an output and input connector |
| Canvas | The infinite workspace where nodes are placed |
| Viewport | The visible portion of the canvas |
| Still | A single frame extracted from a video |
