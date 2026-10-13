# Comic Panel Composer

*Comic page layout and lettering.*

A planned local Windows desktop application for arranging existing pictures into comic pages, with editable panels, speech and thought balloons, captions, decorative lettering and reusable templates.

The focus is comic composition rather than drawing or a full painting application.

## Comic Panel Composer at a glance

![Comic Panel Composer infographic showing the planned comic creation workflow and features](docs/Comic%20Panel%20Composer%20Infographic.png)

*Concept illustration of the product vision. Features shown are planned, not implemented.*

## Project status

Requirements and design stage. No application implementation has been added yet.

- Initial platform target: Windows 10 and later.
- Other operating systems are deferred.
- Rust is the current language candidate; the GUI framework and renderer remain to be selected and validated.
- Minimum hardware requirements will be established through testing.

## Development approach

The project is currently in research and exploration. We will investigate the foundations, agree a delivery plan, then build and test in manageable increments. Findings from prototypes and testing will feed back into the requirements and plan.

### 1. Investigate the foundations

Clarify the intended workflows, distinguish essential features from useful additions and deferred work, and investigate the technical options before committing to a stack. A feature in the requirements is not automatically part of the first release.

Use small, purposeful prototypes to test uncertain foundations, each with a clear question and acceptance criteria. The proposed first technical proof combines an image clipped to an editable shaped panel, an overlapping balloon with an adjustable tail, editable text, save/reopen and a matching export. Assess interaction, typography, performance and the underlying design; a convincing interface alone does not establish architectural quality.

### 2. Plan delivery

Agree the initial scope, architectural boundaries, delivery phases and acceptance criteria using the investigation results. Record significant decisions with their rationale, alternatives and consequences.

For each increment, define what will be delivered, how it will be verified, which checks can be automated, and what requires manual testing by the owner or pilot users. Keep the plan open to adjustment as evidence emerges.

### Engineering priorities

- **Maintainability:** use clear, understandable code and the simplest design that meets the agreed needs.
- **Supportability:** make failures diagnosable, protect saved projects and support recovery from interrupted work.
- **Testability:** keep core behaviour testable independently of the interface, with appropriate automated checks.
- **Extensibility:** maintain clear boundaries between the document model, editing operations, rendering, persistence and interface so features can evolve without unnecessary coupling.
- **Correctness and consistency:** preserve editable content and its intended appearance through editing, save/reopen and export.
- **Responsiveness:** verify performance with representative documents and hardware rather than assume minimum specifications.

Avoid speculative abstractions and unnecessary frameworks. Foundations should support concrete product needs and be demonstrated through evidence.

### 3. Build and test iteratively

Implement against the agreed plan in small, reviewable increments. Review the code and architectural fit alongside automated tests, manual workflow checks and pilot feedback. Verify important claims through direct results, and report unresolved limitations clearly.

Update requirements and design records when decisions change. Revisit foundations when testing exposes a problem rather than carrying an unverified assumption into later work.

### Direction and AI collaboration

The product owner retains responsibility for product direction, architecture and significant technology or scope decisions. AI contributes research, design alternatives, implementation and verification, and is expected to challenge assumptions where there is a sound reason.

Suggestions remain proposals until agreed. Once an implementation increment is authorised, AI should complete the agreed work and routine, reversible choices without repeatedly requesting confirmation.

## Requirements

See [High-Level Requirements](docs/HIGH_LEVEL_REQUIREMENTS.md) for the product scope and capability requirements.

Unresolved questions, provisional delivery ideas and example acceptance scenarios are captured separately in [Project Planning (DRAFT)](docs/PROJECT_PLANNING_DRAFT.md). That document is for information capture only; it is not an approved plan.
