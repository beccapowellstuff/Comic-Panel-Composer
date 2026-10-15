# Proofs of concept

Small, independent experiments for Stage 1 (technology evaluation) of Comic Panel Composer.

These are **not** the Comic Panel Composer application. A successful experiment does not approve a technology choice or establish that the complete editor's requirements can be met.

## Experiments

- [Rust polygon editor](rust-polygon-editor/README.md) — an egui/eframe Windows window with a four-corner polygon, draggable handles and Reset. Exercises basic rendering, interaction and geometry unit tests.
- [Docking workspace, native Windows windows and mixed-DPI](docking-workspace/README.md) — owner-tested floating native windows, internal docking, menu-driven native group merging, shared UI state and workspace activation, with a temporary patched-winit dependency for mixed-DPI support. See [recorded findings](../docs/DOCKING_WORKSPACE_POC_FINDINGS.md) for tests, defects and the unsuccessful direct cross-native-window tab drag experiment.
- [Docking workspace](docking-workspace/README.md) — evaluates dock tabs, genuine native windows, shared state, workspace persistence and Windows multi-monitor/DPI behaviour. Direct drag-to-native docking remains an explicit open requirement.

Each experiment has its own build and test instructions. Do not share application code or dependencies between experiments until an architectural decision has been made.
