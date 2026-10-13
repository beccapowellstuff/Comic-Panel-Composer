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
- [x] Set a full-page background image or fill independently of individual comic panels, with editable positioning, sizing, cropping and appearance; the background remains visible in any uncovered areas or gutters.
- [x] Show optional, non-exported print-layout guides for safe/live area, trim boundaries and bleed, with configurable bleed dimensions distinct from ordinary page margins.
- [x] Use a master page for repeating elements, with a per-page visibility setting.
- [x] Increase or decrease all balloon/caption text on a page at once.
- [x] Clear all balloon/caption text on a page.

## Creator notes and optional annotated output

- [x] Attach editable private creator notes to individual pages and panels, separate from captions, dialogue, script text and editorial notes intended to appear in the comic.
- [x] Show unobtrusive note markers directly over the relevant page or panel while editing (such as a quotation-mark or speech-bubble symbol); select a marker to read, edit or remove its note without changing the comic artwork.
- [x] Provide a clear Show/Hide Notes toggle that hides both the on-page markers and any expanded notes when off, without deleting notes or affecting the underlying page, panel or export.
- [x] Allow note markers to be repositioned to avoid covering important artwork while remaining associated with their page or panel.
- [x] Preserve creator notes, marker positions and page/panel associations through project save/reopen, page and panel rearrangement, and duplication, without silently losing or detaching notes.
- [x] Always save creator notes inside the editable project; exclude their text, markers and metadata from ordinary print, export and publishing output unless the creator explicitly requests an annotated copy.
- [x] Offer a clearly labelled Include Notes option for printing and supported saved/exported output formats; leave it unticked by default for each output operation, so publishing a comic never accidentally reveals private notes.
- [x] When Include Notes is selected, preview an annotated output showing readable notes as well as their markers, with suitable placement to limit obstruction of comic artwork; preserve the normal artwork-only output as a separate choice.
- [x] Allow individual notes to be expanded or collapsed independently. When collapsed, only their note marker remains visible. Expanding or minimising one note must not affect other notes.

## Webcomics and vertical-scroll composition

- [x] Create and edit vertical-scrolling webcomics as a native document format alongside conventional fixed-page comics, without forcing the scrolling composition into fixed print-page boundaries.
- [x] Set a webcomic's working width and adjust its vertical extent as content is added, supporting long compositions without requiring the entire strip to be handled as one enormous bitmap.
- [x] Organise webcomic content into ordered episodes/chapters and manageable editable sections while presenting the episode as a continuous scrolling composition.
- [x] Insert, duplicate, resize, split, move and reorder sections or sequences of panels without flattening or losing editable content.
- [x] Freely position, resize, overlap and arrange panels, balloons, captions, decorative lettering and other objects along a vertical strip, including full-width impact panels and elements positioned between panels.
- [x] Adjust vertical gutters and deliberate empty space between panels or scenes independently, including large gaps for storytelling pace and transitions.
- [x] Place backgrounds, fills and visual motifs behind or across multiple vertically arranged panels and gutters, with normal layer order and clipping controls.
- [x] Provide templates and reusable layouts designed for vertically scrolling comics, rather than relying solely on traditional page templates.
- [x] Preview a webcomic as a continuously scrolling reader would see it, including a configurable phone-width/viewport preview and optional guides for legibility, spacing and safe placement.
- [x] Recompose existing fixed-page comic content into a separate editable vertical-scroll layout, retaining the original artwork and page layout while allowing independent panel positions, crops, balloons and lettering adjustments; do not require an automatic conversion to be perfect.
- [x] Keep webcomic episode/section order and reading flow independent of export-image slice boundaries, preserving all editable objects when saving and reopening.

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
- [x] Choose from a built-in collection of application colour themes, with previews of the overall workspace appearance.
- [x] Customise the colours of application windows, workspace panels, menus, toolbars and other interface controls.
- [x] Apply and customise button themes/styles, including their normal, hovered, pressed, selected and disabled appearances, either as part of an application theme or independently.
- [x] Save, load, import and export custom application and button themes so people can reuse and share them.

## Selecting, arranging and transforming objects

- [x] Select several elements for common editing actions.
- [x] Select the intended object reliably when objects overlap.
- [x] Copy, paste, duplicate and delete selected elements.
- [x] Undo and redo editing actions.
- [x] Move and resize page elements.
- [x] Set precise width, height and X/Y position numerically.
- [x] Constrain proportions when resizing.
- [x] Rotate elements interactively or numerically, with 90-degree snapping.
- [x] Flip images horizontally or vertically.
- [x] Lock elements or pin them to the page.
- [x] Group elements, resize groups and duplicate groups.
- [x] Overlap panels and page elements.
- [x] Control stacking order: front, back, forward and backward, with keyboard shortcuts.
- [x] Provide a Layers panel for each page that shows panels, images, balloons, captions, lettering and effects in their stacking order.
- [x] Reorder objects by dragging in the Layers panel; select, show, hide, lock and unlock individual entries.
- [x] Group objects in the Layers panel with expandable/collapsible groups, retaining independent editing where appropriate.
- [x] Allow balloons, text, imagery and effects to be deliberately arranged in front of or behind each other, including balloons behind characters or other artwork.
- [x] Preserve object stacking, grouping, visibility and locking when saving/reopening, and render visible objects in the same order on export.
- [x] Align elements relative to a page or panel.
- [x] Use rulers, ruler guides, smart alignment guides and spacing guides.
- [x] Offer optional lettering placement guides for speech-tail aiming and near-touching visual tangents between balloons, text, panel borders and other objects, without imposing the suggested layout.
- [x] Snap to a grid.
- [x] Distribute elements evenly horizontally or vertically.
- [x] Resize selected elements using a trackpad pinch.

## Panels and custom shapes

- [x] Add empty image panels.
- [x] Use predefined smart shapes.
- [x] Use freeform shapes.
- [x] Create and edit custom panel shapes with dynamically adjustable geometry.
- [x] Freely reshape the outlines of all shape-based page objects, including panels, balloons, captions, text boxes, image frames, decorative shapes and other editable boxes; do not restrict customisation to a small set of fixed outlines or corner-radius options.
- [x] Insert, remove and move outline points independently, with no fixed limit of three to eight corners for custom shapes; support both straight edges and Bézier curves with editable handles.
- [x] Preserve editable text, images, fills, borders, effects, clipping and balloon tails as their enclosing shapes are changed, saved, reopened and exported.
- [x] Make polygon panels with three to eight independently movable corners.
- [x] Convert an irregular panel back to a bounding rectangle.
- [x] Use frame-free or invisible panel styles.
- [x] Apply caption/panel shape effects with adjustable strength (Factor).
- [x] Combine and edit shapes to customise panel, balloon and caption outlines.
- [x] Apply non-destructive vector Boolean operations to user-created shapes: Combine/Union (including seamless visual merging), Subtract, Intersect and Exclude, producing correct visible outlines without unwanted internal strokes.
- [x] Retain the original shapes as editable components of a compound shape; move, rotate, resize and reshape each component, add or remove components, and change the operation without destroying the original geometry.
- [x] Allow the combined result to be moved and transformed as a single page object, while retaining independent access to its constituent shapes and their relative positions.
- [x] Update the compound shape's outline, fills, cut-outs and borders as component shapes or operations change, consistently in the canvas, saved project and export.
- [x] Save and reuse custom shape presets.
- [x] Move a panel and its contents as a unit.
- [x] Provide explicit editing modes to move, resize, rotate and otherwise transform the panel frame alone, its contained artwork alone, or the frame and contents together; preserve the chosen content crop and placement when editing the frame only.
- [x] Associate each panel with an editable collection of images and other page objects, allowing selection and editing of its contents independently, per-object clipping to the panel shape, and deliberate overflow beyond the frame while retaining the panel relationship.
- [x] Remove a panel frame or border without deleting its associated artwork and other objects, keeping that content editable on the page; alternatively allow intentional deletion of both the panel and its contents, with undo/redo support.
- [x] Place smaller inset panels, framed images and other editable objects above a larger background panel or page background without merging their artwork; preserve independent crops, borders, clipping and stacking order.
- [x] Leave panel gutters and other unoccupied page areas transparent to underlying page artwork when desired, with no forced white fill; allow captions, balloons, sound effects and other objects to occupy or cross gutter space.
- [x] Support page-edge, borderless and full-bleed/open panels which can extend into a configured bleed area, with predictable trimming and clipping in print and digital exports.
- [x] Divide an existing editable panel into two or more independently editable panels using straight or angled cuts, with adjustable gutters; preserve the existing placed image and allow its assignment/crop in the resulting panels to be edited without data loss.
- [x] Offer equal row/column division and manual cuts, with a clear choice to start resulting panels empty, duplicate existing content, or reuse the original source artwork with independently editable positioning and crops; do not silently discard or flatten content.
- [x] Adjust horizontal, vertical and irregular gutters between adjacent panels, including zero gaps; optionally keep adjoining frame boundaries aligned when resizing, without preventing deliberate overlaps or unusual spacing.
- [x] Control clipping to panel boundaries per element and by default; allow elements to extend outside panels.
- [x] Let images, characters, props, leaves, particles and other artwork or effects remain associated with a panel while selected portions extend beyond its frame, without requiring a destructive cut-out of the panel image.
- [x] Render a panel's border behind selected breakout artwork while retaining visible border segments elsewhere, including where an element crosses the border into the gutter.
- [x] Control panel-border breakout behaviour for individual objects or edges without disabling the entire frame, and retain independent editing, clipping and stacking of the panel, border and breakout objects.
- [x] Allow balloons to overlap panel borders and protrude beyond a panel while remaining editable.
- [x] Support fully visible balloons, panel-clipped balloons and balloons partially cut off at panel or page edges for intentional off-panel dialogue effects.

## Image libraries, placement and capture

- [x] Browse image albums, collections and folders through named thumbnails with adjustable preview size.
- [x] Drag images onto pages or panels from the built-in browser or operating system file browser.
- [x] Reveal an external image's source file from its context menu.
- [x] Browse PDF images in the media browser.
- [x] Inspect image file information.
- [x] Import pictures and preserve original image quality for export; choose supported input formats during detailed design.
- [x] Fit a dropped image automatically to its panel.
- [x] Reposition and resize images independently inside panels.
- [x] Restore an image's original size.
- [x] Place multiple independently transformed images in a panel, using a modifier key to add rather than replace.
- [x] Replace a panel image by dropping another image onto it.
- [x] Give images their own frames and shadows.
- [x] Capture webcam pictures, drag frames from a live preview, and freeze or resume the preview.
- [x] Offer camera-preview compatibility settings on Windows.
- [x] Integrate with Apple's Photos library.
- [x] Export comics to the Mac Photos application.
- [x] Launch Photo Booth and drag captured pictures into the editor.
- [x] Import photos from an iPad or iPhone.

## Image adjustments, filters and backgrounds

- [x] Apply standard and advanced parametric comic-style filters, including newer filter effects.
- [x] Choose filters through a visual selector with previews.
- [x] Offer Rotoscope, Paintoscope, Hatched, Sketched/Sketch, Smooth, Pen and Retro filter presets.
- [x] Adjust the selected filter's strength using Level.
- [x] Adjust brightness, exposure, saturation, sharpness, contrast and blur.
- [x] Adjust colour balance.
- [x] Provide image-adjustment sliders, numeric inputs and step controls.
- [x] Provide a dedicated Image Adjustment window or dockable/floating panel that brings image-adjustment and filter controls together, consistent with customisable workspace behaviour.
- [x] Reset image adjustments.
- [x] Keep image adjustments non-destructive and editable, retaining original image data while using suitable editing previews.
- [x] Invert image colours.
- [x] Colourise an image using a selected tint.
- [x] Set image-filter processing resolution independently of export settings.
- [x] Remove image backgrounds by selecting a colour and expanding the removal selection with an Instant Alpha tool.
- [x] Combine a cut-out image with a different background.

## Balloons and dialogue

- [x] Create speech, thought and whisper balloons.
- [x] Offer editable presets for comic-lettering conventions including burst/shout balloons, wavy or distressed speech balloons, telepathic balloons and radio/electric/broadcast balloons; allow the user to override the preset shape and styling.
- [x] Provide thought-balloon tails with an editable default chain of spaced, decreasing-size circles, and whispered-dialogue styles including dashed or muted outlines.
- [x] Independently edit every thought-tail bubble's size, shape (including circles, ovals and custom outlines), position, fill and stroke; add, remove and reorder bubbles without regenerating the entire tail.
- [x] Adjust the gaps, direction and progression between individual thought-tail bubbles, allowing uneven spacing or irregular arrangements without automatically changing neighbouring bubbles.
- [x] Keep individually edited thought-tail bubbles associated with the parent balloon while allowing independent selection and editing, with their appearance and relationships retained after save/reopen and export.
- [x] Insert, position and style editable breath marks (small dash clusters) around dialogue or as standalone closing marks, including use in distressed and telepathic lettering.
- [x] Create tail-free telepathic balloons with independently adjustable irregular or overlapping wavy outlines, optional additional contours and editable breath marks on selected sides or corners; retain full shape and decoration editing after applying a preset.
- [x] Provide electric/radio balloon tail styles, including editable lightning-bolt paths, and small burst accents where a shout balloon tail joins the body.
- [x] Create double-outline balloons with independently adjustable outline spacing, colours, weights and choice of which outline the tail joins.
- [x] Create Exclamation and Exclamation 2 balloons.
- [x] Create Rough and Rough 2 balloons.
- [x] Create Robot, Square, Buzz and Space balloons.
- [x] Create, save and reuse custom balloon shapes.
- [x] Change balloon kind without losing its text.
- [x] Adjust balloon shape using the Variety control.
- [x] Reshape balloon bodies interactively by inserting, moving and removing outline points and editing straight or curved sections, without losing their text or tail connections.
- [x] Add connected extension bubbles, allowing different kinds within an assembly.
- [x] Join adjacent balloons directly as a visually continuous assembly or link them with editable curved/straight connectors; independently reposition component balloons and connectors without unwanted internal outlines.
- [x] Connect bubbles with bendable tails.
- [x] Add multiple tails.
- [x] Draw, add and remove custom tails or point/curve-defined extensions on balloons, captions and other suitable shape-based objects, rather than being limited to predefined tails.
- [x] Join an added tail or extension seamlessly to its parent shape without an internal outline at the attachment, while retaining independent editing of the extension's points, curves and attachment.
- [x] Adjust tail attachment points and tip positions precisely.
- [x] Adjust a balloon tail's width at the attachment, along its length and at its tip independently of its stroke thickness, with point/curve controls for its taper; narrowing the tail must not unintentionally collapse it into a single thin line.
- [x] Shape and resize tails with independently movable edges or equivalent width controls, including asymmetric tapers and curves, without breaking the seamless join to the balloon.
- [x] Attach editable squinks (small multi-pointed bursts), breath-mark-inspired bursts and other origin markers to tail endpoints; let users move, resize and reshape the marker and connected tail independently.
- [x] Make off-panel dialogue tails terminate at a panel border or an off-panel origin using editable straight, curved, arcing or S-shaped paths.
- [x] Choose curved, straight, jagged or automatic tails.
- [x] Enable automatic tail curvature.
- [x] Choose thin, medium or thick tails.
- [x] Customise balloon tail designs.
- [x] Use a stroke-style balloon tail.
- [x] Reverse tail connections in multi-bubble assemblies.
- [x] Wrap a bubble around lettering using Envelop.
- [x] Envelop or reshape balloon bodies around selected lettering, sound effects and other page objects while keeping the balloon, contained text and affected objects editable.
- [x] Move a balloon body while leaving its tail tip anchored.
- [x] Move a balloon and its tails together using a separate handle.
- [x] Keep balloon bodies, tails and text editable after save/reopen, with clean joins between bodies and tails.
- [x] Butt balloons flat against panel/page borders with selectable edge alignment and clipping, rather than only overlapping or hiding part of a rounded balloon.
- [x] Support breaking panel borders, where a balloon's fill visually continues into a gutter or interrupts a panel border, while keeping the relevant shapes, fills and clipping editable.

## Captions and text flow

- [x] Add caption boxes.
- [x] Offer optional caption style presets for location/time, internal monologue, off-panel spoken dialogue and editorial notes, with independently editable wording, typography and shape.
- [x] Create editable caption notes or editorial references marked with asterisks, including placement of their related note text elsewhere on the page.
- [x] Edit caption and text-box shapes and appearance, including inserting and removing outline points to create irregular and curved boxes rather than only rectangles or preset rounded corners.
- [x] Edit balloon and caption text in place.
- [x] Grow balloons/captions to accommodate entered text.
- [x] Add decorative initial capitals to captions.
- [x] Connect captions into threads across a page.
- [x] Offset text inside balloons and captions.
- [x] Wrap text around captions and bubbles.
- [x] Insert manual line breaks to shape dialogue inside a balloon.

## Fonts and ordinary text formatting

- [x] Provide a collection of comic and creative fonts (this is a "if possible" because I can not supply fonts which are not okay ok to share)
- [x] Discover and offer the installed Windows font library.
- [x] Maintain an application-only font library without system-wide installation.
- [x] Support advanced font capabilities; confirm coverage of variable font axes, OpenType features, ligatures and kerning.
- [x] Browse font families and typefaces.
- [x] Preview the selected font.
- [x] Choose preset or custom font sizes.
- [x] Apply or revert font-panel changes.
- [x] Increase or decrease text size with quick toolbar actions.
- [x] Mix fonts and formatting within an editable text object, with dependable text selection and wrapping.
- [x] Apply bold italic, italics, emphasis, underlining, mixed type sizes and other character-level formatting within a balloon or caption, including oversized emphasis words and deliberately small dialogue inside a large balloon.
- [x] Select comic-font glyph alternatives where available, including the crossbar capital I used as a personal pronoun, without forcing an automatic substitution where the font or lettering style differs.
- [x] Set text colour.
- [x] Align text left, centre, right or justified.
- [x] Align text vertically at the top, middle or bottom.
- [x] Adjust character spacing.
- [x] Adjust line spacing, including dialogue and captions.
- [x] Adjust text inset margins.
- [x] Apply strikethrough to text.
- [x] Outline text glyphs.
- [x] Find and replace text in scripts and across comic pages.
- [x] Paste text while matching the destination style.
- [x] Allow authors to enter and edit comic-lettering punctuation and notation, including double dashes, ellipses, combined question/exclamation marks, editorial asterisks, quotation marks, foreign-language delimiters and musical notes, without automatic rewriting to another convention.
- [x] Control manual line breaks, word spacing and hyphenation in balloon and caption text, with the option to avoid unwanted automatic word splits.
- [x] Support editable music/singing notation and placing selected ordinary dialogue text along a curved or wavy baseline inside balloons.
- [x] Check spelling while typing.
- [x] Offer spelling suggestions and replacement actions for misspelt words in balloons, captions, ordinary text and scripts.
- [x] Allow adding words to a custom spelling dictionary, including character names and invented terms.
- [x] Allow ignoring a spelling suggestion once or for the current document, and managing entries in the custom dictionary.
- [x] Use accented letters and umlauts in decorative lettering.
- [x] Paste characters from Windows Character Map.
- [x] Preserve text layout consistently between editing, save/reopen and export.
- [x] Report missing fonts and offer a clear replacement choice.

## Decorative titles and sound effects

- [x] Create titles and sound-effect lettering.
- [x] Provide a categorised SFX library with visual previews of ready-to-place sound-effect wording and editable lettering/effect presets.
- [x] Drag an SFX entry from the library onto a comic page as editable lettering, allowing changes to its wording, font, shape and effects after placement.
- [x] Create and save custom SFX entries, and organise them into reusable library categories.
- [x] Edit the wording and font of lettering.
- [x] Bend and warp lettering using Bezier controls.
- [x] Scale lettering separately from warping it.
- [x] Retain lettering distortion when changing its wording.
- [x] Create multiline lettering with adjustable vertical spread.
- [x] Place lettering along a path with adjustable horizontal shear.
- [x] Adjust glyph spread.
- [x] Adjust glyph skew.
- [x] Adjust glyph stagger.
- [x] Tilt glyphs.
- [x] Join glyphs for a combined appearance.
- [x] Fill individual glyphs or joined lettering with gradients.
- [x] Apply 3D lettering with a selectable extrusion colour.
- [x] Enable or disable 3D lighting and adjust its angle.
- [x] Adjust 3D horizontal and vertical offset.
- [x] Adjust 3D perspective and depth.
- [x] Apply reusable lettering presets from a visual gallery that previews actual outlined, gradient and dimensional effects.
- [x] Create hollow sound-effect lettering with an outlined glyph and transparent interior, allowing underlying artwork to remain visible.
- [x] Apply adjustable non-destructive distressed/eroded details to decorative letters and sound effects (including the comic-lettering 'roach chew' look) without losing the editable wording and styling.

## Fills, colour and compositing

- [x] Enable or disable solid-colour fills.
- [x] Use gradient fills.
- [x] Use image, tinted-image and colourised-image fills.
- [x] Control fill-image size and proportions.
- [x] Import a custom image as a fill.
- [x] Use speedline fills.
- [x] Use monochrome or full-colour halftone fills.
- [x] Use texture and tiled fills.
- [x] Control fill opacity.
- [x] Apply a non-destructive gradient transparency/alpha mask (or equivalent editable fade) to images, page backgrounds, fills and other suitable visual objects, revealing underlying layers progressively instead of using only uniform opacity.
- [x] Edit gradient transparency direction, start/end positions, opacity stops and transition softness/feathering, including top-to-bottom and bottom-to-top fades; combine the fade with object clipping, layers and existing compositing without flattening its source.
- [x] Save, reopen and export gradient transparency with the same visual result while retaining editable fade controls in the project.
- [x] Choose colours using a colour wheel and brightness control, with numeric opacity.
- [x] Control overall object opacity separately from fill and shadow opacity, including balloons and captions.
- [x] Use composite blending.
- [x] Apply shared fill, border, shadow and opacity controls to panels and shapes.

## Borders, outlines and shadows

- [x] Enable or disable strokes.
- [x] Choose from multiple clean or hand-drawn/brush-like stroke styles.
- [x] Set stroke colour and width.
- [x] Apply multiple outlines, controlling count and alternating colours, including on lettering.
- [x] Use the Compact outline option; its precise behaviour still needs inspection.
- [x] Customise line endpoints, including arrows and circles.
- [x] Adjust the stroke miter limit.
- [x] Enable or disable shadows.
- [x] Set shadow colour.
- [x] Position shadows with a visual pad or numeric X/Y offsets.
- [x] Adjust shadow blur and opacity.

## Reusable styles

- [x] Choose predefined styles or create custom element styles.
- [x] Use styles consistently across pages, but can be changed at individual pages if needed.
- [x] Copy and paste element styles.
- [x] Update a named style and propagate changes to elements using it.
- [x] Revert an element's style overrides.
- [x] Save and load complete stylesheets independently of a comic.
- [x] Set a default stylesheet for new comics.
- [x] Define reusable default panel appearance, including fills, borders and other supported effects.

## Script writing and page composition

- [x] Start with a blank script or a sample/tutorial script.
- [x] Write, import or paste scripts in an integrated editor.
- [x] Save a script-only document as a template.
- [x] Recognise page, panel, caption, title, sound-effect and balloon keywords in scripts.
- [x] Add, edit and remove script keywords.
- [x] Map matching dialogue to a balloon kind.
- [x] Assign an element style through the script.
- [x] Show recognised element icons beside script text.
- [x] Drag recognised script elements onto a page to create objects populated with the script text.
- [x] Drop script text onto an existing caption to replace its text.
- [x] Begin page composition without losing the existing script.
- [x] Print a script.
- [x] Export script to standard formats that can be loaded by other systems.
- [x] Import script from standard formats that can be loaded from other systems.
- [x] When a script is changed (for example dialogue) include functionality to update dialogue in the comic.

## Exporting, printing and sharing

- [x] Export a PDF with selectable DPI.
- [x] Export high-resolution page images with selectable DPI.
- [x] Export EPUB with selectable DPI.
- [x] Export CBZ with selectable DPI.
- [x] Export all pages or a selected page range.
- [x] Preview the first and last pages of an export range.
- [x] Preserve DPI information in JPEG export.
- [x] Preserve page order, dimensions, crops, text, shapes and effects consistently in output.
- [x] Configure export image formats, quality, naming and packaging; final options are to be specified.
- [x] Export vertical-scroll webcomic episodes as a continuous image where supported or as an ordered set of sequential images, with selectable output width, format, quality and naming.
- [x] Automatically split a scrolling composition into consecutive image slices according to configurable maximum dimensions and file-size limits, keeping adjacent slices visually seamless and in the intended reading order.
- [x] Preview and, where possible, adjust proposed webcomic export slice boundaries to avoid awkward cuts through dialogue or important artwork, with clear warnings when destination limits prevent a preferred split.
- [x] Offer editable publishing-format presets for common webcomic services, including WEBTOON CANVAS and Tapas, and allow custom profiles; validate exported dimensions, formats and file sizes against the selected profile without hard-coding a platform's rules into the document.
- [x] Preview webcomic export at its intended display width so creators can assess text readability, panel spacing and visual continuity before generating publishing files.
- [x] Print comics with page-range, copy-count and two-sided options.
- ~~[ ] Print using AirPrint.~~
- [x] Print a low-resolution proof.
- [x] Export an HTML comic presentation with a frame theme and JPEG-quality setting.

## Interface languages

- ~~[ ] Provide Dutch, French, German, Italian, Spanish and Portuguese localisation.~~

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
- [Blambot Comic Book Grammar & Tradition](https://blambot.com/pages/comic-book-grammar-tradition) — examples of visual lettering conventions and editorial practices; not mandatory style rules.
- [Clip Studio Paint Frames and Panels manual](https://help.clip-studio.com/en-us/manual_en/540_comic/Frames_and_Panels.htm) — reference workflows for frame borders, masks, frame division and editing.
- [Clip Studio Tips: How to Panel Comics for WEBTOON and Print](https://tips.clip-studio.com/en-us/articles/9420) — creator examples of page-wide background bleeds, open panels and layout/lettering in gutters, plus vertical-scroll and hybrid formatting workflows.
- [WEBTOON CANVAS announcement: Uploading Long Episode Images](https://www.webtoons.com/en/notice/detail?noticeNo=1766) — official description of slicing oversized uploads.
- [Tapas: How to Publish a Comic Episode](https://help.tapas.io/hc/en-us/articles/1260802028970-Series-Basics-How-to-publish-a-comic-episode-on-Tapas) — official upload dimensions and file-format guidance.
- [MediBang Paint: Settings for Comic Guidelines](https://medibangpaint.com/en/use/2023/11/setting-draft/) — reference for print trim, safe area and bleed guides.
