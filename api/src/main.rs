use std::{
    error::Error,
    net::Ipv4Addr,
};

use actix_web::{App, HttpResponse, HttpServer, middleware, web};
use actix_web::web::Data;
use db::SqliteNoteRepository;
use service::{NoteService, NoteServiceImpl};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

use crate::domain::MessageResponse;

mod note;
mod error;
mod domain;

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    #[derive(OpenApi)]
    #[openapi(
        paths(
            note::list_notes,
            note::get_note,
            note::create_note,
            note::put_note,
            note::delete_note
        ),
        components(
            schemas(note::Note, note::ListNotesResponse, note::GetNoteResponse, note::CreateNoteRequest, note::CreateNoteResponse, note::UpdateNoteRequest, note::UpdateNoteResponse, domain::ErrorResponse, domain::MessageResponse)
        ),
        tags(
            (name = "notes", description = "Note management endpoints.")
        )
    )]
    struct ApiDoc;

    // Make instance variable of ApiDoc so all worker threads gets the same instance.
    let openapi = ApiDoc::openapi();

    let note_repository = SqliteNoteRepository::new("./notes.db").await.expect("Failed to connect to database.");
    let note_service = NoteServiceImpl::new(note_repository);
    let note_service_data = Data::new(Box::new(note_service) as Box<dyn NoteService>);

    HttpServer::new(move || {
        // This factory closure is called on each worker thread independently.
        App::new()
            .wrap(middleware::Logger::default())
            .configure(note::configure(note_service_data.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
            .default_service(web::route().to(not_found))
    })
        .bind((Ipv4Addr::UNSPECIFIED, 8081))?
        .run()
        .await
}

async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(MessageResponse {
        message: "not found".to_string(),
    })
}
