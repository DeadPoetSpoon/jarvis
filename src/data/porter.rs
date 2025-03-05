use super::{resource::Resource, ResourceId};
use chrono::{DateTime, Local};
use poll_promise::Promise;

pub trait Porter {
    fn fetch(&self, target: ResourceId) -> Promise<Resource>;
    fn fetch_all(&self, target: ResourceId) -> Promise<Resource>;
    fn deliver(&self, resource: Resource) -> Promise<Resource>;
    fn destroy(&self, target: ResourceId) -> Promise<Resource>;
    fn replace(&self, resource: Resource) -> Promise<Resource>;
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum RocketPorterAction {
    GetAll(ResourceId),
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
    pub fn fetch_promise(&self, action: &RocketPorterAction) -> Promise<Resource> {
        let (sender, promise) = Promise::new();
        let body = ron::ser::to_string(action);
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
}

impl Porter for RocketPorter {
    fn fetch_all(&self, target: ResourceId) -> Promise<Resource> {
        let action = RocketPorterAction::GetAll(target);
        self.fetch_promise(&action)
    }
    fn fetch(&self, target: ResourceId) -> Promise<Resource> {
        let action = RocketPorterAction::Get(target);
        self.fetch_promise(&action)
    }
    fn deliver(&self, resource: Resource) -> Promise<Resource> {
        let action = RocketPorterAction::Post(resource);
        self.fetch_promise(&action)
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
