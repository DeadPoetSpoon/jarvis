use std::{
    collections::VecDeque,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};

use crate::data::{Porter, Resource, ResourceData, ResourceId, RocketPorter};
use egui::RichText;
use poll_promise::Promise;

#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum Anchor {
    Schedule,
}
impl Anchor {}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct JarvisUI {
    name: String,
    #[serde(skip)]
    rocket_porter: RocketPorter,
    #[serde(skip)]
    res_queue: Arc<Mutex<VecDeque<Promise<Resource>>>>,
}

impl Default for JarvisUI {
    fn default() -> Self {
        let rocket_porter = RocketPorter::new("http://jarvis:8000/schedule");
        let res_queue = Arc::new(Mutex::new(VecDeque::new()));
        Self {
            name: "Jarvis".to_owned(),
            rocket_porter,
            res_queue,
        }
    }
}

impl JarvisUI {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // set fonts
        let font_definitiona = JarvisUI::get_font();
        cc.egui_ctx.set_fonts(font_definitiona);
        // load app state
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
    fn get_font() -> egui::FontDefinitions {
        let mut font_definitiona = egui::FontDefinitions::default();
        font_definitiona.font_data.insert(
            "0x Regular".to_owned(),
            egui::FontData::from_static(include_bytes!(
                "../../assets/fonts/0xProto/0xProtoNerdFontPropo-Regular.ttf"
            )),
        );
        font_definitiona.font_data.insert(
            "0x Mono Regular".to_owned(),
            egui::FontData::from_static(include_bytes!(
                "../../assets/fonts/0xProto/0xProtoNerdFontMono-Regular.ttf"
            )),
        );
        font_definitiona.font_data.insert(
            "cn Regular".to_owned(),
            egui::FontData::from_static(include_bytes!(
                "../../assets/fonts/SourceHanSans/SourceHanSansCN-Regular.otf"
            )),
        );

        font_definitiona
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "0x Regular".to_owned());

        font_definitiona
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(1, "cn Regular".to_owned());
        font_definitiona
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "0x Mono Regular".to_owned());
        font_definitiona
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(1, "cn Regular".to_owned());
        font_definitiona
    }
    fn push_promise(&mut self, promise: Promise<Resource>) {
        let mut queue = self.res_queue.lock().unwrap();
        queue.push_back(promise);
    }
    fn pop_promise(&mut self) -> Option<Promise<Resource>> {
        let mut queue = self.res_queue.lock().unwrap();
        queue.pop_front()
    }
    fn handle_resrouse(&mut self, resource: Resource) {
        match resource.data {
            ResourceData::Sample(str) => {
                self.name = str;
            }
            ResourceData::Error(err) => {
                self.name = err;
            }
            _ => {}
        };
    }
}

impl eframe::App for JarvisUI {
    /// save app state
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    /// main
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("change name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.horizontal(|ui| {
                ui.label("Hello~ I'm ");
                ui.label(RichText::new(self.name.clone()).font(egui::FontId {
                    size: 18f32,
                    family: egui::FontFamily::Monospace,
                }));
            });

            if ui.button("Add file to Minio").clicked() {
                // let path = Path::new("/home/Jarvis/test");
                // let mut file = File::create(path).unwrap();
                // write!(file, "asd").unwrap();
                let id = ResourceId {
                    place: "a".to_owned(),
                    path: PathBuf::from_str("asd/asd").unwrap(),
                };
                let promise = self.rocket_porter.fetch(id);
                self.push_promise(promise);
            }

            if let Some(promise) = self.pop_promise() {
                if let Some(resource) = promise.ready() {
                    self.handle_resrouse(resource.to_owned())
                } else {
                    self.push_promise(promise);
                }
            }
        });
    }
}
