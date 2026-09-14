use eframe::egui;
fn main() -> eframe::Result<()> {
    let b =
        viewwright_model::parse_and_resolve(include_str!("../../../examples/project-browser.toml"))
            .expect("example blueprint must be valid");
    println!("{}\n{}", b.semantic_tree(), viewwright_ascii::render(&b));
    eframe::run_native(
        "ViewWright Preview",
        eframe::NativeOptions::default(),
        Box::new(move |_| Ok(Box::new(App { b, fixture: 0 }))),
    )
}
struct App {
    b: viewwright_model::ResolvedBlueprint,
    fixture: usize,
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("fixtures").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Fixture:");
                for (i, f) in self.b.fixtures.iter().enumerate() {
                    if ui.selectable_label(i == self.fixture, &f.id).clicked() {
                        self.fixture = i;
                    }
                }
            });
        });
        let f = self
            .b
            .fixtures
            .get(self.fixture)
            .map(|f| f.id.as_str())
            .unwrap_or("default");
        viewwright_egui::show(ctx, &self.b, f);
    }
}
