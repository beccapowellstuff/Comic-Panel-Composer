# PoC 2 — Docking, native windows and DPI

**Status: the app launched on Windows, but moving a native floating panel between monitors with different DPI caused jumping, resizing, control/pointer problems and shimmering. A diagnostic change is awaiting re-test; unit test results have not yet been supplied.**

This is an isolated technology evaluation for Comic Panel Composer, **not** a production workspace or a technology decision.

## Why we are testing this

The [high-level requirements](../../docs/HIGH_LEVEL_REQUIREMENTS.md) call for customisable dockable panels that can be pulled into independent Windows windows, moved to other monitors, redocked, and restored on reopening, including monitors with different display scaling.

egui/eframe supports genuine native desktop windows through viewports. However, the popular `egui_dock` library's floating `egui::Window` surfaces **remain inside the application's viewport**. This PoC deliberately exposes both behaviours rather than treating them as equivalent.

## Components

- **Canvas**: a sample page whose coloured layer responds to editing controls.
- **Pages**: select pages 1, 2 or 3.
- **Layers**: show or hide the artwork layer.
- **Inspector**: change layer opacity and visibility.
- **DockArea**: rearrange and resize tabs; dragging a tab out creates an **in-application floating egui window**.
- **Native Windows**: click **Detach as a REAL Windows window** inside Pages, Layers or Inspector to create a separate OS-level viewport. Click **Dock back into main workspace** to return that panel to the docking tree.
- **Save workspace**: saves dock layout, detached panel flags, sample state and observed native window bounds as JSON under `%LOCALAPPDATA%\ComicPanelComposerPoCs\docking-workspace.json` (uses the system temporary directory if `LOCALAPPDATA` is unavailable). The file is read on restart. **Diagnostic limitation:** saved native window positions and sizes are currently *recorded but not reapplied*. This is intentional while we isolate the mixed-DPI feedback loop. Other state/layout restoration is still under test.
- **Reset workspace**: restores the default layout in memory; it does not delete the saved layout file.

This uses **egui/eframe 0.36.2 and egui_dock 0.21.1** to evaluate currently available features, independent of the earlier egui 0.27.2 polygon experiment.

## Windows build and run

Prerequisites: Rust stable / Cargo for Windows MSVC and the corresponding MSVC C++ build tools / Windows SDK.

From the repository root in PowerShell:

```powershell
cargo test --manifest-path .\proof-of-concepts\docking-workspace\Cargo.toml
cargo build --manifest-path .\proof-of-concepts\docking-workspace\Cargo.toml
cargo run --manifest-path .\proof-of-concepts\docking-workspace\Cargo.toml
```

The first build will download and compile new dependencies, so it will take longer than the earlier polygon test.

## Manual test matrix — record observations, not assumed success

| Criterion | How to test | Status |
|---|---|---|
| Internal docking | Drag Pages and Layers tabs to other dock positions; resize boundaries. | Not tested |
| Internal floating | Drag a tab out of the DockArea. Verify it **cannot** leave the main OS window. | Not tested |
| Native window | In Inspector, choose **Detach as a REAL Windows window**. Verify a second independent Windows window exists (e.g. taskbar / Alt+Tab). | Not tested |
| Shared state | While Inspector is native, change opacity and visibility. Watch the canvas change. | Not tested |
| Native redocking | Click **Dock back into main workspace**, or close the native window. Verify Inspector becomes a dock tab again. | Not tested |
| Multiple native windows | Detach Pages, Layers and Inspector simultaneously; use controls in each. | Not tested |
| Second monitor | Move each native window outside the main application's boundaries onto another monitor. | Not tested |
| Mixed DPI | Move a window between monitors set to different scaling (e.g. 100% and 150%); assess sizes, text sharpness and hit targets. | **Failed initial test:** window jumped/resized, text/controls and pointer interaction misbehaved, and it shimmered. Re-test needed after diagnostic fix. |
| Window management | Focus another application, minimise/restore the main window, close a floating window; check unexpected focus/visibility issues. | Not tested |
| Save/restore | Arrange and detach panels, **Save workspace**, close application, restart; inspect dock placement, native windows and bounds. | Not tested |
| Native drag-out | **Drag a docked tab directly into a NEW OS-level window without pressing a button.** | **Not implemented** |
| Native drag-back | **Drag a native OS window's panel back into a dock target.** | **Not implemented** |

### Limitations of this first investigation

- **The two direct drag-to-native behaviours are mandatory requirements, but are not implemented here.** In this version, native detach/redock are buttons. This is *partial evidence only*, not a pass against the full requirement.
- The implementation uses `show_viewport_immediate` for simpler shared state. egui documents additional repaint costs with multiple immediate viewports. Responsiveness and CPU usage need real measurement; evaluate `show_viewport_deferred` if necessary.
- Native window bounds are still observed and serialised, but no longer fed back into the live viewport builder on every frame. This isolates a likely DPI-dependent position/size feedback loop; **restoring saved native-window placement is temporarily disabled**. We must assess both DPI movement and placement persistence separately before approving the approach.
- `egui_dock` floating surfaces are internal egui windows; they are intentionally not presented as an alternative satisfying independent native window requirements.
- This experiment does not evaluate multi-document windows, production workspace theming, accessibility, or robust failure recovery.
- If the implementation proves difficult to make reliable, run an equivalent C++/Qt docking experiment rather than lowering the requirements.

## Evidence and decision

Record Rust and Windows versions, compiler errors, warnings, actual test results, any crash or unexpected behaviour, and whether a feature worked as a real OS window or only inside the application. A build is not proof of drag-out or reliable DPI.

**Do not choose Rust/egui or reject it based only on this first version.** The most valuable outstanding test is an actual drag-to-native-window workflow versus Qt's dock widgets.
