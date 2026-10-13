# Comic Panel Composer — Project Planning (DRAFT)

> **DRAFT — INFORMATION CAPTURE ONLY.** This document holds previously recorded open questions, a provisional delivery sequence and example acceptance scenarios. None of these constitute an approved plan, agreed design decision, delivery commitment or final acceptance criteria. Review and explicit agreement are required before relying on any of them for implementation or scheduling.

Product requirements remain in [HIGH_LEVEL_REQUIREMENTS.md](HIGH_LEVEL_REQUIREMENTS.md).

## Decisions to define by feature

### Platform and performance

- Select the language, GUI framework, renderer and text-layout approach.
- Set quantitative performance targets and representative document sizes.
- Decide whether the platform-specific photo, capture, printing and sharing integrations in this draft should be retained, replaced or deferred for Windows.

### Project files and asset libraries

- Decide whether projects package assets/fonts or link to them.
- Decide whether the application-only font library is global, per project, or both.
- Decide whether importing existing Comic Life projects is wanted; it is not yet a committed requirement.

### Webcomic formats and publishing workflows

- Establish the editable document structure for very long scrolling compositions and episodes, including smooth navigation and performance on representative hardware; choose internal sectioning only after testing.
- Specify how print-page and webcomic layouts can share source content while retaining separate non-destructive arrangements, styles and text positions; automatic reflow is not assumed.
- Define the scope of webcomic templates, mobile/reader previews, scrolling guides, gutters and pacing controls.
- Check destination-specific limits and export rules against current official publisher documentation before defining preset values; handle changing platform specifications without requiring a document-format change.
- Decide which webcomic publishing assets beyond episode images (such as thumbnails and cover images) belong in an initial or later release. Direct publication to third-party sites is not implied by export support.

### Pages, templates and the workspace

- Specify facing-page margins, spread behaviour and master-page editing.
- Define reading-order behaviour for overlapping, nested, borderless, split and decorative/background panels, including spread layouts; keep reading order distinct from layer stacking order.
- Define how reading-order suggestions, direction preferences, manual reordering and optional exported numbered markers interact, particularly after adding, removing or rearranging panels.
- Specify texture management and document-keyword editing.
- Confirm click, placement-tool and drag interactions for the element palette, including the plus-marked balloon control.
- Validate floating, docking and workspace restoration behaviour, including mixed-DPI multi-monitor setups, against candidate Windows GUI frameworks.
- Define the theme system, the interface elements and button states it can style, and the format for sharing theme presets.
- Define Layers panel semantics for grouping, selection, clipping, object ownership and stacking order.
- Decide how page-wide background artwork is represented and edited relative to full-page panels, ordinary layers, panel frames and page textures; both workflows must remain possible.
- Specify frame division, gutter alignment and content-preservation behaviour when panels are split, moved or resized.
- Define how panel-owned content is retained or reassigned when a frame is removed, split or combined, including independent frame/content transforms and how the operation is undone.
- Determine whether combining panel frame outlines also combines their associated content collections, or only their geometry; keep the clipping and layer behaviour explicit and avoid assuming Clip Studio's folder architecture.

### Shapes, balloons and effects

- Define the custom-shape editing workflow and preset geometry, including inserting/removing outline points and mixing straight and curved sections across shape-based objects.
- Define how reshaping affects contained text, image clipping, balloon tails, fills and effects.
- Define frame border visibility and transparent-gutter behaviour for inset panels, borderless/open panels and panels extending beyond the page.
- Define per-object panel breakout and border stacking behaviour, including association with a panel, selective clipping, and how borders are hidden behind extending artwork.
- Define where editable gradient transparency is applied relative to fills, image adjustments, clipping masks, borders and object-level opacity.
- Define the editing workflow and operation ordering for non-destructive Combine/Union, Subtract, Intersect and Exclude, including component selection, nested/compound shapes, cut-outs and shared outline rendering.
- Specify balloon Variety and automatic tail behaviour.
- Define selection, movement and association rules for independently editable thought-tail bubbles, including when the parent balloon moves or changes shape.
- Define how telepathic preset contours and breath marks remain separately editable and whether their relative positions follow balloon reshaping.
- Define independently adjustable tail-width/taper controls, attachment and terminal squink geometry, including connections to off-panel origins and shape combinations.
- Define editing and rendering for conventional balloon styles, flat butting, border breaking, multi-outline balloons and directly joined versus connector-joined balloon assemblies.
- Define whether balloon clipping follows the panel shape, page bounds or a user-adjustable mask, and how enveloping interacts with other objects.
- Define the Compact outline option and the three unlabeled stroke controls.
- Factor represents caption/panel effect strength. Filter Level represents filter strength; detailed parameter behaviour still needs defining.

### Text and fonts

- Confirm examples and exact coverage of dynamic fonts, variable axes and OpenType features.
- Specify text layout and formatting preservation when applying styles or changing decorative lettering.
- Use Blambot's Comic Book Grammar & Tradition as a reference catalogue for supported comic-lettering conventions; treat its editorial rules as optional guidance, not enforced restrictions on creators.
- Validate character-level emphasis, alternate glyphs, punctuation, caption notes and music/singing text against the selected fonts and text engine.
- Define the SFX library's built-in categories and how custom entries are saved and organised.

### Images and output

- Select input/output formats and the final filter/preset catalogues.
- Specify image-adjustment ranges, units and Reset scope.
- Specify EPUB layout and reader targets, CBZ packaging, and PDF font/vector handling.
- Define print export behaviour for live/safe area, trim and bleed, including whether page-edge artwork is cropped or extended for the output format.
- Establish compatibility before retaining external sharing services.

## Proposed delivery sequence

This earlier, feature-led sequence is retained as a provisional idea, not an approved plan or a reduction of target scope. See [Development Direction (DRAFT)](DEVELOPMENT_DIRECTION_DRAFT.md) for a newer, investigation-first proposal. Neither sequence is a release commitment.

1. **Technical proof:** an image clipped to an editable shaped panel, an overlapping balloon with an adjustable tail, editable text, save/reopen and a matching export. Evaluate typography and interaction alongside performance.
2. **Usable document editor:** multi-page management, page settings, ordinary panels/balloons/captions, font selection, undo/redo and dependable saving.
3. **Reusable composition:** templates, panel layouts, custom shapes, master pages, image collections and style libraries.
4. **Advanced presentation:** advanced font controls, decorative lettering, outlines, shadows, image adjustments and filters.
5. **Complete output workflow:** all target export formats, document metadata and agreed quality settings.

## High-level acceptance scenarios

- Compose a multi-page comic, move pages up and down in the Pages panel, and confirm that the document sequence follows the new order. Save, reopen and export it; retain that order and the editable page content.
- Save a custom panel layout and a styled page template; reuse both in another page without losing their intended geometry or appearance.
- Overlap differently shaped panels, adjust their image crops, and select/edit the intended object reliably.
- Place an adjustable full-page background image, overlay independently cropped and bordered inset panels, and place narration captions in the visible background/gutters; confirm panel moves, background adjustments and layering remain editable after save/reopen and render correctly in export.
- Place leaves, particles and a partially cut-out character/animal so they extend beyond their panel while the frame line appears behind them and remains visible elsewhere; alter the breakout positions and clipping, save/reopen and check that the intended layering appears on export.
- Apply an editable top-to-bottom transparency fade to a background or placed image, reveal underlying artwork, adjust the fade endpoints and softness, then save/reopen and export with the fade and original artwork preserved.
- Split an existing panel with an angled cut, modify the resulting gutters and panel crops independently, and create a borderless image extending into a bleed area; verify saved content, on-screen preview and final trimmed output.
- Move and resize a frame without altering its image crop, reposition its artwork without moving the frame, then move both together; split the frame into equal and custom divisions with different content-handling choices, and remove its border without losing artwork. Verify undo/redo, panel associations, save/reopen and exported appearance.
- Combine two panel outlines while retaining their independently editable images and clipping, then confirm that merging frame geometry does not silently merge or delete the associated artwork.
- Assign a reading sequence to irregular, overlapping panels over a full-page background by clicking them or reordering a list; review optional numbered/arrows overlays, add a styled reader-visible number marker, then confirm the reading sequence survives save/reopen and only intentionally placed markers appear in export.
- Create speech and thought balloons, customise a shape, add and move tails, and edit the text afterwards.
- Draw a custom curved tail, join it to a balloon without a visible line across the connection, then reshape or detach it; verify that the editability and seamless appearance survive save/reopen and export.
- Resize the wide base, taper and tip of a balloon tail independently without reducing it to a thin stroke; add a squink at the tail tip, move its off-panel origin and verify the joins remain correct on export.
- Create a thought balloon with several tail bubbles; resize, reshape and reposition each independently, change the gaps, add and remove bubbles, then move the parent balloon and confirm the custom arrangement survives save/reopen and export.
- Create a tail-free telepathic balloon with overlapping editable wavy contours and repositionable breath marks, customise its appearance and confirm the parts remain editable after save/reopen and export.
- Construct a burst balloon, a double-outline balloon and a radio/electric balloon; directly join two balloons, create a separate editable connector, and butt/break a balloon into a panel border while preserving independent editing.
- Use styled editorial captions, emphasis and alternate comic glyphs, music notes/wavy dialogue, hollow and distressed SFX lettering, then check that unusual lettering remains editable after save/reopen and export.
- Create custom shapes and exercise Combine/Union, Subtract, Intersect and Exclude; move and reshape the source shapes after each operation, transform the result as a whole, and verify that the intended appearance and independent editing survive save/reopen and export.
- Insert and remove points on a balloon and an irregular caption/text box, mix straight and curved edges, and confirm that contained content and balloon tails remain editable after save/reopen and export.
- Drag a categorised SFX preset onto a page, customise its wording and effects, then save it as a reusable library entry.
- Customise application and button colours, save and export a theme, and restore the same appearance after importing it.
- Place a balloon behind an image while another balloon crosses a panel boundary; reorder them in the Layers panel, save/reopen and verify the same appearance in export.
- Detach an inspector, move it to a monitor with a different display scaling setting and restore the workspace without inaccessible or incorrectly scaled controls.
- Correct a misspelling, add a fictional character name to the user dictionary and verify it is recognised in later text editing.
- Select installed and app-only fonts, apply supported advanced font features, and retain the intended appearance in export.
- Apply borders, outlines, shadows and decorative lettering while retaining editable source objects.
- Adjust image appearance, reset adjustments and reopen the project without losing access to the original image.
- Export the complete comic and a selected page range in the applicable formats, preserving page order and matching the edited layout.
- Compose a multi-section vertical-scroll episode with a continuous background, editable panels, intentionally varied gaps and dialogue between panels; inspect its pacing and lettering in the phone-width scrolling preview, save and reopen without loss of editability.
- Duplicate and recompose conventional comic artwork into a separate vertical-scroll layout; alter panel crops and balloon positions, then confirm the original page layout is unchanged.
- Export a tall webcomic episode using two configurable publishing profiles with different maximum image dimensions; inspect the numbered slices for correct sequence, seamless joins, visual quality, file-size compliance and suitable break placement.
