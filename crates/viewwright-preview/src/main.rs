use eframe::egui;
use viewwright_model::{parse_and_resolve, ResolvedBlueprint};

fn main() -> eframe::Result<()> {
    let blueprints = vec![
        parse_and_resolve(include_str!("../../../examples/project-browser.toml"))
            .expect("project specimen must be valid"),
        parse_and_resolve(include_str!("../../../specimens/reader-workspace.toml"))
            .expect("reader specimen must be valid"),
    ];
    for b in &blueprints {
        println!("{}\n{}", b.semantic_tree(), viewwright_ascii::render(b));
    }
    eframe::run_native(
        "ViewWright Preview",
        eframe::NativeOptions::default(),
        Box::new(move |_| {
            Ok(Box::new(App {
                blueprints,
                screen: 0,
                fixture: 0,
            }))
        }),
    )
}

struct App {
    blueprints: Vec<ResolvedBlueprint>,
    screen: usize,
    fixture: usize,
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
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
                    }
                }
                ui.separator();
                ui.label("Fixture:");
                for (i, f) in self.blueprints[self.screen].fixtures.iter().enumerate() {
                    if ui.selectable_label(i == self.fixture, &f.id).clicked() {
                        self.fixture = i;
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
        viewwright_egui::show(ctx, b, fixture);
    }
}
