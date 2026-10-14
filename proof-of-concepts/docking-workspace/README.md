# PoC 2 — Docking, native windows and DPI

**Status: user testing confirms pinned winit fixes mixed-DPI motion and native windows can merge via menu, restore positions and come forward with the main window. This branch explores direct mouse dragging of egui_dock tabs between genuine OS windows, including tab grouping and split drop targets. Windows CI passed 20 model tests and built the application successfully on commit f8b2c80; the real cross-window mouse gesture still requires testing. The tab-close defect and main window position persistence remain open.**

This is an isolated technology evaluation for Comic Panel Composer, **not** a production workspace or a technology decision.

## True cross-OS-window tab dragging — current isolated experiment

**Branch:** `poc/cross-window-tab-drag`. Based on the owner-tested native grouping/focus branch. It retains the successful Windows mixed-DPI patch, physical screen position restoration, and native group persistence.

**Goal:** Discover whether the existing Rust/egui/egui_dock stack can transfer an **actual docking tab with the mouse between two separate native Windows OS windows** (including different DPI monitors), and allow centre (group) or edge (split) drops. This remains a technology probe, not a product architecture decision.

### What has changed

- Uses **`egui_dock::TabViewer::on_tab_button`** to observe a drag started on the real Pages/Layers/Inspector tab, **not** on a new surrogate toolbar handle.
- Samples the left mouse button and **physical Windows cursor position** using Win32 while dragging, even if the pointer leaves the original OS window. `WindowFromPoint` / `GetAncestor` identify the real OS window under the cursor; only recognised windows owned by this process are eligible.
- The receiving window paints a translucent drop-zone preview, using a deliberately simple **whole-window** five-zone model: centre merges into the current group; left/right/above/below inserts a new split.
- On release over another native window or the main window, removes the dragged panel from its source `DockState` and inserts it into the destination `DockState`; automatically closes an empty source native window. The shared document model remains the same.
- Leaves drag-and-drop *within* one `DockArea` to `egui_dock`. Mouse release outside a recognised different window should be ignored by the cross-window transfer logic.
- Supports detaching to the next free native-window slot if the panel's old named slot now hosts another group.

**Important limits and risks to evaluate:** `egui_dock` still owns its original drag gesture in the source window, while our Win32 bridge also tracks the tab. The combined event handling may abort a drag, lose the tab to an **in-app egui window**, or stop repainting when the mouse leaves the source. The preview uses the *entire OS client area*, rather than detecting precisely which leaf panel is under the cursor. The current PoC also uses static window titles for handle identification, and a panel might remain in a native window whose title was derived from a different original panel. Those are technical debts or potential rejection criteria, not resolved requirements.

**Not included:** Dragging to empty desktop space to create a new native window, Windows OLE drag/drop interoperability, native per-leaf target highlighting, drag-to-undock by crossing the main window border, or production-grade focus/handle lifecycle management. The existing **Merge this window into...** menu remains a fallback.

### Manual Windows acceptance test

From PowerShell in the repository root, check for unrelated local modifications before switching:

```powershell
git status --short
git fetch origin
git switch --track origin/poc/cross-window-tab-drag
cargo test --manifest-path .\proof-of-concepts\docking-workspace\Cargo.toml
cargo run --manifest-path .\proof-of-concepts\docking-workspace\Cargo.toml
```

Use `git switch poc/cross-window-tab-drag` if the branch already exists locally.

1. **Actual tab drag:** Create separate Pages and Layers **native Windows windows**. Drag the *Pages tab itself* from its window into the middle of Layers, across monitors if possible. Look for the receiving blue **DROP: Centre** preview. Release. The receiving OS window should contain both tabs and the empty Pages OS window should close. This is the definitive cross-window result; using the Merge menu **does not count**.
2. **Split:** Detach Inspector into another OS window. Drag its tab over the **bottom** (and later left/right/top) edge of the grouped native window, release, and check that Inspector forms a separately resizeable pane rather than another tab.
3. **Main window:** Drag an actual native tab into the centre/side of the **main application window**. It should move into the main DockArea. Drag a docked tool tab from the main application into a **different existing native window**.
4. **Ordinary docking regression:** Reorder tabs and split panes **within** a single DockArea. These ordinary local drags must still work and should **not** trigger a cross-window transfer.
5. **Cancel behaviour:** Drop over another application or the desktop; the original panel should remain recoverable (do not count an internal egui float as success). Repeat with a quick movement.
6. **Mixed DPI and persistence:** Try left portrait 100% and right portrait 100% versus central landscape 150%, saving and restarting after successful drops. Verify grouped/split layouts, physical locations and interactions.
7. **Focus/recovery regression:** Clicking the main application should raise floating tools without permanently marking them always-on-top. **Recover floating panels** should still return all tabs to the main workspace.

**Verified CI:** [Windows workflow for commit f8b2c80](https://github.com/becciStuff/Comic-Panel-Composer/actions/runs/38536077620): 4 library tests, 16 binary tests, and native Windows build succeeded. No GUI session was launched in CI; native mouse dragging and cross-monitor input remain unverified.

**Interpretation:** Automated model tests can prove that moving a `Panel` between `DockState`s is correct, but **cannot prove the real cross-window mouse gesture** survives native Windows/egui event routing. Only user testing does that. If the pointer never highlights the other native viewport, or release doesn't cause a transfer, the interaction is **not passed**. That is important evidence for comparing egui with Qt's docking ecosystem.

---

## Native tab grouping and workspace activation — previous experiment

**Branch:** `poc/native-window-grouping-focus`, based on the user-tested `poc/native-window-position-safe`. All prior window positioning and the pinned experimental winit mixed-DPI fix remain in place.

**Why this experiment:** Internal `egui_dock` tabs group and undock correctly, but genuine independent OS windows previously contained a lone panel without tab controls. Additionally, activating the main application did not bring its independent native tool windows forward.

### New functionality (awaiting Windows testing)

- Each floating OS viewport now contains its **own `DockArea` and `DockState`**. Panels within one floating OS window show normal draggable tabs.
- The **Merge this window into…** menu moves all tabs from one actual native Windows window into another already-open native window; the emptied source window closes. This is a **controlled merge**, not cross-native-window mouse drag.
- **Dock all tabs in main workspace** brings every tab in the selected native group back to the main application.
- Saved `native_docks` preserve native window tab groups across restart. An older saved workspace missing this field is accepted and single-panel native windows are migrated when the app opens.
- On Windows, main-viewport focus transitions trigger a Win32 `SetWindowPos(HWND_TOP, SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOSIZE)` for tool windows belonging to this process, without changing the always-on-top flag. This uses **window-title matching for the PoC only**, not a proposed production architecture.
- **Bring tool windows forward** is an explicit diagnostic button in the main window. It separates auto-focus event handling issues from Win32 Z-order issues.

**Still NOT implemented:** Dragging a tab from one actual native Windows window into another, dragging a dock tab directly outside the main OS viewport to create a new native window, or dragging a native window's tab back to a main dock target. The Merge menu is not a pass for these requirements. Main application window positioning on restart is still not stored by this PoC.

### Run and test

First preserve any local modifications before switching:

```powershell
git status --short
git fetch origin
git switch --track origin/poc/native-window-grouping-focus
cargo test --manifest-path .\proof-of-concepts\docking-workspace\Cargo.toml
cargo run --manifest-path .\proof-of-concepts\docking-workspace\Cargo.toml
```

Use `git switch poc/native-window-grouping-focus` if the branch already exists locally.

Manual checks, with main monitor 3 (150%) and secondary monitors 2/1 (100%):

1. **Native group creation:** Detach Pages and Layers into two separate Windows windows. In one, use **Merge this window into…** to transfer into the other. Verify the receiver now has both **Pages** and **Layers** tabs and the source window closes.
2. **Native tab docking:** Reorder the two tabs by dragging inside that native window; where supported, place them in separate docking panes and then rejoin them.
3. **Shared state:** Change selected page or layer visibility in the native group; verify the main Canvas updates immediately.
4. **Group persistence:** Move the grouped native window to a secondary monitor, save, exit and restart. Verify the group, its two tabs, selected state, and monitor positioning are restored. Verify neither duplicate nor missing panels.
5. **Redock all:** Click **Dock all tabs in main workspace**; check both panel tabs return. Use **Recover floating panels** in the main window to recover all other native groups if needed.
6. **Main app activation:** Put Explorer or another application in front of the main window and the floating tool window(s). Click the main Comic Panel Composer window. Verify its independent native tool panels come forward **without taking keyboard focus away from the main window**.
7. **External app behaviour:** Activate Explorer again. The tool windows must **not** remain above Explorer as permanent always-on-top windows.
8. **Focus troubleshooting:** If automatic bring-to-front fails, click **Bring tool windows forward** and note its reported count. If this works while automatic activation fails, the issue is the focus-transition detection; if the count is zero, the Win32 handle discovery is suspect.
9. **Regression:** Drag grouped windows across monitors in both directions, use Win+Shift+Arrows, resize, restart and recover to verify the previous DPI and window-position fixes remain sound.

The Windows workflow passed **12 tests (4 library, 8 binary)** and completed a Windows build on [commit 1a7325a](https://github.com/becciStuff/Comic-Panel-Composer/actions/runs/38531481061). The tests also exposed an upstream `egui_dock` persistence hazard: a fresh layout uses non-finite `Rect::NOTHING` bounds that JSON cannot round-trip. The PoC now sanitises a **copy** of its tab layout before saving, preserving the live layout. **GUI behaviour must still be tested manually**. Record whether the interface follows the expected rules rather than assuming it does because it compiles.

---

## Safe physical window placement — previous experiment

**Branch:** `poc/native-window-position-safe` (based on `poc/native-window-position-restore`, retaining the same experimental patched winit dependency).

**Observed user result before this change:** detached panels restored to the left monitor but a panel saved on the right became invisible and Windows keyboard monitor-moving shortcuts did not recover it. The saved value reported for one native window was `position: [4102.0, 568.0]` and `inner_size: [319.0, 328.0]`.

**Cause under investigation:** in egui 0.36.2, `ViewportInfo.outer_rect` is stored in egui points (physical coordinates divided by the current viewport's pixels per point), but `ViewportBuilder.with_position` goes through winit logical coordinates at the newly created window's initial DPI. This can place a right-monitor window off-screen when it starts on the 150%-scaled primary monitor. The correction below still requires Windows testing.

**Changes in this branch:**

- New saved field `native_position_px`: virtual-desktop **physical pixel positions** for detached windows. The original `native_bounds` field remains, retaining layout compatibility and window-size information; its historical *position* values are ignored because their scaling is ambiguous.
- A native window is initially created with its remembered size but **without an initial position**. Only in its first native callback, the saved pixel location is converted into egui points using that particular viewport's actual pixels-per-point and applied **once** using `ViewportCommand::OuterPosition`. Subsequent frames never send more positioning commands.
- On Windows, the target position is checked against the nearest connected monitor's actual working area through `MonitorFromRect` and `GetMonitorInfoW`. The upper-left part of the title bar is clamped to a reachable part of that monitor; a disconnected monitor's position therefore falls back to the nearest connected one. If the Windows API fails, no saved positioning command is sent.
- **Recover floating panels** in the main application's toolbar redocks all native panels and clears their saved OS bounds. It intentionally keeps document state. **Save workspace** afterwards if you want the recovered layout persisted.
- Four pure geometry tests cover left/negative positions, valid right positions, extreme off-screen restoration and an off-top/left window. They do **not** establish that Windows positioning works; use the manual checks.

### Run the current test

From the repository's PowerShell terminal, preserve local modifications if any:

```powershell
git status --short
git fetch origin
git switch --track origin/poc/native-window-position-safe
cargo test --manifest-path .\proof-of-concepts\docking-workspace\Cargo.toml
cargo run --manifest-path .\proof-of-concepts\docking-workspace\Cargo.toml
```

If the branch is already checked out locally, use `git switch poc/native-window-position-safe` instead of `git switch --track`.

**Important on the first run:** Your existing JSON was saved by the previous version, so it does **not** contain `native_position_px`. The old dock configuration and native-detached flags should still load, but the native windows may start on the primary display. Position them on the desired monitors and click **Save workspace** to create physical-coordinate records.

1. Detach Pages and Inspector; place one on **monitor 2 (left, 100%)** and the other on **monitor 1 (right, 100%)**, with the main workspace on **monitor 3 (150%)**.
2. Change both floating-window sizes; save the workspace, close the app, reopen. **Both should return to their correct monitors** and usable positions and sizes, and neither should shimmer.
3. Repeat the save/restart test after swapping the two tool windows between monitors.
4. Check **Win + Shift + Arrow**, snapping and mouse hit targets still work.
5. Test **Recover floating panels** while one or more native tool windows are open (even if one is off-screen): they should return to docked tabs in the main application. Click Save workspace, restart and verify none reopen off-screen.
6. Optional resilience check: while the app is closed, disconnect or disable a secondary display that held a native window, restart, and verify the panel appears on a connected display. Reconnect and restore the arrangement afterwards.

**Criteria:** No lost/off-screen windows on a stable three-monitor setup. At least a draggable title-bar section remains on a connected monitor when the layout changes. Recover must be available independently of the native windows. Display/DPI behaviour must remain stable using the patched winit dependency.

**Verification:** The GitHub source changes and API signatures were inspected, but the new code has **not** yet been compiled or exercised on the user's Windows hardware. Treat the behaviour as unverified until the test is run. A successful test still does not solve native cross-window dock-and-merge.

---

## Previous position-restoration experiment (historical)

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
