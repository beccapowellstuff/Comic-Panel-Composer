# Comic Panel Composer

*Comic page layout and lettering — an experimental project.*

**In plain English:** Bring your existing artwork, arrange it into comic panels, add speech bubbles, captions and sound effects, then turn it into comic pages or a scrolling webcomic. Think of it as putting a comic together, **not** drawing or painting the pictures.

Comic Panel Composer aims to make that process easy and enjoyable on Windows, while giving creators room to customise their layouts and lettering.

## The vision

![Comic Panel Composer infographic illustrating the planned comic creation workflow and features](docs/Comic%20Panel%20Composer%20Infographic.png)

*Product-vision illustration, not a screenshot of working software. The features shown are ideas and requirements, not promises of what will be delivered.*
*More details of the requirements can be found at [High-Level Requirements](docs/HIGH_LEVEL_REQUIREMENTS.md)*

## Current work item — Stage 1 of 6

### 🧪 Prove the technology

| | |
|---|---|
| **Status** | **Stage 1 experiments underway** — an editable-polygon PoC and a native Windows docking-workspace PoC are in the repository. The docking interactions have been tested by the product owner; there is no finished comic editor or approved technology stack. |
| **Goal** | Find out whether Rust and the available graphics, interface and text tools can support the comic editor we want. |
| **What we'll test** | Custom panel shapes, images inside and beyond panel borders, speech balloons, text, smooth editing and accurate export. |
| **Next stage** | Use what we've learnt to choose the features for the first build. |

**What needs to happen before this stage is complete:**

- [ ] Build small working experiments, including a Rust option and a credible alternative.
- [ ] Check the difficult editing, rendering, text, Windows scaling and export behaviours.
- [ ] Record the results and decide whether to proceed with a technology, investigate further or change direction.

**This is an experiment, not a release promise.** Source-only technology PoCs are available in the repository, but there is no usable comic editor release, guaranteed delivery date or commitment to build every requested feature. If the results don't meet our expectations, we'll improve the approach, change direction, pause or stop.

**Research so far:** [Polygon editor PoC](proof-of-concepts/rust-polygon-editor/README.md) demonstrates basic editable geometry in an independent experiment. The [Windows docking PoC](proof-of-concepts/docking-workspace/README.md) has owner-tested floating tool windows, internal docking, mixed-DPI movement using an experimental pinned winit patch, window-position restoration, menu-driven native tab-group merging and focus behaviour. **Direct tab dragging between separate native Windows windows did not work in owner testing.** See the [docking technology findings](docs/DOCKING_WORKSPACE_POC_FINDINGS.md) for verified results, limitations and remaining risks. The next proposed research is actual comic-page geometry, rich lettering and matching export. Rust/egui is still an evaluation option, not the selected product stack.

## How the project will be approached

![Comic Panel Composer direction infographic illustrating the planned delivery approach](docs/Comic%20Panel%20Composer%20Direction%20Infograph.png)
*Directly vision of how we are going to approach development, and direction*
*More details of the plan can be found at [Development Direction](docs/DEVELOPMENT_DIRECTION_DRAFT.md)*

## Welcome suggestions throughout but not making promises

Ideas, feedback and feature suggestions are welcome at any stage, especially from people who create comics. Suggestions will be considered against the product's purpose, priorities, complexity and technical findings.

**A suggestion is not a commitment.** There is no promise that it will be added to the requirements, implemented, or delivered by a particular date.

## Engineering principles and AI collaboration

The product owner directs the product, architecture, priorities and acceptance decisions. AI is used to help research options, challenge assumptions, implement agreed tasks and verify results — **not** to decide independently what the application should become.

The main engineering priorities are:

- **Maintainability and extensibility:** clear, simple designs that can be understood, tested and changed.
- **Reliability and supportability:** protect project files, handle errors clearly and recover safely from failures.
- **Testability:** automated checks where useful, alongside real editing and export workflows.
- **Correctness:** preserve editable content and visual results across editing, saving, reopening and exporting.
- **Responsiveness:** test performance on representative projects and hardware rather than assuming it will be fast.

A convincing interface or a large amount of AI-generated code is not proof of a sound application. The foundations must hold up as the software grows.

## Platform and technology

- **Initial target:** Windows 10 and later, subject to confirming compatibility with the selected technology.
- **Other operating systems:** deferred.
- **Language and framework:** not yet decided. Rust is a candidate, not an approved choice; the user interface and rendering technologies must be investigated and tested.
- **Minimum hardware requirements:** to be established through testing.

## Project documents

- [Development Direction (DRAFT)](docs/DEVELOPMENT_DIRECTION_DRAFT.md) — a friendly overview of the proposed investigation and development stages. This is a direction for discussion, not a release promise.
- [High-Level Requirements](docs/HIGH_LEVEL_REQUIREMENTS.md) — the detailed product requirements and intended capabilities. Ticked requirements have been validated as wanted; they are **not** completed features or first-release promises.
- [Project Planning (DRAFT)](docs/PROJECT_PLANNING_DRAFT.md) — captured questions, provisional sequencing ideas and example acceptance scenarios. This is **not** an approved delivery plan.
- [Docking Workspace PoC Findings](docs/DOCKING_WORKSPACE_POC_FINDINGS.md) — observed Windows behaviour, CI evidence, known failures and remaining technology risks.
