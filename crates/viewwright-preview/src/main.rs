use eframe::egui;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use viewwright_model::{parse_and_resolve, ResolvedBlueprint};

struct Demo {
    blueprint: ResolvedBlueprint,
    title: String,
    source: PathBuf,
}

fn demo_sources() -> Vec<PathBuf> {
    let mut args = std::env::args().skip(1);
    let mut sources = Vec::new();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--demos" => match args.next() {
                Some(path) => sources.push(PathBuf::from(path)),
                None => {
                    eprintln!("--demos requires a file or directory path");
                    std::process::exit(2);
                }
            },
            "--help" | "-h" => {
                println!(
                    "ViewWright demo gallery\nUsage: viewwright-preview [--demos PATH]...\n\nWithout arguments, loads ./demos recursively.\nEach --demos path may be a TOML file or a directory.\nBlueprints remain fixtures/designs, not live application backends."
                );
                std::process::exit(0);
            }
            _ => {
                eprintln!("Unknown argument: {arg}\nUse --help for usage.");
                std::process::exit(2);
            }
        }
    }
    if sources.is_empty() {
        sources.push(PathBuf::from("demos"));
    }
    sources
}

fn collect_toml_files(path: &Path, files: &mut Vec<PathBuf>, errors: &mut Vec<String>) {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) => {
            errors.push(format!("{}: {error}", path.display()));
            return;
        }
    };
    if metadata.file_type().is_symlink() {
        errors.push(format!("{}: symlink skipped", path.display()));
        return;
    }
    if metadata.is_file() {
        if path.extension().is_some_and(|extension| extension == "toml") {
            files.push(path.to_path_buf());
        } else {
            errors.push(format!("{}: expected a .toml file", path.display()));
        }
        return;
    }
    if !metadata.is_dir() {
        errors.push(format!("{}: not a file or directory", path.display()));
        return;
    }
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(error) => {
            errors.push(format!("{}: {error}", path.display()));
            return;
        }
    };
    for entry in entries {
        match entry {
            Ok(entry) => {
                let name = entry.file_name();
                if name == ".git" || name == "target" {
                    continue;
                }
                let child = entry.path();
                if child.is_dir()
                    || child.extension().is_some_and(|extension| extension == "toml")
                {
                    collect_toml_files(&child, files, errors);
                }
            }
            Err(error) => errors.push(format!("{}: {error}", path.display())),
        }
    }
}

fn load_demos(sources: &[PathBuf]) -> (Vec<Demo>, Vec<String>) {
    let mut files = Vec::new();
    let mut errors = Vec::new();
    for source in sources {
        collect_toml_files(source, &mut files, &mut errors);
    }
    files.sort();
    files.dedup();
    let mut demos = Vec::new();
    let mut ids = BTreeSet::new();
    for path in files {
        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(error) => {
                errors.push(format!("{}: {error}", path.display()));
                continue;
            }
        };
        match parse_and_resolve(&content) {
            Ok(blueprint) => {
                if !ids.insert(blueprint.screen.id.clone()) {
                    errors.push(format!(
                        "{}: duplicate screen.id {}; skipped (IDs must be unique across demo roots)",
                        path.display(),
                        blueprint.screen.id
                    ));
                    continue;
                }
                let group = path
                    .parent()
                    .and_then(Path::file_name)
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "demo".to_string());
                let title = format!("{group} / {}", blueprint.screen.id);
                demos.push(Demo {
                    blueprint,
                    title,
                    source: path,
                });
            }
            Err(error) => errors.push(format!("{}: {error}", path.display())),
        }
    }
    (demos, errors)
}

fn main() -> eframe::Result<()> {
    let (demos, load_errors) = load_demos(&demo_sources());
    for error in &load_errors {
        eprintln!("ViewWright demo load: {error}");
    }
    println!("ViewWright loaded {} demo blueprint(s)", demos.len());
    for demo in &demos {
        println!("{} ({})", demo.title, demo.source.display());
    }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1440.0, 900.0]),
        ..Default::default()
    };
    eframe::run_native(
        "ViewWright Demo Gallery",
        options,
        Box::new(move |_| {
            Ok(Box::new(App {
                demos,
                load_errors,
                demo_search: String::new(),
                screen: 0,
                fixture: 0,
                last_action: None,
                render_state: viewwright_egui::RenderState::default(),
            }))
        }),
    )
}

struct App {
    demos: Vec<Demo>,
    load_errors: Vec<String>,
    demo_search: String,
    screen: usize,
    fixture: usize,
    last_action: Option<viewwright_egui::InteractionEvent>,
    render_state: viewwright_egui::RenderState,
}

impl App {
    fn select_screen(&mut self, screen: usize) {
        self.screen = screen;
        self.fixture = 0;
        self.last_action = None;
        self.render_state.clear();
    }

    fn select_fixture(&mut self, fixture: usize) {
        self.fixture = fixture;
        self.last_action = None;
        self.render_state.clear();
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        egui::Panel::top("preview-host").show(ui, |ui| {
            ui.heading("ViewWright Demo Gallery");
            if !self.load_errors.is_empty() {
                ui.collapsing(
                    format!("Load problems ({})", self.load_errors.len()),
                    |ui| {
                        for error in &self.load_errors {
                            ui.label(error);
                        }
                    },
                );
            }
            if let Some(demo) = self.demos.get(self.screen) {
                let b = &demo.blueprint;
                ui.label(format!("{} — {}", demo.title, b.screen.purpose));
                ui.small(demo.source.display().to_string());
                if let Some(variant) = b.responsive_variant(ui.available_width()) {
                    ui.label(format!("Responsive variant: {}", variant.id));
                }
                if let Some(audit) = viewwright_audit::audit(b) {
                    ui.collapsing(
                        format!("Visual audit: {} warning(s)", audit.findings.len()),
                        |ui| ui.monospace(audit.summary()),
                    );
                }
                if let Some(event) = &self.last_action {
                    let value = event.value.as_ref().map(|value| match value {
                        viewwright_egui::TypedValue::Choice(value) => format!("Choice({value:?})"),
                        viewwright_egui::TypedValue::Boolean(value) => format!("Boolean({value})"),
                        viewwright_egui::TypedValue::Scalar(value) => format!("Scalar({value})"),
                    });
                    ui.label(format!(
                        "Last action: {} ({}){}",
                        event.action,
                        event.element_id,
                        value.map(|value| format!(" — {value}")).unwrap_or_default()
                    ));
                }
            } else {
                ui.label(
                    "No valid demos loaded. Pass --demos PATH or add TOML files under ./demos.",
                );
            }
        });
        if self.demos.is_empty() {
            return;
        }
        let mut selected_screen = None;
        let mut selected_fixture = None;
        egui::Panel::bottom("demo-gallery").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Find demo:");
                ui.text_edit_singleline(&mut self.demo_search);
                ui.label(format!("{} loaded", self.demos.len()));
            });
            let search = self.demo_search.to_lowercase();
            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.horizontal(|ui| {
                    for (i, demo) in self.demos.iter().enumerate() {
                        if !demo.title.to_lowercase().contains(&search) {
                            continue;
                        }
                        if ui
                            .selectable_label(i == self.screen, &demo.title)
                            .clicked()
                        {
                            selected_screen = Some(i);
                        }
                    }
                });
            });
            ui.horizontal_wrapped(|ui| {
                ui.label("State:");
                for (i, fixture) in self.demos[self.screen]
                    .blueprint
                    .fixtures
                    .iter()
                    .enumerate()
                {
                    if ui.selectable_label(i == self.fixture, &fixture.id).clicked() {
                        selected_fixture = Some(i);
                    }
                }
            });
        });
        if let Some(screen) = selected_screen {
            self.select_screen(screen);
        } else if let Some(fixture) = selected_fixture {
            self.select_fixture(fixture);
        }
        let b = &self.demos[self.screen].blueprint;
        let fixture = b
            .fixtures
            .get(self.fixture)
            .map(|fixture| fixture.id.as_str())
            .unwrap_or("default");
        let output = viewwright_egui::show(ui, b, fixture, &mut self.render_state);
        if let Some(event) = output.activation {
            self.last_action = Some(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{load_demos, App};
    use std::path::PathBuf;

    #[test]
    fn specimen_and_fixture_selection_clear_search_state() {
        let mut app = App {
            demos: Vec::new(),
            load_errors: Vec::new(),
            demo_search: String::new(),
            screen: 0,
            fixture: 0,
            last_action: None,
            render_state: viewwright_egui::RenderState::default(),
        };
        app.render_state
            .search_value_mut("project_search")
            .push_str("abc");
        app.select_screen(1);
        assert_eq!(app.render_state.search_value_mut("project_search"), "");
        app.render_state
            .search_value_mut("project_search")
            .push_str("xyz");
        app.select_fixture(2);
        assert_eq!(app.render_state.search_value_mut("project_search"), "");
    }

    #[test]
    fn project_demos_have_different_composition_architectures() {
        use viewwright_model::CompositionChild;

        fn children(
            blueprint: &viewwright_model::ResolvedBlueprint,
            composition_id: &str,
        ) -> Vec<String> {
            let composition = blueprint
                .compositions
                .iter()
                .find(|composition| composition.id == composition_id)
                .expect("expected composition");
            composition
                .children
                .iter()
                .map(|child| match child {
                    CompositionChild::Region(id) | CompositionChild::Composition(id) => id.clone(),
                })
                .collect()
        }

        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../demos");
        let (demos, errors) = load_demos(&[path]);
        assert!(errors.is_empty(), "demo load errors: {errors:#?}");

        let find = |id: &str| {
            &demos
                .iter()
                .find(|demo| demo.blueprint.screen.id == id)
                .expect("missing project demo")
                .blueprint
        };
        let chat = find("chatarium_session_observability");
        let calendar = find("ephemeris_calendar_workbench");
        let game = find("starbyte_emulation_library");

        assert_eq!(
            children(chat, &chat.root),
            ["commands", "body", "telemetry", "status"]
        );
        assert_eq!(children(chat, "body"), ["history", "transcript"]);
        assert_eq!(
            children(calendar, &calendar.root),
            ["toolbar", "calendars", "week_board", "event_inspector", "sync_status"]
        );
        assert_eq!(
            children(calendar, "week_board"),
            ["day_mon", "day_tue", "day_wed", "day_thu", "day_fri"]
        );
        assert_eq!(
            children(game, &game.root),
            ["toolbar", "systems", "games", "game_information", "runtime"]
        );
        assert_eq!(children(game, "game_information"), ["details", "cheats"]);
    }

    #[test]
    fn repository_demo_packs_parse_and_have_unique_ids() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../demos");
        let (demos, errors) = load_demos(&[path]);
        assert!(errors.is_empty(), "demo load errors: {errors:#?}");
        assert!(demos.len() >= 3, "expected three project examples");
        assert!(demos.iter().all(|demo| !demo.blueprint.fixtures.is_empty()));
    }
}
