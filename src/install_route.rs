use rocket::http::uri::Origin;
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::Route;
use std::path::PathBuf;

pub const BASE: Origin<'static> = uri!("/longlaoshi");

pub fn routes() -> Vec<Route> {
    routes![download_apk]
}

#[get("/download-apk")]
async fn download_apk() -> Result<NamedFile, Status> {
    let file_path: PathBuf = PathBuf::from("./versions/app-release.apk");

    // Print the absolute path
    match file_path.canonicalize() {
        Ok(absolute_path) => println!("Absolute file path: {:?}", absolute_path),
        Err(e) => println!("Error getting absolute path: {:?}", e),
    }

    NamedFile::open(file_path).await.map_err(|_| Status::NotFound)
}
