use actix_web::{delete, get, HttpResponse, post, put, web::{Data, Path, ServiceConfig}};
use actix_web::web::Json;
use db::UpdateNote;
use serde::{Deserialize, Serialize};
use service::NoteService;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::error::ApiError;
use crate::domain::{ErrorResponse, MessageResponse};
#[cfg(test)]
use mockall::{mock, predicate::*};

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
  };
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

#[cfg(test)]
mod tests {
  use actix_web::test;
  use super::*;
  use anyhow::Result;
  use async_trait::async_trait;
  use actix_web::App;
  use mockall::predicate;

  mock! {
    Service {}
    #[async_trait]
    impl service::NoteService for Service {
      async fn all(&self) -> Result<Vec<db::Note>;
      async fn get(&self, id: &str) -> Result<db::Note>;
      async fn create(&self, note: &db::NewNote) -> Result<db::Note>;
      async fn update(&self, id: &str, note: &db::UpdateNote) -> Result<db::Note>;
      async fn delete(&self, id: &str) -> Result<db::Note>;
    }
  }

  #[actix_web::test]
  async fn test_list_notes() {
    let mut mock_service = MockService::new();

    mock_service.expect_all()
      .times(1)
      .returning(|| Ok(vec![db::Note {
        id: String::from("14322988-32fe-447c-ac38-06fb6c699b4a"),
        title: String::from("Note 1"),
        content: String::from("This is note #1."),
        created_at: String::from("2021-01-01T00:00:00Z"),
      }]));

    let note_service_data = Data::new(Box::new(mock_service) as Box<dyn NoteService>);

    let mut app = test::init_service(
      App::new().configure(configure(note_service_data.clone()))
    ).await;

    let req = test::TestRequest::get().uri("/notes").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success());
  }

  // #[actix_web::test]
  // async fn test_get_note() {

  // }

  #[actix_web::test]
  async fn test_create_note() {
    let mut mock_service = MockService::new();

    let new_note = db::NewNote {
      id: String::from("new-id"),
      title: String::from("Note 1"),
      content: String::from("This is note #1."),
      created_at: String::from("2021-01-01T00:00:00Z"),
    };
    let new_note_test = new_note.clone();
    mock_service.expect_create()
      .times(1)
      .returning(move |_| Ok(db::Note {
        id: String::from("new-id"),
        title: new_student_test.title.clone(),
        content: new_student_test.content.clone(),
        created_at: new_student_test.created_at.clone(),
      }));

    let note_service_data = Data::new(Box::new(mock_service) as Box<dyn NoteService>);

    let mut app = test::init_service(
      App::new().configure(configure(note_service_data.clone()))
    ).await;

    let note = CreateNoteRequest {
      title: "Note 1".to_string(),
      content: "This is note #1.".to_string(),
    };

    let req = test::TestRequest::post()
      .uri("/notes")
      .set_json(&note)
      .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;

    let returned_note: CreateNoteResponse = serde_json::from_slice(&body).unwrap();

    let expected_note = db::Note {
      id: String::from("new-id"),
      title: String::from("Note 1"),
      content: String::from("This is note #1."),
      created_at: String::from("2021-01-01T00:00:00Z"),
    };
    assert_eq!(returned_note.note.id, expected_note.id);
    assert_eq!(returned_note.note.title, expected_note.title);
    assert_eq!(returned_note.note.content, expected_note.content);
    assert_eq!(returned_note.note.created_at, expected_note.created_at);
  }

  #[actix_web::test]
  async fn test_update_note() {
    let mut mock_service = MockService::new();

    let note_id = "some-id";
    let update_request = UpdateNoteRequest {
      title: "Updated Title".to_string(),
      content: "Updated content".to_string(),
    };
    let updated_note = db::UpdateNote {
      title: update_request.title.clone(),
      content: update_request.content.clone(),
    };
    let updated_note_test = updated_note.clone();
    let updated_note_test_predicate = updated_note.clone();
    mock_service.expect_update()
      .with(predicate::eq(note_id), predicate::eq(updated_note_test_predicate))
      .times(1)
      .returning(move |_, _| Ok(db::Note {
        id: note_id.clone().to_string(),
        title: updated_note_test.title.clone(),
        content: updated_note_test.content.clone(),
        created_at: String::from("2021-01-01T00:00:00Z"),
      }));

    let note_service_data = Data::new(Box::new(mock_service) as Box<dyn NoteService>);

    let mut app = test::init_service(
      App::new().configure(configure(note_service_data.clone()))
    ).await;

    let req = test::TestRequest::put()
      .uri(&format!("/notes/{}", note_id))
      .set_json(&update_request)
      .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let returned_Note: UpdateNoteResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(returned_note.Note.title, update_request.title);
    assert_eq!(returned_note.Note.content, update_request.content);
  }

  #[actix_web::test]
  async fn test_delete_note() {
    let mut mock_service = MockService::new();

    let note_id = "some-id";
    mock_service.expect_delete()
      .with(predicate::eq(note_id))
      .times(1)
      .returning(|_| Ok(db::Note {
        id: note_id.to_string(),
        title: String::from("Note 1"),
        content: String::from("This is note #1."),
        created_at: String::from("2021-01-01T00:00:00Z"),
      }));

    let note_service_data = Data::new(Box::new(mock_service) as Box<dyn NoteService>);

    let mut app = test::init_service(
      App::new().configure(configure(note_service_data.clone()))
    ).await;

    let req = test::TestRequest::delete()
      .uri(&format!("/notes/{}", note_id))
      .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success());
    assert_eq!(resp.status(), 204);
  }

  #[actix_web::test]
  async fn test_delete_student_not_found() {
      let mut mock_service = MockService::new();

      // Set up the mock to return an error of type db::NotFound
      mock_service.expect_delete()
          .times(1)
          .returning(|_| Err(anyhow::anyhow!(db::DbError::NotFound)));

      let student_service_data = Data::new(Box::new(mock_service) as Box<dyn StudentService>);

      let mut app = test::init_service(
          App::new().configure(configure(student_service_data.clone()))
      ).await;

      let req = test::TestRequest::delete().uri("/students/some_id").to_request();
      let resp = test::call_service(&mut app, req).await;

      assert_eq!(resp.status(), 404);
  }
}
