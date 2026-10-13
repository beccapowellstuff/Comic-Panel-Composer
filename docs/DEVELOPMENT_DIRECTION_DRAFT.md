# Comic Panel Composer — Development Direction (DRAFT)

> **A suggested route, not a promised roadmap.** This is an easy-to-read outline of how we might investigate and build Comic Panel Composer. The stages, features, technology and release dates are **not yet agreed commitments**. We can change course or stop if the results do not meet expectations.

## The idea

We want an enjoyable way to **build comics from existing artwork**: arrange pages and panels, place images, add balloons and lettering, and export finished comics. We're **not** trying to build a drawing or painting application.

Rather than building hundreds of features at once, we'll work towards **a few meaningful results**. Each stage should teach us something, leave us with something we can demonstrate or evaluate, and help us decide what to do next.

## The proposed journey

### 1. Decide what matters first

**Goal:** Agree what a useful first version needs — and what can wait.

Pick a small set of realistic comic-creation tasks to use for testing. Identify the hardest technical questions and agree what a convincing result would look like.

**We finish this step with:** A proposed first-phase scope, sample comic material and clear ways to judge success.

### 2. Test the technology before committing to it

**Goal:** Find out whether Rust and the proposed graphics and interface tools are actually right for this application.

Make small, working experiments. Compare a Rust-based option with a credible alternative, such as C++/Qt, using the **same comic-editing challenge**.

The challenge should exercise real requirements: an irregular panel, an image clipped to the panel, artwork extending outside the border, an editable balloon and tail, readable text, smooth zooming and movement, and an export that looks like the canvas. Test Windows display scaling and relevant hardware too.

**We finish this step with:** Evidence about what works, what does not, and which technology is worth building on. **Rust is a candidate, not a decision.**

A pretty interface mockup does **not** count as a successful technical test.

### 3. Build the application's foundations

**Goal:** Create a solid base for the editor, without building every feature yet.

Set up the development project and automated checks. Keep the comic document, editing actions, screen rendering, project saving and user interface clearly separated.

Prove a small but complete workflow: edit an object, undo the change, save the project, reopen it, and get the intended appearance back. Test the JSON-based project format and separate image references without tying the editing model directly to the file format.

**We finish this step with:** A small working application foundation we can understand, test and build upon.

### 4. Create one complete comic page

**Goal:** Prove the foundations work together for a real creative task.

Import existing artwork, arrange a few editable panels, place and adjust images, add ordinary speech balloons and text, then save, reopen and export the page.

We don't need every advanced feature yet. We do need the whole journey to work reliably and **feel straightforward to use**.

**We finish this step with:** A real, editable comic page — not just a graphics demonstration.

### 5. Make the first genuinely usable comic editor

**Goal:** Grow from one page into a small comic that someone can realistically create and finish.

Bring together the essential multi-page workflow: page management, everyday panels and captions, basic lettering, undo/redo, dependable project handling and useful export. The exact feature set still needs to be agreed.

Test with realistic artwork and documents, not just ideal demo files.

**We finish this step with:** A practical editor we can try properly and potentially put in front of a small group of testers.

### 6. Review the results and choose what comes next

**Goal:** Decide whether this approach is delivering the quality and experience we want.

Look at usability, reliability, performance, code quality and feedback. Then make an explicit choice: **continue**, **improve what we have**, **change approach**, **pause** or **stop**.

A later stage might concentrate on advanced balloon shapes, decorative lettering, reusable layouts or vertical webcomics. We won't promise which one comes next before reviewing the evidence.

## What makes a stage successful?

We won't call something complete just because it runs or looks good in a screenshot. It needs to:

- **Do the agreed job** in a real comic-creation workflow.
- **Keep work safe**, including through saving, reopening and undo where applicable.
- **Behave and perform acceptably** on relevant Windows hardware.
- **Be understandable, maintainable and testable**, rather than relying on fragile fixes.
- **Meet the product owner's expectations** — not just be "good enough".

We'll use automated tests where they make sense, along with hands-on testing. A failed test or an awkward workflow is a reason to investigate, not something to hide.

## Experiments are not releases

A technology experiment, an engineering milestone and a public release are different things. **We won't release a new version merely because another button works.** Any release should offer a coherent, useful improvement.

There is **no guaranteed finished application, schedule or release**. This is an experiment, and taking the time to make sound decisions is part of the work.

## Ideas are welcome

Suggestions and feedback from other comic creators are welcome throughout the project. We'll consider them, but **adding an idea does not promise it will be implemented**, and an approved requirement does not automatically belong in the first version.

## Related documents

- [High-Level Requirements](HIGH_LEVEL_REQUIREMENTS.md) — what we'd like the application to be capable of.
- [Project Planning (DRAFT)](PROJECT_PLANNING_DRAFT.md) — collected open questions and earlier, provisional planning ideas. Neither document is an approved release plan.
