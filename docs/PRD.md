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

### FR-8: Generate Dialog Node

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-8.1 | Single input connector for stills | Must Have |
| FR-8.2 | Context text input for video description | Must Have |
| FR-8.3 | Generate button with progress indicator | Must Have |
| FR-8.4 | Configurable Ollama vision model connection | Must Have |
| FR-8.5 | Output: prolog, clips dialog, epilog, YouTube description | Must Have |
| FR-8.6 | Single output connector for generated text | Must Have |

### FR-9: Text View Node

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-9.1 | Single input connector for text | Must Have |
| FR-9.2 | Display prolog, clips with timestamps, epilog | Must Have |
| FR-9.3 | Display YouTube description | Must Have |
| FR-9.4 | Scrollable text view for long content | Should Have |

### FR-10: Extract Commentary Node (Whisper STT)

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-10.1 | Single input connector for video | Must Have |
| FR-10.2 | Extract audio from video | Must Have |
| FR-10.3 | Send audio to remote Whisper STT server | Must Have |
| FR-10.4 | Output: timestamped transcript text | Must Have |
| FR-10.5 | Processing status with progress indicator | Must Have |
| FR-10.6 | Queue management for remote server requests | Must Have |

### FR-11: Edit Dialog Node (LLM Text Editing)

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-11.1 | Single input connector for text | Must Have |
| FR-11.2 | Send text to remote LLM for editing/cleanup | Must Have |
| FR-11.3 | Configurable editing prompts/instructions | Should Have |
| FR-11.4 | Output: cleaned/edited text | Must Have |
| FR-11.5 | Prolog and epilog generation from context | Must Have |

### FR-12: Narration Node (Vibe Voice TTS)

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-12.1 | Single input connector for text | Must Have |
| FR-12.2 | Send text to remote Vibe Voice TTS server | Must Have |
| FR-12.3 | Use cloned voice profile | Must Have |
| FR-12.4 | Output: audio file (WAV/MP3) | Must Have |
| FR-12.5 | Progress indicator for TTS generation | Must Have |

### FR-13: Lip-Sync Node (Muse Talk)

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-13.1 | Two input connectors: avatar video, audio | Must Have |
| FR-13.2 | Send to remote Muse Talk server | Must Have |
| FR-13.3 | Stretch avatar video to match audio duration | Must Have |
| FR-13.4 | Output: lip-synced video | Must Have |
| FR-13.5 | Queue management for GPU resource allocation | Must Have |

### FR-14: Transparency Node (rembg)

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-14.1 | Single input connector for video | Must Have |
| FR-14.2 | Remove background using rembg | Must Have |
| FR-14.3 | Output: video with transparent background | Must Have |
| FR-14.4 | Frame-by-frame processing with progress | Must Have |

### FR-15: Composite Node (ffmpeg overlay)

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-15.1 | Two input connectors: base video, overlay video | Must Have |
| FR-15.2 | Position selection for overlay (default: lower-right) | Must Have |
| FR-15.3 | Size/scale controls for overlay | Should Have |
| FR-15.4 | Output: composited video clip | Must Have |
| FR-15.5 | Audio mixing options | Should Have |

### FR-16: Combine Node (Final Assembly)

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-16.1 | Multiple input connectors for video clips | Must Have |
| FR-16.2 | Order/sequence control | Must Have |
| FR-16.3 | Transition options between clips | Could Have |
| FR-16.4 | Output: final assembled video | Must Have |
| FR-16.5 | Include prolog/epilog clips | Must Have |

### FR-17: Project Metadata

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-17.1 | Title graphics configuration | Should Have |
| FR-17.2 | Overlay graphics (watermark, branding) | Should Have |
| FR-17.3 | Outro graphics | Should Have |
| FR-17.4 | Project links (GitHub, website, etc.) | Should Have |
| FR-17.5 | Export metadata as JSON | Should Have |

### FR-18: Remote Server Queue System

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-18.1 | Request queue per remote server type | Must Have |
| FR-18.2 | Prevent GPU VRAM overload | Must Have |
| FR-18.3 | Priority-based scheduling | Should Have |
| FR-18.4 | Multi-project support (parallel video processing) | Must Have |
| FR-18.5 | Resource utilization dashboard | Could Have |
| FR-18.6 | Configurable concurrency limits per server | Must Have |

## Parallel Processing Architecture

The system is designed to maximize throughput by running operations in parallel where dependencies allow:

```
VideoInput ──┬── StillSampler ──── GenerateDialog ──── EditDialog ──┬── Narration ──── LipSync ──┐
             │                                                       │                            │
             └── ExtractCommentary ─────────────────────────────────┘                            │
                                                                                                  │
             ┌────────────────────────────────────────────────────────────────────────────────────┘
             │
             └── Transparency ──── Composite ──── Combine ──── FinalVideo
```

**Parallel Operations:**
- STT (Whisper) and Vision analysis can run simultaneously
- Multiple TTS generations can queue and run in parallel
- Background removal can process while lip-sync runs
- Compositing begins as soon as clips are ready

**Queue Management:**
- Each remote server (Whisper, Ollama, Vibe Voice, Muse Talk) has its own queue
- Configurable concurrency limits prevent VRAM exhaustion
- Multiple projects can share the pipeline, maximizing GPU utilization

## Out of Scope (v1.0)

- Real-time video preview (live streaming)
- Collaboration features (multi-user editing)
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
