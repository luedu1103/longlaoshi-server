use rocket::http::uri::Origin;
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::Route;
use std::path::PathBuf;
use rocket::http::Header;
use rocket::response::Responder;
use rocket::Request;

pub const BASE: Origin<'static> = uri!("/longlaoshi");

pub fn routes() -> Vec<Route> {
    routes![download_apk]
}

struct FileWithHeaders {
    file: NamedFile,
    headers: Vec<Header<'static>>,
}

impl<'r> Responder<'r, 'static> for FileWithHeaders {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let mut response = self.file.respond_to(req)?;
        for header in self.headers {
            response.set_header(header);
        }
        Ok(response)
    }
}

#[get("/download-apk")]
async fn download_apk() -> Result<FileWithHeaders, Status> {
    let file_path: PathBuf = PathBuf::from("./versions/app-release-v7.apk");

    match NamedFile::open(file_path).await {
        Ok(file) => {
            let headers = vec![
                Header::new("Content-Type", "application/vnd.android.package-archive"),
                Header::new("Content-Disposition", "attachment; filename=\"app-release-v7.apk\""),
            ];
            Ok(FileWithHeaders { file, headers })
        }
        Err(_) => Err(Status::NotFound),
    }
}
// #[get("/download-apk")]
// async fn download_apk() -> Result<NamedFile, Status> {
//     let file_path: PathBuf = PathBuf::from("./versions/app-release.apk");

//     NamedFile::open(file_path).await.map_err(|_| Status::NotFound)
// }
