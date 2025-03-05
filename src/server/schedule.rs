use std::path::PathBuf;

use crate::{
    data::{Resource, RocketPorterAction},
    Job, ResourceData, ResourceId,
};
use minio::s3::{
    builders::ObjectContent,
    client::Client,
    types::{S3Api, ToStream},
};
use rocket::{futures::StreamExt, post, response::content, State};
#[post("/", data = "<input>")]
pub async fn handle(input: String, client: &State<Client>) -> String {
    let action = ron::from_str(input.as_str());
    if action.is_err() {
        let err = action.err().unwrap();
        return ron::to_string(&Resource::pkg_error(err)).unwrap();
    }
    let action = action.unwrap();
    let resource = handle_action(action, client).await;
    ron::to_string(&resource)
        .map_err(|err| err.to_string())
        .unwrap()
}

async fn handle_action(action: RocketPorterAction, client: &State<Client>) -> Resource {
    match action {
        RocketPorterAction::GetAll(target) => handle_get_all(target, client).await,
        RocketPorterAction::Get(id) => handle_get(id, client).await,
        RocketPorterAction::Post(resource) => handle_post(resource, client).await,
        RocketPorterAction::Delete(_) => todo!(),
        RocketPorterAction::Put(_) => todo!(),
    }
}

async fn handle_get_all(target: ResourceId, client: &State<Client>) -> Resource {
    let mut job_table = Vec::new();
    let mut objects = client
        .list_objects(target.place.as_str())
        .prefix(Some(target.path.to_str().unwrap().to_string()))
        .to_stream()
        .await;

    while let Some(object) = objects.next().await {
        match object {
            Ok(content) => {
                //job_table.push(content.contents.first().unwrap().name.clone());
                for item in content.contents {
                    job_table.push(item.name.clone());
                }
            }
            Err(err) => return Resource::pkg_error(err),
        }
    }
    Resource::message(job_table.join(";"))
}
async fn get_object_from_minio(id: ResourceId, client: &State<Client>) -> Resource {
    let result = client
        .get_object(id.place.as_str(), id.path.to_str().unwrap())
        .send()
        .await;
    match result {
        Ok(object) => {
            let content = object.content.to_segmented_bytes().await;
            match content {
                Ok(content) => {
                    let bytes = content.to_bytes().to_vec();
                    let content_str = String::from_utf8(bytes);
                    match content_str {
                        Ok(content_str) => {
                            let jobs: Vec<Job> = ron::from_str(content_str.as_str()).unwrap();
                            Resource {
                                data: ResourceData::Jobs(jobs),
                                ..Default::default()
                            }
                        }
                        Err(err) => Resource::pkg_error(err),
                    }
                }
                Err(err) => Resource::pkg_error(err),
            }
        }
        Err(err) => Resource::pkg_error(err),
    }
}
async fn handle_get(id: ResourceId, client: &State<Client>) -> Resource {
    get_object_from_minio(id, client).await
}

async fn handle_post(resource: Resource, client: &State<Client>) -> Resource {
    match resource.data {
        ResourceData::Job(job) => post_job(resource.id, job, client).await,
        _ => Resource::message("reseive wrong resource. "),
    }
}

async fn post_job(id: ResourceId, job: Job, client: &State<Client>) -> Resource {
    let bucket = id.place;
    let father_path = id.path;
    if job.is_father() {
        post_father_job(bucket, father_path, job, client).await
    } else {
        post_sub_job(bucket, father_path, job, client)
    }
}
async fn post_father_job(
    bucket: String,
    father_path: PathBuf,
    job: Job,
    client: &State<Client>,
) -> Resource {
    let path = father_path.join(job.path());
    let vec = vec![job];
    let job = ron::to_string(&vec).unwrap();
    let object_content = ObjectContent::from(job);
    match client
        .put_object_content(bucket.as_str(), path.to_str().unwrap(), object_content)
        .send()
        .await
    {
        Ok(response) => Resource::message(format!(
            "Post {}({}) to {}.",
            response.object_name, response.object_size, response.bucket_name,
        )),
        Err(err) => Resource::pkg_error(err),
    }
}
fn post_sub_job(
    bucket: String,
    father_path: PathBuf,
    job: Job,
    client: &State<Client>,
) -> Resource {
    Resource::nothing()
}
