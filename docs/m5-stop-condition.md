# M5 — Stop Condition

M5 stops when the Reader Workspace is representative enough to judge honestly without introducing a document or interaction subsystem.

Required end state:

- Reader outline content is authored fixture data.
- Reader document title/paragraphs are authored fixture data.
- Reader settings use existing property fixture content.
- Reader status uses existing text fixture content.
- selected outline state is authored and validated.
- empty Reader state is authored explicitly.
- the egui backend no longer checks Reader fixture ids/states to invent document state.
- structural and visual Reader previews remain layout-correct under M4.
- existing Project Browser and Dependency Workbench fixture behavior remains intact.

Once those conditions hold, stop.

Do not continue into:

- rich text
- scrolling
- pagination
- tree expansion state
- interaction/event systems
- TTS semantics
- application data binding
- visual-palette redesign
- accessibility ontology
- ViewWitness integration

Human visual QA after M5 determines whether the dark Reader palette itself has earned a later visual-legibility milestone.