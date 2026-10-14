# Proofs of concept

Small, independent experiments for Stage 1 (technology evaluation) of Comic Panel Composer.

These are **not** the Comic Panel Composer application. A successful experiment does not approve a technology choice or establish that the complete editor's requirements can be met.

## Experiments

- [Rust polygon editor](rust-polygon-editor/README.md) — an egui/eframe Windows window with a four-corner polygon, draggable handles and Reset. Exercises basic rendering, interaction and geometry unit tests.
- [Docking workspace](docking-workspace/README.md) — evaluates dock tabs, genuine native windows, shared state, workspace persistence and Windows multi-monitor/DPI behaviour. Direct drag-to-native docking remains an explicit open requirement.

Each experiment has its own build and test instructions. Do not share application code or dependencies between experiments until an architectural decision has been made.
