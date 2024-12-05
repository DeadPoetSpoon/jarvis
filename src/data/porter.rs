use super::{resource::Resource, ResourceId};
use poll_promise::Promise;

pub trait Porter {
    fn fetch(&self, target: ResourceId) -> Promise<Resource>;
    fn deliver(&self, resource: Resource) -> Promise<Resource>;
    fn destroy(&self, target: ResourceId) -> Promise<Resource>;
    fn replace(&self, resource: Resource) -> Promise<Resource>;
}

#[derive(serde::Serialize, serde::Deserialize)]
enum RocketPorterAction {
    Get(ResourceId),
    Post(Resource),
    Delete(ResourceId),
    Put(Resource),
}

#[derive(Debug, Clone)]
pub struct RocketPorter {
    url: String,
}

impl RocketPorter {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_owned(),
        }
    }
}

impl Porter for RocketPorter {
    fn fetch(&self, target: ResourceId) -> Promise<Resource> {
        let (sender, promise) = Promise::new();
        let action = RocketPorterAction::Get(target.clone());
        let body = ron::ser::to_string(&action);
        if body.is_err() {
            sender.send(Resource::pkg_error(body.err().unwrap()));
            return promise;
        }
        let body = body.unwrap();
        let body = body.as_bytes().to_vec();
        let request = ehttp::Request::post(self.url.clone(), body);
        ehttp::fetch(request, move |response| {
            let resource = match response.map(|response| response.into()) {
                Ok(res) => res,
                Err(err) => Resource::pkg_error(err),
            };
            sender.send(resource);
        });
        promise
    }
    fn deliver(&self, resource: Resource) -> Promise<Resource> {
        let (sender, promise) = Promise::new();
        sender.send(Resource::default());
        promise
    }
    fn destroy(&self, target: ResourceId) -> Promise<Resource> {
        let (sender, promise) = Promise::new();
        sender.send(Resource::default());
        promise
    }
    fn replace(&self, resource: Resource) -> Promise<Resource> {
        let (sender, promise) = Promise::new();
        sender.send(Resource::default());
        promise
    }
}
