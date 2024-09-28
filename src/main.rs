use eframe::egui;
use workspace::Workspace;

pub mod workspace;

fn main() -> eframe::Result<()> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            title: Some("Satisfactory Solver".into()),
            ..Default::default()
        }
        .into(),
        ..Default::default()
    };

    eframe::run_native(
        "Satisfactory Solver",
        options,
        Box::new(|_| Box::new(SolverApp::new())),
    )
}

struct SolverApp {
    workspace: Workspace,
}

impl SolverApp {
    fn new() -> Self {
        SolverApp {
            workspace: Workspace::new(),
        }
    }
}

impl eframe::App for SolverApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World!");

            ui.push_id("workspace", |ui| {
                self.workspace.show(ui);
            });
        });
    }
}
