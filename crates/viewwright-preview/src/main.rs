use eframe::egui;
use viewwright_model::{parse_and_resolve, ResolvedBlueprint};

fn main() -> eframe::Result<()> {
    let blueprints = vec![
        parse_and_resolve(include_str!("../../../examples/project-browser.toml"))
            .expect("project specimen must be valid"),
        parse_and_resolve(include_str!("../../../specimens/reader-workspace.toml"))
            .expect("reader specimen must be valid"),
        parse_and_resolve(include_str!(
            "../../../specimens/reader-workspace-visual.toml"
        ))
        .expect("visual reader specimen must be valid"),
        parse_and_resolve(include_str!("../../../specimens/dependency-workbench.toml"))
            .expect("dependency workbench specimen must be valid"),
        parse_and_resolve(include_str!(
            "../../../specimens/density-pressure-comfortable.toml"
        ))
        .expect("comfortable density pressure specimen must be valid"),
        parse_and_resolve(include_str!(
            "../../../specimens/density-pressure-dense.toml"
        ))
        .expect("dense density pressure specimen must be valid"),
        parse_and_resolve(include_str!(
            "../../../specimens/overlay-command-palette-pressure.toml"
        ))
        .expect("overlay command palette pressure specimen must be valid"),
        parse_and_resolve(include_str!(
            "../../../specimens/reader-overflow-pressure.toml"
        ))
        .expect("reader overflow pressure specimen must be valid"),
    ];
    for b in &blueprints {
        println!("{}\n{}", b.semantic_tree(), viewwright_ascii::render(b));
        if b.visual.is_some() {
            println!("{}", viewwright_concept::render(b));
        }
    }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1440.0, 900.0]),
        ..Default::default()
    };
    eframe::run_native(
        "ViewWright Preview",
        options,
        Box::new(move |_| {
            Ok(Box::new(App {
                blueprints,
                screen: 2,
                fixture: 0,
                last_action: None,
                render_state: viewwright_egui::RenderState::default(),
            }))
        }),
    )
}

struct App {
    blueprints: Vec<ResolvedBlueprint>,
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
            ui.heading("ViewWright Preview");
            ui.label(&self.blueprints[self.screen].screen.purpose);
            if let Some(audit) = viewwright_audit::audit(&self.blueprints[self.screen]) {
                ui.collapsing(
                    format!("Visual audit: {} warning(s)", audit.findings.len()),
                    |ui| {
                        ui.monospace(audit.summary());
                    },
                );
            }
            if let Some(event) = &self.last_action {
                ui.label(format!(
                    "Last action: {} ({})",
                    event.action, event.element_id
                ));
            }
        });
        let mut selected_screen = None;
        let mut selected_fixture = None;
        egui::Panel::bottom("fixtures").show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Specimen:");
                for (i, b) in self.blueprints.iter().enumerate() {
                    if ui
                        .selectable_label(i == self.screen, &b.screen.id)
                        .clicked()
                    {
                        selected_screen = Some(i);
                    }
                }
            });
            ui.horizontal_wrapped(|ui| {
                ui.label("Fixture:");
                for (i, f) in self.blueprints[self.screen].fixtures.iter().enumerate() {
                    if ui.selectable_label(i == self.fixture, &f.id).clicked() {
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
        let b = &self.blueprints[self.screen];
        let fixture = b
            .fixtures
            .get(self.fixture)
            .map(|f| f.id.as_str())
            .unwrap_or("default");
        let output = viewwright_egui::show(ui, b, fixture, &mut self.render_state);
        if let Some(event) = output.activation {
            self.last_action = Some(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::App;

    #[test]
    fn specimen_and_fixture_selection_clear_search_state() {
        let mut app = App {
            blueprints: Vec::new(),
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
}
