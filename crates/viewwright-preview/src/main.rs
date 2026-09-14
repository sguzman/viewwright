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
            }))
        }),
    )
}

struct App {
    blueprints: Vec<ResolvedBlueprint>,
    screen: usize,
    fixture: usize,
    last_action: Option<viewwright_egui::InteractionEvent>,
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::TopBottomPanel::top("preview-host").show(ctx, |ui| {
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
        egui::TopBottomPanel::bottom("fixtures").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Specimen:");
                for (i, b) in self.blueprints.iter().enumerate() {
                    if ui
                        .selectable_label(i == self.screen, &b.screen.id)
                        .clicked()
                    {
                        self.screen = i;
                        self.fixture = 0;
                        self.last_action = None;
                    }
                }
                ui.separator();
                ui.label("Fixture:");
                for (i, f) in self.blueprints[self.screen].fixtures.iter().enumerate() {
                    if ui.selectable_label(i == self.fixture, &f.id).clicked() {
                        self.fixture = i;
                        self.last_action = None;
                    }
                }
            });
        });
        let b = &self.blueprints[self.screen];
        let fixture = b
            .fixtures
            .get(self.fixture)
            .map(|f| f.id.as_str())
            .unwrap_or("default");
        let output = viewwright_egui::show(ctx, b, fixture);
        if let Some(event) = output.activation {
            self.last_action = Some(event);
        }
    }
}
