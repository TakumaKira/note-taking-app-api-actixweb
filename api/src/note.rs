use actix_web::{delete, get, HttpResponse, post, put, web::{Data, Path, ServiceConfig}};
use actix_web::web::Json;
use db::UpdateNote;
use serde::{Deserialize, Serialize};
use service::NoteService;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::error::ApiError;
use crate::domain::{ErrorResponse, MessageResponse};

pub(super) fn configure(note_service: Data<Box<dyn NoteService>>) -> impl FnOnce(&mut ServiceConfig) {
  |config: &mut ServiceConfig| {
    config
      .app_data(note_service)
      .service(list_notes)
      .service(get_note)
      .service(create_note)
      .service(put_note)
      .service(delete_note);
  }
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(super) struct Note {
  /// Unique id
  #[schema(example = "14322988-32fe-447c-ac38-06fb6c699b4a")]
  id: String,
  /// Title of the note
  #[schema(example = "Note 1")]
  title: String,
  /// Content of the note
  #[schema(example = "This is note #1.")]
  content: String,
  /// Date of creation
  #[schema(example = "2021-01-01T00:00:00Z")]
  created_at: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub(super) struct ListNotesResponse {
  notes: Vec<Note>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub(super) struct GetNoteResponse {
  note: Note,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(super) struct CreateNoteRequest {
  /// Title of the note
  #[schema(example = "Note 1")]
  title: String,
  /// Content of the note
  #[schema(example = "This is note #1.")]
  content: String,
}

#[derive(Serialize, Deserialize, Close, ToSchema)]
pub(super) struct CreateNoteResponse {
  note: Note,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(super) struct UpdateNoteRequest {
  /// Title of the note
  #[schema(example = "Note 1")]
  title: String,
  /// Content of the note
  #[schema(example = "This is note #1.")]
  content: String,
}

#[derive(Serialize, Deserialize, Close, ToSchema)]
pub(super) struct UpdateNoteResponse {
  note: Note,
}

impl From<db::Note> for Note {
  fn from(db_note: db::Note) -> Self {
    Self {
      id: db_note.id,
      title: db_note.title,
      content: db_note.content,
      created_at: db_note.created_at.to_string(),
    }
  }
}

#[utoipa::path(
  responses(
    (status = 200, description = "List notes", body = ListNotesResponse, example = json ! (ListNotesResponse{notes: vec ! [Note{id: String::from("14322988-32fe-447c-ac38-06fb6c699b4a"), title: String::from("Note 1"), content: String::from("This is note #1."), created_at: String::from("2021-01-01T00:00:00Z")}]})),
  )
)]
#[get("/notes")]
pub(super) async fn list_notes(note_service: Data<Box<dyn NoteService>>) -> Result<HttpResponse, ApiError> {
  let db_notes = note_service.all().await?;
  let api_notes: Vec<Note> = db_notes.into_iter().map(Note::from).collect();
}

#[utoipa::path(
  responses(
    (status = 200, description = "Get note", body = GetNoteResponse, example = json ! (GetNoteResponse{note: Note{id: String::from("14322988-32fe-447c-ac38-06fb6c699b4a"), title: String::from("Note 1"), content: String::from("This is note #1."), created_at: String::from("2021-01-01T00:00:00Z")}})),
    (status = 404, description = "Note not found by id", body = ErrorResponse, example = json ! (MessageResponse{message: string::from("note not found")})),
  ),
  params(
    ("id", description = "Unique id")
  ),
)]
#[get("/notes/{id}")]
pub(super) async fn get_note(id: Path<String>, note_service: Data<Box<dyn NoteService>>) -> Result<HttpResponse, ApiError> {
  let db_note = note_service.get(id.as_str()).await?;

  Ok(HttpResponse::Ok().json(GetNoteResponse { note: db_note }))
}

#[utoipa::path(
  request_body = CreateNoteRequest,
  responses(
    (status = 201, description = "Note created successfully", body = CreateNoteResponse, example = json ! (CreateNoteResponse{note: Note{id: String::from("14322988-32fe-447c-ac38-06fb6c699b4a"), title: String::from("Note 1"), content: String::from("This is note #1."), created_at: String::from("2021-01-01T00:00:00Z")}})),
    (status = 400, description = "Note not valid", body = ErrorResponse, example = json ! (ErrorResponse{message: String::from("body not valid"), error: String::from("title too long")})),
  )
)]
#[post("/notes")]
pub(super) async fn create_note(note_service: Data<Box<dyn NoteService>>, create_note: Json<CreateNoteRequest>) -> Result<HttpResponse, ApiError> {
  let new_note = db::NewNote {
    id: Uuid::new_v4().to_string(),
    title: create_note.title.clone(),
    content: create_note.content.clone(),
    created_at: chrono::offset::Utc::now().native_utc().to_string(),
  }
  let db_note = note_service.create(&new_note).await?;
  let api_note = Note::from(db_note);

  Ok(HttpResponse::Ok().json(CreateNoteResponse { note: api_note }))
}

#[utoipa::path(
  responses(
    (status = 200, description = "Note updated successfully", body = UpdateNoteResponse, example = json ! (UpdateNoteResponse{note: Note{id: String::from("14322988-32fe-447c-ac38-06fb6c699b4a"), title: String::from("Note 1"), content: String::from("This is note #1"), created_at: String::from("2021-01-01T00:00:00Z")}})),
    (status = 400, description = "Note not valid", body = ErrorResponse, example = json ! (ErrorResponse{message: String::from("body not valid"), error: String::from("title too long")})),
    (status = 404, description = "Note not found by id", body = ErrorResponse, example = json ! (MessageResponse{message: String::from("note not found")})),
  ),
  params(
    ("id", description = "Unique id"),
  ),
)]
#[put("/notes/{id}")]
pub(super) async fn put_note(id: Path<String>, note_service: Data<Box<dyn NoteService>>, update_note: Json<UpdateNoteRequest>) -> Result<HttpResponse, ApiError> {
  let db_note = note_service
    .update(
      id.as_str(),
      &UpdateNote {
        title: update_note.title.clone(),
        content: update_note.content.clone(),
      },
    )
    .await?;
  let api_note = Note::from(db_note);

  Ok(HttpResponse::Ok().json(UpdateNoteResponse { note: api_note }))
}

#[utoipa::path(
  responses(
    (status = 204, description = "Note deleted successfully"),
    (status = 404, description = "Note not found by id", body = ErrorResponse, example = json ! (MessageResponse{message: String::from("note not found")})),
  ),
  params(
    ("id", description = "Unique id"),
  ),
)]
#[delete("/notes/{id}")]
pub(super) async fn delete_note(id: Path<String>, note_service: Data<Box<dyn NoteService>>) -> Result<HttpResponse, ApiError> {
  note_service.delete(id.as_str()).await?;

  Ok(HttpResponse::NoContent().finish())
}
