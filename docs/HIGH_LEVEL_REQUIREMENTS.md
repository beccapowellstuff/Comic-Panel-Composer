# Comic Panel Composer — High-Level Requirements

Version: 0.1 — requirements draft  
Date: 12 October 2026  
Application name: Comic Panel Composer
Subtitle: Comic page layout and lettering

## 1. Purpose and scope

Create a local Windows desktop application for composing comics from existing pictures, editable panels, lettering, captions and balloons. The intended workflow takes inspiration from Comic Life and the comic composition capabilities of Clip Studio Paint.

The application must support complete multi-page comics, reusable layouts and styles, custom shapes, advanced fonts and export. Initial image editing is limited to basic adjustments and filters. A comprehensive Photoshop-style painting, retouching or image manipulation suite is outside the initial scope.

This document consolidates the agreed product direction and high-level requirements. It describes the intended product, not functionality already implemented. All listed functional areas remain in the target scope; a delivery sequence does not remove later features.

## 2. Platform and performance

| ID | Requirement |
| --- | --- |
| PL-01 | Initially target Windows 10 and later as a local desktop application with local project files and assets. Earlier Windows versions are outside the target. Compatibility must be verified with the selected toolchain and renderer. A browser-based interface is not the chosen direction. |
| PL-02 | Use a performant compiled implementation suitable for responsive graphics editing. |
| PL-03 | Keep dragging, resizing, zooming and text editing responsive while working with large images and multi-page documents. |
| PL-04 | Perform expensive image processing and exports without blocking normal interface interaction. |
| PL-05 | Support normal Windows display scaling with legible controls and crisp page rendering. |
| PL-06 | Prioritise the owner's Windows workflow and hardware for initial development. Support for other operating systems is deferred. Graphics compatibility and minimum RAM, GPU memory and CPU requirements will be established through testing; a dedicated graphics card is not yet a confirmed requirement. |

**Technology direction:** Rust is the current candidate, not a final selection. Rust with egui/eframe and a suitable graphics renderer is an option to evaluate. C++/Qt remains an alternative. Framework selection must demonstrate the required text, shape, editing and export capabilities; language choice alone does not establish performance.

## 3. Documents and pages

| ID | Requirement |
| --- | --- |
| DOC-01 | Create multi-page comic documents; add, insert, duplicate and remove pages. |
| DOC-02 | Reorder pages directly in the Pages panel by moving page thumbnails up or down. The resulting panel order determines the document's page sequence and export order. Dragging thumbnails is a proposed interaction; the exact gesture is not yet confirmed. |
| DOC-03 | Show a scrollable page thumbnail list with page numbers and a clear current-page selection. |
| DOC-04 | Configure page size and document margins: left, right, top and bottom. |
| DOC-05 | Support facing-page documents. Exact margin and spread behaviour remains open. |
| DOC-06 | Continue page numbering from the preceding page or specify a starting number. |
| DOC-07 | Support page textures and master-page elements, with control over whether master elements are drawn on a page. |
| DOC-08 | Store author, title, issue, series and copyright metadata. Support keywords; their interface remains open. |
| DOC-09 | Configure the resolution used for image filters independently of export settings. |

## 4. Templates, layouts and reusable styles

| ID | Requirement |
| --- | --- |
| LIB-01 | Create, save and reuse full-page templates containing layouts and styling. |
| LIB-02 | Create, save and reuse panel layouts independently of full-page templates. Allow saving the current page's panel arrangement as a layout. |
| LIB-03 | Define reusable default panel appearance, including borders, fills and other supported styling. |
| LIB-04 | Provide visual thumbnail galleries for page templates and panel layouts, with selectable collections and built-in layouts. |
| LIB-05 | Include panel arrangements with different counts, sizes and proportions; retain user-created arrangements. |
| LIB-06 | Provide reusable lettering and appearance presets with previews showing their actual effects. |
| LIB-07 | Provide an image library with selectable albums/collections, named thumbnails and scrolling. |
| LIB-08 | Save and reuse custom balloon and shape presets. |

A **page template** may include styled page content. A **panel layout** describes panel arrangement. A **style preset** describes appearance. They should be distinct concepts even where they work together.

## 5. Panels, shapes and object editing

| ID | Requirement |
| --- | --- |
| OBJ-01 | Create and edit panels with standard and custom shapes, including dynamically adjustable geometry. |
| OBJ-02 | Allow overlapping panels and other page objects, with controllable stacking order. |
| OBJ-03 | Place images inside panels and crop/clip them to the panel shape. Reposition and scale the image independently of the panel. |
| OBJ-04 | Move, resize and rotate page objects using direct manipulation and numeric controls. |
| OBJ-05 | Expose width, height, X/Y position and rotation; support constrained proportions and restoring an applicable object's original size. |
| OBJ-06 | Provide a shape selector for captions and panels. Rectangle is verified; additional shapes are to be defined. |
| OBJ-07 | Support selecting overlapping objects reliably, multi-selection, grouping, duplication and deletion. |
| OBJ-08 | Provide canvas zooming and panning. Alignment, distribution, guides and snapping are proposed editing aids. |
| OBJ-09 | Provide a visual element-selection controller/palette beside the editing canvas. Let the user choose an element type through recognisable previews, including a panel, caption/text box, decorative lettering, and different balloon shapes. The reference presents a horizontal palette below the canvas. |

The screenshot's caption/panel **Factor** control and two metrics-panel icon controls remain open; no behaviour is assumed for them.

## 6. Balloons, thoughts and captions

| ID | Requirement |
| --- | --- |
| BAL-01 | Create speech balloons, thought balloons, captions and custom balloon shapes. |
| BAL-02 | Provide reference kinds: Speech, Thought, Exclamation, Exclamation 2, Whisper, Rough, Rough 2, Robot, Square, Buzz and Space. Their precise geometry must be specified. |
| BAL-03 | Offer adjustable shape variation for balloon kinds, corresponding to the reference Variety control. |
| BAL-04 | Create and edit tails with Curved, Straight, Jagged and Auto modes, plus an Auto-curve option. Exact automatic behaviour remains open. |
| BAL-05 | Offer Thin, Medium and Thick tail widths and an Add Tail action. |
| BAL-06 | Allow multiple tails and direct adjustment of their attachment and tip positions. |
| BAL-07 | Keep balloon shapes, tails and text editable after creation and reopening a project. Maintain clean joins between balloon bodies and tails. |

## 7. Text and fonts

| ID | Requirement |
| --- | --- |
| TXT-01 | Create editable text for dialogue, captions, titles and sound effects. |
| TXT-02 | Discover and offer the installed Windows font library. |
| TXT-03 | Support an application-only font library without requiring system-wide installation. |
| TXT-04 | Support advanced font capabilities. Variable font axes, OpenType features, ligatures and kerning are candidate interpretations of the user's request for dynamic fonts; confirm the exact coverage. |
| TXT-05 | Provide font-family and typeface selection, preset and custom font sizes, live previews, and apply/revert actions. |
| TXT-06 | Support text colour and left, centre, right and justified alignment. |
| TXT-07 | Support top, middle and bottom vertical alignment, line spacing and text inset margins. |
| TXT-08 | Support mixed formatting within a text object, dependable wrapping and text selection/editing. |
| TXT-09 | Preserve text layout consistently between editing, saving/reopening and export. Report missing fonts with a clear replacement choice. |

Ordinary paragraph text and decorative lettering share font handling but require different editing capabilities. Ordinary text must remain practical for dialogue; decorative effects must not prevent later wording changes.

## 8. Decorative lettering and visual appearance

| ID | Requirement |
| --- | --- |
| FX-01 | Enable/disable object fills; select fill colour and fill opacity. |
| FX-02 | Enable/disable strokes; control stroke style, colour and width. |
| FX-03 | Support outlines, including an outline-count control and outline colour. Clarify the reference Compact option and other stroke controls. |
| FX-04 | Provide endpoint styles where applicable. |
| FX-05 | Enable/disable shadows; control colour, X/Y offset, blur and opacity. |
| FX-06 | Control overall object opacity separately from fill and shadow opacity. |
| FX-07 | Provide colour selection with a colour wheel, brightness and opacity controls, including numeric opacity input. |
| FX-08 | Support decorative glyph controls: Joined, Spread, Skew and Stagger. Define their exact effects during detailed design. |
| FX-09 | Support optional dimensional lettering with colour, lighting angle, X/Y offset, perspective and depth controls. |
| FX-10 | Support reusable lettering styles, including the outlined, gradient and dimensional appearances visible in the style gallery. |

## 9. Images, adjustments and filters

| ID | Requirement |
| --- | --- |
| IMG-01 | Import existing pictures and preserve their original quality for export. Supported input formats remain to be selected. |
| IMG-02 | Provide brightness, exposure, saturation, sharpness, contrast and blur adjustments. |
| IMG-03 | Provide colour balance adjustments. |
| IMG-04 | Offer adjustment sliders alongside numeric inputs with step controls, plus a Reset action. |
| IMG-05 | Provide an image-filter selector with preview, Invert, optional Colorize with colour selection, and a Level control. The filter catalogue and Level semantics remain open. |
| IMG-06 | Keep adjustments non-destructive and editable; use suitable previews while retaining original image data. |

Advanced painting, healing, complex selections, Photoshop plug-ins and generative image features are not initial requirements.

## 10. Saving and export

| ID | Requirement |
| --- | --- |
| OUT-01 | Save and reopen editable projects containing all pages, objects, styles, image references/assets and adjustment settings. |
| OUT-02 | Provide undo/redo and autosave/recovery. Save reliably without corrupting the last valid project on failure. |
| OUT-03 | Export PDF, images, EPUB and CBZ. |
| OUT-04 | For PDF, configure an image-resolution limit; choose all pages or a page range; preview the range's first and last pages. |
| OUT-05 | Produce output consistent with the document's page order, dimensions, image crops, text, shapes and effects. |
| OUT-06 | Define export quality, image formats, naming and packaging options for images, EPUB and CBZ during detailed requirements. |

## 11. Open questions

1. Measured minimum hardware targets. Initial OS target: Windows 10 and later; other operating systems are deferred. Confirm compatible graphics hardware after selecting and testing the renderer.
2. Final language, GUI framework, renderer and text-layout approach; Rust is a candidate.
3. Exact meaning and expected examples of dynamic fonts; required variable/OpenType features.
4. Whether app-only fonts are global to the app, per project, or both; whether projects package assets/fonts or link to them.
5. Custom shape editing workflow and the exact meaning of balloon Variety, automatic tails and caption/panel Factor.
6. Stroke Compact behaviour, the three unlabeled stroke controls, and the two metrics icon controls.
7. Master-page editing, facing-page margins, page textures and keyword management.
8. Filter catalogue, filter Level behaviour, numeric ranges/units and Reset scope.
9. Supported input/output image formats, EPUB layout/reader targets, CBZ packaging and PDF font/vector handling.
10. Whether importing existing Comic Life projects is wanted. It is not currently a committed requirement.
11. Quantitative performance targets and representative document sizes.
12. Element-palette interaction: whether choosing an icon creates an element immediately, activates a placement tool, or supports dragging onto the page. The screenshot establishes the palette and choices, not the placement gesture or the meaning of its plus-marked balloon icon.

## 12. Proposed delivery sequence

This sequence is a proposal, not a reduction of the target scope.

1. **Technical proof:** an image clipped to an editable shaped panel, an overlapping balloon with an adjustable tail, editable text, save/reopen and a matching export. Evaluate typography and interaction alongside performance.
2. **Usable document editor:** multi-page management, page settings, ordinary panels/balloons/captions, font selection, undo/redo and dependable saving.
3. **Reusable composition:** templates, panel layouts, custom shapes, master pages, image collections and style libraries.
4. **Advanced presentation:** advanced font controls, decorative lettering, outlines, shadows, image adjustments and filters.
5. **Complete output workflow:** all target export formats, document metadata and agreed quality settings.

## 13. High-level acceptance scenarios

- Compose a multi-page comic, move pages up and down in the Pages panel, and confirm that the document sequence follows the new order. Save, reopen and export it; retain that order and the editable page content.
- Save a custom panel layout and a styled page template; reuse both in another page without losing their intended geometry or appearance.
- Overlap differently shaped panels, adjust their image crops, and select/edit the intended object reliably.
- Create speech and thought balloons, customise a shape, add and move tails, and edit the text afterwards.
- Select installed and app-only fonts, apply supported advanced font features, and retain the intended appearance in export.
- Apply borders, outlines, shadows and decorative lettering while retaining editable source objects.
- Adjust image appearance, reset adjustments and reopen the project without losing access to the original image.
- Export the complete comic and a selected page range in the applicable formats, preserving page order and matching the edited layout.
