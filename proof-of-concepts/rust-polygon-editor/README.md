# Rust polygon editor — proof of concept

**Status:** Unverified in the repository until the Windows build and hands-on checks pass.

This is an isolated experiment for [Comic Panel Composer's technology investigation](../../docs/HIGH_LEVEL_REQUIREMENTS.md). It reuses the basic geometry and GUI approach from an earlier manually tested local experiment, but the code in this repository must be tested independently. It is **not** a release or the application's architecture.

## Purpose

Establish that Rust with egui/eframe can:

- Open a graphical Windows desktop window.
- Draw a filled, outlined, four-corner irregular polygon.
- Move the polygon's four vertices by dragging visible handles.
- Restore its initial shape with **Reset**.
- Keep the polygon coordinates independent of UI drawing code, with automated tests for initial coordinates, moving a corner, hit testing and resetting.

## Build and test on Windows

Prerequisites: Rust stable with Cargo and the MSVC C++ build tools/Windows SDK (as required by the Rust Windows MSVC target).

From the repository root, in PowerShell:

```powershell
cargo test --manifest-path .\proof-of-concepts\rust-polygon-editor\Cargo.toml
cargo build --manifest-path .\proof-of-concepts\rust-polygon-editor\Cargo.toml
cargo run --manifest-path .\proof-of-concepts\rust-polygon-editor\Cargo.toml
```

Cargo downloads dependencies on the first run. The first build may take several minutes.

### Manual acceptance checks

1. Confirm the **Rust polygon proof of concept** window appears.
2. Confirm a blue filled and outlined four-corner polygon appears with four yellow handles.
3. Drag each handle to a different position. Confirm its corner follows the pointer and the polygon updates.
4. Click **Reset**. Confirm all four corners return to their initial positions.
5. Resize the window and note any clipping or interaction oddities.

Record compiler errors, warnings and visual anomalies. Automated tests do **not** prove the Windows mouse interaction works.

## Structure

- `src/lib.rs` — framework-independent, deliberately minimal four-point polygon geometry and its unit tests.
- `src/main.rs` — egui/eframe window, rendering, pointer input and Reset.
- `Cargo.toml` — dependencies pinned to the egui/eframe 0.27.2 versions used for this experiment.

The project is independent of other experiments. Its `target` directory is ignored by Git. Cargo creates a `Cargo.lock` on first build; retain it when making the experiment reproducible.

## Boundaries and open questions

This experiment draws **convex** polygons only. Dragging corners into a concave or self-intersecting shape does **not** validate correct fill rendering. Four corners are a test constraint, not a product limit.

Coordinates are currently absolute window points, not a document space with pan/zoom transforms. This experiment does not establish clipping, Bézier editing, Boolean shape operations, multi-panel editing, persistence, export, image rendering, text layout, display scaling behaviour or acceptable performance for a real comic project.

Passing the experiment supports further investigation; it does **not** approve Rust or egui as the final technology.
