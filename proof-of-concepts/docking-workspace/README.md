# PoC 2 — Docking, native windows and DPI

**Status: the unpatched app launched on Windows, but moving a native floating panel from monitor 3 (150%) to monitor 2 (100%) caused jumping, resizing, shimmering and incorrect interaction. Removing the window-bounds feedback in the app did not fix it. This branch now tries a pinned winit 0.30.13 backport; patched Windows behaviour and automated test results are awaiting verification.**

This is an isolated technology evaluation for Comic Panel Composer, **not** a production workspace or a technology decision.

## Window-position restoration follow-up — this branch

This branch, `poc/native-window-position-restore`, builds on the experimental patched-winit branch without altering the successful mixed-DPI dependency fix. **The new source change restores each floating native window's saved outer position and inner size only when that window is first created**, never repeatedly during dragging. This is intended to restore detached panels to their previous monitors and dimensions.

### Windows test (not yet verified)

From the repository root, check `git status --short` and preserve local changes before switching branches.

```powershell
git fetch origin
git switch --track origin/poc/native-window-position-restore
cargo test --manifest-path .\proof-of-concepts\docking-workspace\Cargo.toml
cargo run --manifest-path .\proof-of-concepts\docking-workspace\Cargo.toml
```

If you've already switched to this branch before, use `git switch poc/native-window-position-restore` instead.

1. **Detach** Pages, Layers and Inspector into native windows.
2. Move one floating window to monitor 2 (left; 100% scaling) and another to monitor 1 (right; 100% scaling). Keep the main workspace on monitor 3 (150%). Resize the floating windows.
3. Click **Save workspace** and exit the application.
4. Start the app again. Check that the detached panels return to their respective monitors and approximate previous positions and sizes.
5. Drag them between monitors again; check for DPI bouncing, runaway resizing, blurry text and incorrect mouse hit targets.
6. Try **Win + Shift + Left/Right** and **Win + Left/Right**. These must remain stable.
7. Test the Reset button, remembering it resets *in-memory* layout only. To make the default layout the newly saved layout, press **Save workspace** after Reset.

**Known limitations:** Window positions and sizes are currently stored in egui UI points, not a monitor-independent Windows physical-coordinate representation. Saved placement may need further work if DPI conversion alters the result, monitors are unplugged, or the monitor arrangement changes. A failed restore is **a result to report**, not evidence of a completed feature. The code is not yet compiled in this session; user testing on Windows is required.

**Next separate experiment:** provide tabbed `egui_dock` containers inside native viewports and investigate *real* drag-and-drop between independent Windows windows. This is **not implemented by the position-restore change**. Internal floating-tab grouping is already confirmed working and must not be mistaken for cross-OS-window docking.

---

## Experimental winit fix — pinned dependency used by this branch

This branch, `poc/docking-workspace-winit-fix`, tests **exactly the same application** as `poc/docking-workspace` using a [winit 0.30.13 backport of upstream DPI fix #4341](https://github.com/bigtree108-dmytro/winit/commit/42b2007c000ff854ce43486f650190791067c890). The dependency is pinned to that exact commit in `Cargo.toml` using Cargo's `[patch.crates-io]` mechanism.

**Purpose:** determine whether the mixed-DPI bouncing, repeated resize, shimmering and bad pointer interaction observed on the user's three monitors originate in winit's Windows 11 DPI handling. This is not a permanent dependency recommendation; the code comes from a third-party fork. Review and official upstream support would be necessary before product use.

From the existing repository folder, first check for local changes, then switch to this experimental branch:

```powershell
git status --short
git fetch origin
git switch --track origin/poc/docking-workspace-winit-fix
```

If the experimental branch is already present locally, use `git switch poc/docking-workspace-winit-fix` instead of `git switch --track`.

Confirm that Cargo has selected the patched dependency:

```powershell
cargo tree --manifest-path .\proof-of-concepts\docking-workspace\Cargo.toml -i winit
```

The output should identify `winit v0.30.13` from `github.com/bigtree108-dmytro/winit`, with commit `42b2007c...`. If it instead shows the registry copy, **stop and report that**, rather than attributing the result to the patch.

Then run:

```powershell
cargo test --manifest-path .\proof-of-concepts\docking-workspace\Cargo.toml
cargo run --manifest-path .\proof-of-concepts\docking-workspace\Cargo.toml
```

### Comparison acceptance checks

Compare with the previous unpatched branch on the **same Windows setup**: monitor 3 (primary, 3840 × 2160 at 150%) in the middle; monitors 2 and 1 (1440 × 2560 at 100%) to the left and right.

1. Detach Inspector as a genuine native window on monitor 3.
2. Drag it to monitor **2 (left)** and back several times, checking for shimmer, jumping, runaway resizing, mismatched text size and incorrect pointer input.
3. Drag it to monitor **1 (right)** and back; compare.
4. Try **Windows + Shift + Left/Right** while the floating window is active.
5. Try Windows snapping with **Windows + Left/Right**; inspect window size and final placement.
6. Move the Inspector and use its opacity slider on each monitor. Confirm the canvas updates without incorrect mouse targeting.
7. Re-dock the Inspector via the existing button. Note any focus or visibility problems.

**Pass for this narrowly scoped patch test:** independent floating windows traverse all three monitors with no DPI bounce, growth or pointer problems, including keyboard movement and snapping. Direct drag-to-native undocking/redocking is still not implemented and cannot be marked passed. Restoring native-window coordinates is temporarily disabled in both variants and is not covered by this test.

To return to the unpatched experiment:

```powershell
git switch poc/docking-workspace
cargo run --manifest-path .\proof-of-concepts\docking-workspace\Cargo.toml
```

Cargo may refresh the local `Cargo.lock` when switching dependency sources. Preserve unrelated local work. Do not commit generated changes unless agreed.

**Current verification:** the pinned patch is configured in GitHub; the Windows compiler and interactive result are **not yet verified**. If the patched variant fails to build, capture the exact error and do not treat it as a graphics-framework finding until the dependency issue is resolved.

---



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
- **Save workspace**: saves dock layout, detached panel flags, sample state and observed native window bounds as JSON under `%LOCALAPPDATA%\ComicPanelComposerPoCs\docking-workspace.json` (uses the system temporary directory if `LOCALAPPDATA` is unavailable). The file is read on restart. **On this branch:** saved native-window placement is reapplied only once when opening a window. Multi-monitor restore is awaiting real Windows verification.
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
| Mixed DPI | Move a window between monitors set to different scaling (e.g. 100% and 150%); assess sizes, text sharpness and hit targets. | **Failed original and unpatched diagnostic tests:** jumping/resizing/shimmering and mouse mismatch moving to left monitor; patched dependency awaiting re-test. |
| Window management | Focus another application, minimise/restore the main window, close a floating window; check unexpected focus/visibility issues. | Not tested |
| Save/restore | Arrange and detach panels, **Save workspace**, close application, restart; inspect dock placement, native windows and bounds. | Internal dock layout already confirmed by user; **native monitor/position restoration awaiting testing on this branch**. |
| Native drag-out | **Drag a docked tab directly into a NEW OS-level window without pressing a button.** | **Not implemented** |
| Native drag-back | **Drag a native OS window's panel back into a dock target.** | **Not implemented** |

### Limitations of this first investigation

- **The two direct drag-to-native behaviours are mandatory requirements, but are not implemented here.** In this version, native detach/redock are buttons. This is *partial evidence only*, not a pass against the full requirement.
- The implementation uses `show_viewport_immediate` for simpler shared state. egui documents additional repaint costs with multiple immediate viewports. Responsiveness and CPU usage need real measurement; evaluate `show_viewport_deferred` if necessary.
- Native window bounds are observed and serialised, and are applied once at window creation. This avoids repeated per-frame position commands. Behaviour across mixed-DPI monitors, display changes and keyboard snapping still needs manual verification.
- `egui_dock` floating surfaces are internal egui windows; they are intentionally not presented as an alternative satisfying independent native window requirements.
- This experiment does not evaluate multi-document windows, production workspace theming, accessibility, or robust failure recovery.
- If the implementation proves difficult to make reliable, run an equivalent C++/Qt docking experiment rather than lowering the requirements.

## Evidence and decision

Record Rust and Windows versions, compiler errors, warnings, actual test results, any crash or unexpected behaviour, and whether a feature worked as a real OS window or only inside the application. A build is not proof of drag-out or reliable DPI.

**Do not choose Rust/egui or reject it based only on this first version.** The most valuable outstanding test is an actual drag-to-native-window workflow versus Qt's dock widgets.
