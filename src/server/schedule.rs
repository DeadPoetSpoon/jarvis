use crate::data::Resource;
use rocket::post;
#[post("/")]
pub fn hello() -> String {
    let fmt = "%Y年%m月%d日 %H:%M:%S";
    let now = chrono::Local::now().format(fmt);
    let resource = Resource {
        data: crate::data::ResourceData::Sample(now.to_string()),
        ..Default::default()
    };
    ron::to_string(&resource)
        .map_err(|err| err.to_string())
        .unwrap()
}
