# Comic Panel Composer

*Comic page layout and lettering — an experimental project.*

Comic Panel Composer is a proposed Windows desktop application for **building comics from existing artwork**. The aim is to retain the straightforward comic-creation experience of tools such as Comic Life while offering more control over pages, panels, image placement, layers, speech and thought balloons, captions, lettering, effects and publishing formats, including vertical-scroll webcomics.

This is a **comic composition tool**, not a drawing, painting or inking application.

## The vision

![Comic Panel Composer infographic illustrating the planned comic creation workflow and features](docs/Comic%20Panel%20Composer%20Infographic.png)

*Product-vision illustration, not a screenshot of working software. The features shown are ideas and requirements, not promises of what will be delivered.*
*More details of the requirements can be found at [High-Level Requirements](docs/HIGH_LEVEL_REQUIREMENTS.md)*

## Current status: research and requirements

**There is no working application or release to download yet.**

This project is an experiment to discover whether a carefully directed, AI-assisted software development approach can produce an application that meets the product owner's expectations for usability, functionality and engineering quality.

**There is no guaranteed release, delivery date or commitment to implement every requirement.** This will take time. A feature being recorded or approved as desirable does not mean it belongs in the first phase or will necessarily be built.

The aim is not to settle for something that merely looks impressive or is "good enough". Progress will be assessed against agreed behaviour and quality expectations. If the results do not meet those expectations, work may be rethought, repeated, paused or stopped.

## How the project will be approached

### 1. Prove the technology can meet our needs

Use a few challenging requirements to test the possible graphics, interface, text and export technologies through small, purposeful prototypes. Investigate Rust alongside credible alternatives before choosing a technology stack. These are technical experiments, not the application framework or finished features.

### 2. Decide the first feature phase and establish the foundations

Use the investigation results to prioritise a manageable first set of features, with clear success criteria; defer the rest rather than promising everything at once.

Then establish the real application foundation, keeping the document model, editing operations, rendering, persistence and interface clearly separated. Use the simplest maintainable design that meets the proven needs, and verify it through tests rather than appearance alone.

### 3. Build the first phase and test it

Implement the agreed first phase in manageable increments. Review the code and design, use appropriate automated tests, and perform manual workflow testing. Check that the application behaves correctly with realistic comic projects, not just demonstration examples.

A feature is not accepted merely because it appears to work. It must meet the agreed requirements and the product owner's quality expectations.

### 4. Review the results and decide what comes next

Assess what has actually been achieved, what works well, what needs improvement and whether continuing makes sense. The outcome may be another phase, rework, a change of direction, a pause **or a decision to stop**. No future phase is guaranteed.

### 5. Welcome suggestions throughout — without making promises

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
