use eframe::egui::*;

pub struct Workspace {}

impl Workspace {
    pub fn new() -> Self {
        Workspace {}
    }

    pub fn show(&mut self, ui: &mut Ui) {
        let draw_rect = ui.max_rect();
        let response = ui.allocate_rect(draw_rect, Sense::hover());

        let node_position = pos2(30., 40.);
        let node_size = vec2(100., 100.);

        let mut node_ui = ui.child_ui_with_id_source(
            Rect::from_min_size(node_position, node_size),
            Layout::default(),
            "A Node",
        );
    }
}
