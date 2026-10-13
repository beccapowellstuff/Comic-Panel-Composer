# Comic Page Composer — High-Level Requirements

Version: 0.2
Date: 11 October 2026  
Subtitle: Comic page layout and lettering

## Purpose

Build a local Windows application for composing multi-page comics from existing pictures, editable panels, balloons, captions and lettering. Provide reusable page templates, panel layouts, styles, custom shapes, advanced fonts and publication output.

Initial image editing focuses on basic adjustments. Filters, background alpha, Drawing, line-art creation, advanced painting/healing, complex selections, Photoshop plug-ins and generative image features are outside the initial scope. Generative Image creation is out of scope for the project, however AI assistance of improving panels, layout, balloons, captions and lettering maybe be included in later stages.

This is the consolidated review draft from the previous requirements and feature list. The idea is to take what is liked from present tooling, and upgrade the behaviour offering more future capabilities, and grow it into a full blown Comic Page Composer. The primary purpose of the application is create replace Comic Life 3, since that hasn't been updated or enhanced for many years.

Requirements that are ticked have been validate as required.

## Platform, performance and scope

- [x] Initially target Windows 10+ as a local desktop application using local projects and assets; verify compatibility with the selected toolchain and renderer.
- [x] Prioritise the owner's Windows workflow and hardware. Support for other operating systems is deferred.
- [x] Use a performant compiled implementation suitable for interactive graphics editing.
- [x] Dragging, resizing, zooming and text editing responsive with large images and multi-page projects.
- [x] Run expensive image processing and exports without blocking normal interaction.
- [x] Support Windows display scaling with legible controls and crisp rendering.
- [x] Establish supported graphics hardware and minimum RAM, GPU memory and CPU requirements through representative tests. A dedicated graphics card is not yet a confirmed requirement.
- [x] Rust is the current language candidate; egui/eframe with a suitable renderer is one option, and C++/Qt remains an alternative. The final stack must prove its text, shape, editing and export capabilities.

## Projects and saving

- [x] Create a comic from a blank document or a designed template.
- [x] Save and reopen an editable project containing all pages, editable objects, styles, image assets/references and adjustment settings.
- [x] Autosave documents and recover interrupted work.
- [x] Revert a document to its saved state.
- [x] Save safely without corrupting the last valid project if a save fails.
- [x] Store author, title, issue, series and copyright information.
- [x] Store document keywords.
- [x] Check for application updates on launch.

## Pages and document layout

- [x] Add, insert and duplicate pages in a multi-page comic.
- [x] Delete pages with confirmation.
- [x] Choose a template page when adding a page.
- [x] Create a new page using the current page's panel layout.
- [x] Navigate and reorder pages by moving thumbnails up or down in the Pages panel, with visible page numbers, scrolling while moving pages and a clear selected-page highlight. The new sequence must determine document and export order.
- [x] Use preset or custom page sizes.
- [x] Choose page orientation.
- [x] Allow multiple orientations in the comic.
- [x] Allow for double page displays (full open pages)
- [x] Set a default page format.
- [x] Set left, right, top and bottom margins.
- [x] Configure facing pages.
- [x] Show two pages side by side.
- [x] Continue page numbering or specify a starting number.
- [x] Position and style page numbers, including inset and a caption-style backdrop.
- [x] Apply built-in or custom page textures.
- [x] Use a master page for repeating elements, with a per-page visibility setting.
- [x] Increase or decrease all balloon/caption text on a page at once.
- [x] Clear all balloon/caption text on a page.

## Templates, panel layouts and automatic composition

- [x] Browse a visual collection of comic templates.
- [x] Start from a blank template with or without predefined styles.
- [x] Use complete template pages with panels, balloons, captions and lettering.
- [x] Edit the content and styles supplied by a template.
- [x] Use templates with predefined image filters.
- [x] Create reusable comic templates.
- [x] Export templates for reuse on other machines
- [x] Load templates from saved file
- [x] Keep full-page templates, panel-only layouts and appearance presets as separate reusable concepts.
- [x] Use separate panel-only layout presets.
- [x] Create Separate panel-only layout presets.
- [x] Save Seperate panel-only layout presets.
- [x] Load Seperate panel-only layout presets
- [x] Browse built-in panel layouts by category.
- [x] Provide panel layouts with different counts, sizes and proportions, alongside saved user layouts.
- [x] Save the current panel arrangement and browse it as a reusable layout.
- [x] Resize layouts automatically to fit page margins.
- [x] Generate a grid page from row, column and gutter settings.
- [x] Automatically compose a comic from an album using a layout collection, random page arrangements, a stylesheet, page format and orientation.

## Workspace and navigation

- [x] Provide a clear central editing canvas with surrounding page, style, script and asset panels.
- [x] Add panels, captions, lettering and balloons from a visual element well.
- [x] Show recognisable element previews in a controller below or beside the canvas, including panels, captions, lettering and balloon kinds.
- [x] Hide or show the element well and other workspace sections.
- [x] Dock, undock, move and resize workspace panels, including inspectors, libraries and tool panels.
- [x] Float workspace panels as independent windows, including on other monitors and alongside other application windows.
- [x] Keep floating panels usable when moving between monitors with different DPI scaling; preserve correct text and control sizes, positions and hit targets.
- [x] Save and restore customised workspace layouts, provide named layout presets and allow restoration of the default layout.
- [x] Access save and print actions from the toolbar.
- [x] Customise toolbar contents.
- [x] Zoom in, zoom out and zoom to fit.
- [x] Pan around the editing canvas.
- [x] Work with multiple documents in separate windows or tabs.
- [x] Open multiple inspectors for document, page, script, graphic, shape, image, lettering, metrics and text settings.
- [x] Use full-screen mode.
- [x] Use dark mode.
- [ ] Choose from a built-in collection of application colour themes, with previews of the overall workspace appearance.
- [ ] Customise the colours of application windows, workspace panels, menus, toolbars and other interface controls.
- [ ] Apply and customise button themes/styles, including their normal, hovered, pressed, selected and disabled appearances, either as part of an application theme or independently.
- [ ] Save, load, import and export custom application and button themes so people can reuse and share them.

## Selecting, arranging and transforming objects

- [ ] Select several elements for common editing actions.
- [ ] Select the intended object reliably when objects overlap.
- [ ] Copy, paste, duplicate and delete selected elements.
- [ ] Undo and redo editing actions.
- [ ] Move and resize page elements.
- [ ] Set precise width, height and X/Y position numerically.
- [ ] Constrain proportions when resizing.
- [ ] Rotate elements interactively or numerically, with 90-degree snapping.
- [ ] Flip images horizontally or vertically.
- [ ] Lock elements or pin them to the page.
- [ ] Group elements, resize groups and duplicate groups.
- [ ] Overlap panels and page elements.
- [ ] Control stacking order: front, back, forward and backward, with keyboard shortcuts.
- [ ] Provide a Layers panel for each page that shows panels, images, balloons, captions, lettering and effects in their stacking order.
- [ ] Reorder objects by dragging in the Layers panel; select, show, hide, lock and unlock individual entries.
- [ ] Group objects in the Layers panel with expandable/collapsible groups, retaining independent editing where appropriate.
- [ ] Allow balloons, text, imagery and effects to be deliberately arranged in front of or behind each other, including balloons behind characters or other artwork.
- [ ] Preserve object stacking, grouping, visibility and locking when saving/reopening, and render visible objects in the same order on export.
- [ ] Align elements relative to a page or panel.
- [ ] Use rulers, ruler guides, smart alignment guides and spacing guides.
- [ ] Snap to a grid.
- [ ] Distribute elements evenly horizontally or vertically.
- [ ] Resize selected elements using a trackpad pinch.

## Panels and custom shapes

- [ ] Add empty image panels.
- [ ] Use predefined smart shapes.
- [ ] Use freeform shapes.
- [ ] Create and edit custom panel shapes with dynamically adjustable geometry.
- [ ] Freely reshape the outlines of all shape-based page objects, including panels, balloons, captions, text boxes, image frames, decorative shapes and other editable boxes; do not restrict customisation to a small set of fixed outlines or corner-radius options.
- [ ] Insert, remove and move outline points independently, with no fixed limit of three to eight corners for custom shapes; support both straight edges and Bézier curves with editable handles.
- [ ] Preserve editable text, images, fills, borders, effects, clipping and balloon tails as their enclosing shapes are changed, saved, reopened and exported.
- [ ] Make polygon panels with three to eight independently movable corners.
- [ ] Convert an irregular panel back to a bounding rectangle.
- [ ] Use frame-free or invisible panel styles.
- [ ] Apply caption/panel shape effects with adjustable strength (Factor).
- [ ] Combine and edit shapes to customise panel, balloon and caption outlines.
- [ ] Save and reuse custom shape presets.
- [ ] Move a panel and its contents as a unit.
- [ ] Control clipping to panel boundaries per element and by default; allow elements to extend outside panels.
- [ ] Allow balloons to overlap panel borders and protrude beyond a panel while remaining editable.
- [ ] Support fully visible balloons, panel-clipped balloons and balloons partially cut off at panel or page edges for intentional off-panel dialogue effects.

## Image libraries, placement and capture

- [ ] Browse image albums, collections and folders through named thumbnails with adjustable preview size.
- [ ] Drag images onto pages or panels from the built-in browser or operating system file browser.
- [ ] Reveal an external image's source file from its context menu.
- [ ] Browse PDF images in the media browser.
- [ ] Inspect image file information.
- [ ] Import pictures and preserve original image quality for export; choose supported input formats during detailed design.
- [ ] Fit a dropped image automatically to its panel.
- [ ] Reposition and resize images independently inside panels.
- [ ] Restore an image's original size.
- [ ] Place multiple independently transformed images in a panel, using a modifier key to add rather than replace.
- [ ] Replace a panel image by dropping another image onto it.
- [ ] Give images their own frames and shadows.
- [ ] Capture webcam pictures, drag frames from a live preview, and freeze or resume the preview.
- [ ] Offer camera-preview compatibility settings on Windows.
- [ ] Integrate with Apple's Photos library.
- [ ] Export comics to the Mac Photos application.
- [ ] Launch Photo Booth and drag captured pictures into the editor.
- [ ] Import photos from an iPad or iPhone.

## Image adjustments, filters and backgrounds

- [ ] Apply standard and advanced parametric comic-style filters, including newer filter effects.
- [ ] Choose filters through a visual selector with previews.
- [ ] Offer Rotoscope, Paintoscope, Hatched, Sketched/Sketch, Smooth, Pen and Retro filter presets.
- [ ] Adjust the selected filter's strength using Level.
- [ ] Adjust brightness, exposure, saturation, sharpness, contrast and blur.
- [ ] Adjust colour balance.
- [ ] Provide image-adjustment sliders, numeric inputs and step controls.
- [ ] Provide a dedicated Image Adjustment window or dockable/floating panel that brings image-adjustment and filter controls together, consistent with customisable workspace behaviour.
- [ ] Reset image adjustments.
- [ ] Keep image adjustments non-destructive and editable, retaining original image data while using suitable editing previews.
- [ ] Invert image colours.
- [ ] Colourise an image using a selected tint.
- [ ] Set image-filter processing resolution independently of export settings.
- [ ] Remove image backgrounds by selecting a colour and expanding the removal selection with an Instant Alpha tool.
- [ ] Combine a cut-out image with a different background.

## Balloons and dialogue

- [ ] Create speech, thought and whisper balloons.
- [ ] Create Exclamation and Exclamation 2 balloons.
- [ ] Create Rough and Rough 2 balloons.
- [ ] Create Robot, Square, Buzz and Space balloons.
- [ ] Create, save and reuse custom balloon shapes.
- [ ] Change balloon kind without losing its text.
- [ ] Adjust balloon shape using the Variety control.
- [ ] Reshape balloon bodies interactively by inserting, moving and removing outline points and editing straight or curved sections, without losing their text or tail connections.
- [ ] Add connected extension bubbles, allowing different kinds within an assembly.
- [ ] Connect bubbles with bendable tails.
- [ ] Add multiple tails.
- [ ] Adjust tail attachment points and tip positions precisely.
- [ ] Choose curved, straight, jagged or automatic tails.
- [ ] Enable automatic tail curvature.
- [ ] Choose thin, medium or thick tails.
- [ ] Customise balloon tail designs.
- [ ] Use a stroke-style balloon tail.
- [ ] Reverse tail connections in multi-bubble assemblies.
- [ ] Wrap a bubble around lettering using Envelop.
- [ ] Envelop or reshape balloon bodies around selected lettering, sound effects and other page objects while keeping the balloon, contained text and affected objects editable.
- [ ] Move a balloon body while leaving its tail tip anchored.
- [ ] Move a balloon and its tails together using a separate handle.
- [ ] Keep balloon bodies, tails and text editable after save/reopen, with clean joins between bodies and tails.

## Captions and text flow

- [ ] Add caption boxes.
- [ ] Edit caption and text-box shapes and appearance, including inserting and removing outline points to create irregular and curved boxes rather than only rectangles or preset rounded corners.
- [ ] Edit balloon and caption text in place.
- [ ] Grow balloons/captions to accommodate entered text.
- [ ] Add decorative initial capitals to captions.
- [ ] Connect captions into threads across a page.
- [ ] Offset text inside balloons and captions.
- [ ] Wrap text around captions and bubbles.
- [ ] Insert manual line breaks to shape dialogue inside a balloon.

## Fonts and ordinary text formatting

- [ ] Provide a collection of comic and creative fonts.
- [ ] Discover and offer the installed Windows font library.
- [ ] Maintain an application-only font library without system-wide installation.
- [ ] Support advanced font capabilities; confirm coverage of variable font axes, OpenType features, ligatures and kerning.
- [ ] Browse font families and typefaces.
- [ ] Preview the selected font.
- [ ] Choose preset or custom font sizes.
- [ ] Apply or revert font-panel changes.
- [ ] Increase or decrease text size with quick toolbar actions.
- [ ] Mix fonts and formatting within an editable text object, with dependable text selection and wrapping.
- [ ] Set text colour.
- [ ] Align text left, centre, right or justified.
- [ ] Align text vertically at the top, middle or bottom.
- [ ] Adjust character spacing.
- [ ] Adjust line spacing, including dialogue and captions.
- [ ] Adjust text inset margins.
- [ ] Apply strikethrough to text.
- [ ] Outline text glyphs.
- [ ] Find and replace text in scripts and across comic pages.
- [ ] Paste text while matching the destination style.
- [ ] Check spelling while typing.
- [ ] Offer spelling suggestions and replacement actions for misspelt words in balloons, captions, ordinary text and scripts.
- [ ] Allow adding words to a custom spelling dictionary, including character names and invented terms.
- [ ] Allow ignoring a spelling suggestion once or for the current document, and managing entries in the custom dictionary.
- [ ] Use accented letters and umlauts in decorative lettering.
- [ ] Paste characters from Windows Character Map.
- [ ] Preserve text layout consistently between editing, save/reopen and export.
- [ ] Report missing fonts and offer a clear replacement choice.

## Decorative titles and sound effects

- [ ] Create titles and sound-effect lettering.
- [ ] Provide a categorised SFX library with visual previews of ready-to-place sound-effect wording and editable lettering/effect presets.
- [ ] Drag an SFX entry from the library onto a comic page as editable lettering, allowing changes to its wording, font, shape and effects after placement.
- [ ] Create and save custom SFX entries, and organise them into reusable library categories.
- [ ] Edit the wording and font of lettering.
- [ ] Bend and warp lettering using Bezier controls.
- [ ] Scale lettering separately from warping it.
- [ ] Retain lettering distortion when changing its wording.
- [ ] Create multiline lettering with adjustable vertical spread.
- [ ] Place lettering along a path with adjustable horizontal shear.
- [ ] Adjust glyph spread.
- [ ] Adjust glyph skew.
- [ ] Adjust glyph stagger.
- [ ] Tilt glyphs.
- [ ] Join glyphs for a combined appearance.
- [ ] Fill individual glyphs or joined lettering with gradients.
- [ ] Apply 3D lettering with a selectable extrusion colour.
- [ ] Enable or disable 3D lighting and adjust its angle.
- [ ] Adjust 3D horizontal and vertical offset.
- [ ] Adjust 3D perspective and depth.
- [ ] Apply reusable lettering presets from a visual gallery that previews actual outlined, gradient and dimensional effects.

## Fills, colour and compositing

- [ ] Enable or disable solid-colour fills.
- [ ] Use gradient fills.
- [ ] Use image, tinted-image and colourised-image fills.
- [ ] Control fill-image size and proportions.
- [ ] Import a custom image as a fill.
- [ ] Use speedline fills.
- [ ] Use monochrome or full-colour halftone fills.
- [ ] Use texture and tiled fills.
- [ ] Control fill opacity.
- [ ] Choose colours using a colour wheel and brightness control, with numeric opacity.
- [ ] Control overall object opacity separately from fill and shadow opacity, including balloons and captions.
- [ ] Use composite blending.
- [ ] Apply shared fill, border, shadow and opacity controls to panels and shapes.

## Borders, outlines and shadows

- [ ] Enable or disable strokes.
- [ ] Choose clean or hand-drawn/brush-like stroke styles.
- [ ] Set stroke colour and width.
- [ ] Apply multiple outlines, controlling count and alternating colours, including on lettering.
- [ ] Use the Compact outline option; its precise behaviour still needs inspection.
- [ ] Customise line endpoints, including arrows and circles.
- [ ] Adjust the stroke miter limit.
- [ ] Enable or disable shadows.
- [ ] Set shadow colour.
- [ ] Position shadows with a visual pad or numeric X/Y offsets.
- [ ] Adjust shadow blur and opacity.

## Reusable styles

- [ ] Choose predefined styles or create custom element styles.
- [ ] Use styles consistently across pages.
- [ ] Copy and paste element styles.
- [ ] Update a named style and propagate changes to elements using it.
- [ ] Revert an element's style overrides.
- [ ] Save and load complete stylesheets independently of a comic.
- [ ] Set a default stylesheet for new comics.
- [ ] Define reusable default panel appearance, including fills, borders and other supported effects.

## Script writing and page composition

- [ ] Start with a blank script or a sample/tutorial script.
- [ ] Write, import or paste scripts in an integrated editor.
- [ ] Save a script-only document as a template.
- [ ] Recognise page, panel, caption, title, sound-effect and balloon keywords in scripts.
- [ ] Add, edit and remove script keywords.
- [ ] Map matching dialogue to a balloon kind.
- [ ] Assign an element style through the script.
- [ ] Show recognised element icons beside script text.
- [ ] Drag recognised script elements onto a page to create objects populated with the script text.
- [ ] Drop script text onto an existing caption to replace its text.
- [ ] Begin page composition without losing the existing script.
- [ ] Print a script.

## Exporting, printing and sharing

- [ ] Export a PDF.
- [ ] Export high-resolution page images.
- [ ] Export EPUB.
- [ ] Export CBZ.
- [ ] Limit PDF image resolution.
- [ ] Export all pages or a selected page range.
- [ ] Preview the first and last pages of an export range.
- [ ] Preserve DPI information in JPEG export.
- [ ] Preserve page order, dimensions, crops, text, shapes and effects consistently in output.
- [ ] Configure export image formats, quality, naming and packaging; final options are to be specified.
- [ ] Print comics with page-range, copy-count and two-sided options.
- [ ] Print using AirPrint.
- [ ] Print a low-resolution proof.
- [ ] Export an HTML comic presentation with a frame theme and JPEG-quality setting.
- [ ] Share by email.
- [ ] Share to Facebook; integration compatibility remains to be established.

## Interface languages

- [ ] Provide Dutch, French, German, Italian, Spanish and Portuguese localisation.

## Decisions to define by feature

### Platform and performance

- Select the language, GUI framework, renderer and text-layout approach.
- Set quantitative performance targets and representative document sizes.
- Decide whether the platform-specific photo, capture, printing and sharing integrations in this draft should be retained, replaced or deferred for Windows.

### Project files and asset libraries

- Decide whether projects package assets/fonts or link to them.
- Decide whether the application-only font library is global, per project, or both.
- Decide whether importing existing Comic Life projects is wanted; it is not yet a committed requirement.

### Pages, templates and the workspace

- Specify facing-page margins, spread behaviour and master-page editing.
- Specify texture management and document-keyword editing.
- Confirm click, placement-tool and drag interactions for the element palette, including the plus-marked balloon control.
- Validate floating, docking and workspace restoration behaviour, including mixed-DPI multi-monitor setups, against candidate Windows GUI frameworks.
- Define the theme system, the interface elements and button states it can style, and the format for sharing theme presets.
- Define Layers panel semantics for grouping, selection, clipping, object ownership and stacking order.

### Shapes, balloons and effects

- Define the custom-shape editing workflow and preset geometry, including inserting/removing outline points and mixing straight and curved sections across shape-based objects.
- Define how reshaping affects contained text, image clipping, balloon tails, fills and effects.
- Specify balloon Variety and automatic tail behaviour.
- Define whether balloon clipping follows the panel shape, page bounds or a user-adjustable mask, and how enveloping interacts with other objects.
- Define the Compact outline option and the three unlabeled stroke controls.
- Factor represents caption/panel effect strength. Filter Level represents filter strength; detailed parameter behaviour still needs defining.

### Text and fonts

- Confirm examples and exact coverage of dynamic fonts, variable axes and OpenType features.
- Specify text layout and formatting preservation when applying styles or changing decorative lettering.
- Define the SFX library's built-in categories and how custom entries are saved and organised.

### Images and output

- Select input/output formats and the final filter/preset catalogues.
- Specify image-adjustment ranges, units and Reset scope.
- Specify EPUB layout and reader targets, CBZ packaging, and PDF font/vector handling.
- Establish compatibility before retaining external sharing services.

## Proposed delivery sequence

This sequence is a proposal, not a reduction of the target scope.

1. **Technical proof:** an image clipped to an editable shaped panel, an overlapping balloon with an adjustable tail, editable text, save/reopen and a matching export. Evaluate typography and interaction alongside performance.
2. **Usable document editor:** multi-page management, page settings, ordinary panels/balloons/captions, font selection, undo/redo and dependable saving.
3. **Reusable composition:** templates, panel layouts, custom shapes, master pages, image collections and style libraries.
4. **Advanced presentation:** advanced font controls, decorative lettering, outlines, shadows, image adjustments and filters.
5. **Complete output workflow:** all target export formats, document metadata and agreed quality settings.

## High-level acceptance scenarios

- Compose a multi-page comic, move pages up and down in the Pages panel, and confirm that the document sequence follows the new order. Save, reopen and export it; retain that order and the editable page content.
- Save a custom panel layout and a styled page template; reuse both in another page without losing their intended geometry or appearance.
- Overlap differently shaped panels, adjust their image crops, and select/edit the intended object reliably.
- Create speech and thought balloons, customise a shape, add and move tails, and edit the text afterwards.
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

## References

- [Supplied legacy manual](https://plasq.com/helpfiles/Comic_LIfe_manual.pdf)
- [Comic Life 3 Getting Started guide](https://plasq.com/manuals/comiclife/3.0/comiclife-3.0-gettingstarted.pdf)
- [Comic Life 3 desktop features](https://plasq.com/apps/comiclife/macwin/features-galore/)
- [What's New in Comic Life 3](https://plasq.com/apps/comiclife/macwin/new-in-cl3/)
- [Comic Life 3 Fancy Fills tutorial](https://plasq.com/2014/12/fancy-fills/)
- [Comic Life 3 Windows release history](https://plasq.com/downloads/comiclife3/win/history/)
- [Comic Life 3 Mac release history](https://plasq.com/downloads/comiclife3/mac/history/)
- [Comic Life 4 product page](https://plasq.com/apps/comiclife4/mac/)
- [Comic Life version comparison](https://plasq.com/apps/comiclife4/why-upgrade-mac/)
