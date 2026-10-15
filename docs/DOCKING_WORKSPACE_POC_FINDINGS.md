# Docking workspace PoC — technology findings

**Recorded:** 15 October 2026  
**Status:** Stage 1 technology investigation, not production design approval  
**Preserved working implementation:** [`proof-of-concepts/docking-workspace/`](../proof-of-concepts/docking-workspace/)  
**Product requirements:** [`HIGH_LEVEL_REQUIREMENTS.md`](HIGH_LEVEL_REQUIREMENTS.md)

This record separates observed behaviour, automated verification and unresolved
questions. Keeping a PoC in `main` does **not** mean its GUI technology, external
patches, OS integration or engineering choices have been accepted for the product.

## Confirmed through owner testing on Windows

The owner tested the egui/eframe and egui_dock workspace on three monitors:
centre primary 3840 × 2160 at 150%; left and right portrait displays
1440 × 2560 at 100%.

- **Native floating panels:** Pages, Layers and Inspector can be detached into
  genuine Windows windows on separate monitors.
- **Internal docking:** Tabs can be dragged and rearranged within the same
  window and dropped into groups or separate panes (including splitting below
  another pane). This is **not** cross-native-window tab dragging.
- **Shared state:** The UI can update the sample canvas and controls across
  independent native tool windows.
- **Mixed-DPI movement:** The original winit integration exhibited serious
  positioning/dragging issues when moved between monitors with different
  display scaling. With the pinned third-party winit backport in this PoC,
  the owner confirmed that the mixed-DPI behaviour worked in their tests.
- **Restoring floating windows:** A first attempt stored ambiguous logical
  coordinates and could restore a window off the right-hand display. The
  subsequent physical-desktop-pixel placement and monitor-work-area checks
  restored native windows to the expected left/right displays in owner testing.
- **Menu-driven native group merge:** The owner successfully used **Merge this
  window into...** to consolidate tabs into an existing native Windows window.
- **Workspace activation:** Clicking the main application brings detached
  native tool windows forward, without requiring an always-on-top setting.

These results establish specific Windows interactions on the test machine,
not compatibility with every Windows 10/11 configuration.

## Automated verification

- [Windows CI — native-window grouping/focus implementation](https://github.com/becciStuff/Comic-Panel-Composer/actions/runs/38531481061):
  **4 library and 8 binary tests passed**, and the Windows application built.
- The code includes model tests for sample state, safe position arithmetic,
  merging/redocking tab groups, persistence round-trips and loading older
  workspace files. Automated tests do not prove native mouse behaviour or
  mixed-DPI rendering.
- The `egui_dock` JSON layout can contain non-finite rectangle coordinates
  for freshly created nodes. This PoC sanitises a **copy** of the layout for
  serialisation, leaving live UI geometry unchanged. The round-trip test
  covers the previous save/load failure.

## Failed cross-native-window drag experiment — deliberately not adopted

A later experiment, [`poc/cross-window-tab-drag` at
`8cbaa24e`](https://github.com/becciStuff/Comic-Panel-Composer/commit/8cbaa24ecb02c2314ccc2b0b37fe03f6aa99b7da),
attempted to bridge `egui_dock` tab-button dragging across independent
Windows OS windows using physical cursor position and Windows hit testing.

Its [Windows CI](https://github.com/becciStuff/Comic-Panel-Composer/actions/runs/38536077620)
**passed 20 automated tests** and compiled. However, **owner testing did not
achieve direct tab dragging from one native Windows window into another**.
Docking within one window worked; between separate OS windows, the
**Merge this window into...** control remained the only working way to transfer
tabs.

**Decision for the clean research baseline:** retain the last
owner-tested `poc/native-window-grouping-focus` implementation, **not**
the unsuccessful cross-OS dragging bridge. Retain this finding and commit
link as research history. Neither Rust as a language nor egui as a framework
is rejected or approved by that experiment.

## Known defects and limits in the preserved PoC

- Closing an individual tab can make the panel disappear without returning
  it to the main workspace. The owner expressly deferred fixing this during
  the experiment; a recoverable close/hide/reopen policy is needed later.
- Main application window **position** is not yet saved/restored, though
  floating tool windows are. Named saved workspace presets are not implemented.
- Floating-native-window tab groups are serialised and covered by a model
  round-trip test; comprehensive owner verification of full group restoration
  across restart is not recorded here.
- No confirmed native **drag tab out onto empty desktop** to create a new
  Windows window or native-to-native direct tab dragging.
- Win32 title matching for tool-window activation is a PoC expedient, not a
  robust production handle/ownership mechanism.
- The mixed-DPI solution depends on a **pinned third-party winit fork** at
  commit `42b2007c000ff854ce43486f650190791067c890`. This must be
  reviewed against upstream releases before any product implementation.
- Windows 10 compatibility, minimum RAM/graphics requirements, detailed
  accessibility and broad hardware validation are not established.
- The example Canvas/Pages/Layers/Inspector state is **not** the comic
  document model. No image clipping, lettering, shaped balloons, real
  document persistence, or production rendering/export was proved here.

## Preliminary memory observations (not benchmarks)

The owner observed about **108 MB**, **129.7 MB**, and **172.7 MB** in
different debug/release and floating-window configurations. Usage fell when
floating windows were closed. The conditions were not controlled, so no
quantitative minimum-RAM or leak-free claim follows from these numbers.

## Research conclusion and next focus

The small Rust + egui Windows workspace can display and manage genuine native
floating tool windows, and supports functional internal docking and controlled
native-window grouping. Cross-OS drag remains an unresolved GUI-framework risk.

The next **proposed** technology investigation is a **small editable comic-page
PoC**: irregular panel geometry, independently positioned/clipped artwork,
an editable speech balloon, mixed-format lettering, save/reopen and visual
export consistency. This is research direction, **not** approval of a final
framework or implementation architecture. A credible alternative (such as
C++/Qt) should still be compared on relevant editor requirements.

The high-level requirements remain unchanged, and checked requirements
identify desired capabilities, **not** finished or first-release features.
