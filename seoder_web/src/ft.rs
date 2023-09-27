use bytes::BufMut;
use futures::TryStreamExt;
use std::convert::Infallible;
use uuid::Uuid;
use warp::{
    http::StatusCode,
    multipart::{FormData, Part},
    Filter, Rejection, Reply,
};

use crate::controls;
use crate::{string_concat_impl, utils::get_prospects};
use seoder_lib::{
    packages::spider::utils::logd, string_concat::string_concat, tokio, ENTRY_PROGRAM,
};

/// file server basic server init
pub async fn file_server() {
    let cors = warp::cors().allow_any_origin();

    let upload_route = warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(5_000_000))
        .and_then(upload)
        .with(&cors);

    let download_route = warp::path("download")
        .and(warp::fs::dir(ENTRY_PROGRAM.3.to_string()))
        .with(&cors);

    let prospect_route = warp::path("prospect")
        .and(warp::post())
        .and(warp::multipart::form().max_length(5_000_000))
        .and_then(prospect)
        .with(&cors);

    let router = upload_route
        .or(download_route)
        .or(prospect_route)
        .recover(handle_rejection)
        .with(&cors);

    println!("Server started at localhost:7050");

    warp::serve(router).run(([0, 0, 0, 0], 7050)).await;
}

/// prospect from api endpoint
async fn prospect(form: FormData) -> Result<impl Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject()
    })?;

    let mut title = String::from("");

    for p in parts {
        if p.name() == "title" {
            title = p.filename().unwrap_or_default().to_string();
        }
    }

    let key = controls::list::license().await;
    let prospect = get_prospects(&key, &title).await;

    Ok(prospect)
}

async fn upload(form: FormData) -> Result<impl Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject()
    })?;

    for p in parts {
        if p.name() == "file" {
            let f = p.filename();

            let content_type = p.content_type();
            let file_ending;

            match content_type {
                Some(file_type) => match file_type {
                    "text/plain" => {
                        file_ending = "txt";
                    }
                    v => {
                        eprintln!("invalid file type found: {}", v);
                        return Err(warp::reject::reject());
                    }
                },
                None => {
                    eprintln!("file type could not be determined");
                    return Err(warp::reject::reject());
                }
            }

            let d = f.unwrap();

            let d = if d.is_empty() {
                Uuid::new_v4().to_string()
            } else {
                d.to_string()
            };

            let file_name = if d.ends_with(file_ending) {
                string_concat!(&ENTRY_PROGRAM.1, d)
            } else {
                string_concat!(&ENTRY_PROGRAM.1, d, ".", file_ending)
            };

            let value = p
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    eprintln!("reading file error: {}", e);
                    warp::reject::reject()
                })?;

            tokio::fs::write(&file_name, value).await.map_err(|e| {
                eprint!("error writing file: {}", e);
                warp::reject::reject()
            })?;

            logd(string_concat!("created file: ", file_name));
        }
    }

    Ok("success")
}

async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}
