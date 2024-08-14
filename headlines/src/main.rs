
mod headlines;

use eframe::{
    egui::{
        CentralPanel, Context, FontFamily, Hyperlink, Label, RichText, ScrollArea, Separator, TextStyle, TopBottomPanel, Ui, ViewportBuilder
    },
    run_native, App, NativeOptions,
};
use headlines::{Headlines, PADDING};

impl App for Headlines {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.render_top_panel(ctx);
        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                self.render_news_cards(ui);
            });
            
            ui.add(Separator::default());
            ui.add_space(10.0);
            render_footer(ctx);
        });
    }
}

fn render_header(ui: &mut Ui) {
    ui.add_space(PADDING);
    ui.vertical_centered(|_ui| _ui.heading(RichText::new("Headlines").text_style(TextStyle::Name("UbuntuMono-Bold".into()))));
    ui.add(Separator::default().spacing(20.0));
}

fn render_footer(context: &Context) {
    TopBottomPanel::bottom("footer").show(context, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.add(Label::new(RichText::new("API source: newsapi.org").family(FontFamily::Monospace)));
            ui.add(Hyperlink::from_label_and_url("Made with egui", "https://github.com/emilk/egui"));
            ui.add(Hyperlink::from_label_and_url("Bishal-9/headlines", "https://github.com/Bishal-9/headlines"));
            ui.add_space(10.0);
        })
    });
}

fn main() {
    let window_options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([540.0, 960.0]),
        ..Default::default()
    };
    let _ = run_native(
        "Headlines",
        window_options,
        Box::new(|creator_context| Ok(Box::new(Headlines::new(creator_context)))),
    );
}
