
use eframe::egui::{
        menu, Align, Button, Color32, Context, FontData, FontDefinitions, FontFamily, FontId, Hyperlink, Label, Layout, RichText, Separator, TextStyle, TopBottomPanel, Ui
    };

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

struct NewsCardData {
    title: String,
    description: String,
    url: String,
}
pub struct Headlines {
    articles: Vec<NewsCardData>,
}
impl Headlines {
    pub fn new(creation_context: &eframe::CreationContext<'_>) -> Headlines {
        setup_custom_fonts(&creation_context.egui_ctx);

        let iterator = (0..20).map(|a| NewsCardData {
            title: format!("Title - {}", a),
            description: format!("Description - {}", a),
            url: format!("https://example.com/{}", a),
        });
        Headlines {
            articles: Vec::from_iter(iterator),
        }
    }

    pub fn render_news_cards(&self, ui: &mut Ui) {
        for item in &self.articles {
            ui.add_space(PADDING);

            let title = format!("▶ {}", item.title);
            ui.colored_label(WHITE, RichText::new(&title).family(FontFamily::Monospace).size(18.0).text_style(TextStyle::Name("UbuntuMono-Bold".into())));

            ui.add_space(PADDING);
            let description = Label::new(RichText::new(&item.description).text_style(TextStyle::Body));
            ui.add(description);

            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(Align::LEFT), |ui| {
                // ui.add(Hyperlink::from_label_and_url(RichText::new("read more ⤴").text_style(TextStyle::Name("UbuntuMono-Italic".into())), &item.url))
                ui.add(Hyperlink::from_label_and_url(RichText::new("read more...").text_style(TextStyle::Name("UbuntuMono-Italic".into())), &item.url))
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }
    pub fn render_top_panel(&self, context: &Context) {
        TopBottomPanel::top("top_panel").show(context, |ui| {
            ui.add_space(10.0);
            menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(Align::LEFT), |ui| {
                    ui.add(Label::new(RichText::new("📰").text_style(TextStyle::Button)));
                });

                ui.with_layout(Layout::right_to_left(Align::LEFT), |ui| {
                    let close_button = ui.add(Button::new(RichText::new("❌").text_style(TextStyle::Button)));
                    let refresh_button = ui.add(Button::new(RichText::new("🔃").text_style(TextStyle::Button)));
                    let theme_button = ui.add(Button::new(RichText::new("🌙").text_style(TextStyle::Button)));
                });
            });
            ui.add_space(10.0);
        });
    }
}

fn setup_custom_fonts(context: &Context) {
    // Initializing default Fonts and Font Style
    let mut fonts = FontDefinitions::default();
    let mut style = (*context.style()).clone();

    // Installing fonts
    fonts.font_data.insert(
        "UbuntuMono-Regular".to_owned(),
        FontData::from_static(include_bytes!(
            "../../assets/Ubuntu_Mono/UbuntuMono-Regular.ttf"
        )),
    );
    fonts.font_data.insert(
        "UbuntuMono-Bold".to_owned(),
        FontData::from_static(include_bytes!(
            "../../assets/Ubuntu_Mono/UbuntuMono-Bold.ttf"
        )),
    );
    fonts.font_data.insert(
        "UbuntuMono-Italic".to_owned(),
        FontData::from_static(include_bytes!(
            "../../assets/Ubuntu_Mono/UbuntuMono-Italic.ttf"
        )),
    );

    // Setting font as highest priority
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "UbuntuMono-Regular".to_owned());
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, "UbuntuMono-Regular".to_owned());
    fonts
        .families
        .entry(FontFamily::Name("UbuntuMono-Bold".into()))
        .or_default()
        .insert(0, "UbuntuMono-Bold".to_owned());
    fonts
        .families
        .entry(FontFamily::Name("UbuntuMono-Italic".into()))
        .or_default()
        .insert(0, "UbuntuMono-Italic".to_owned());

    // Setting Font Styles
    style.text_styles = [
        (
            TextStyle::Heading,
            FontId::new(32.0, FontFamily::Proportional),
        ),
        (TextStyle::Body, FontId::new(14.0, FontFamily::Proportional)),
        (TextStyle::Button, FontId::new(16.0, FontFamily::Proportional)),
        (TextStyle::Name("UbuntuMono-Bold".into()), FontId::new(32.0, FontFamily::Name("UbuntuMono-Bold".into()))),
        (TextStyle::Name("UbuntuMono-Italic".into()), FontId::new(12.0, FontFamily::Name("UbuntuMono-Italic".into()))),
    ]
    .into();

    context.set_fonts(fonts);
    context.set_style(style);
}
