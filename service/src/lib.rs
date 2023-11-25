use anyhow::Result;
use async_trait::async_trait;
use db::NewNote;
use db::Note;
use db::NoteRepository;
use db::UpdateNote;
use validator::Validate;
#[cfg(test)]
use mockall::{mock, predicate::*};

#[async_trait]
pub trait NoteService: Sync + Send {
    async fn all(&self) -> Result<Vec<Note>>;
    async fn get(&self, id: &str) -> Result<Note>;
    async fn create(&self, note: &NewNote) -> Result<Note>;
    async fn update(&self, id: &str, note: &UpdateNote) -> Result<Note>;
    async fn delete(&self, id: &str) -> Result<Note>;
}

pub struct NoteServiceImpl<R: NoteRepository + Send + Sync> {
    repository: R,
}

impl<R: NoteRepository + Send + Sync> NoteServiceImpl<R> {
    pub fn new(repository: R) -> Self {
        NoteServiceImpl { repository }
    }
}

#[async_trait]
impl<R: NoteRepository + Send + Sync> NoteService for NoteServiceImpl<R> {
    async fn all(&self) -> Result<Vec<Note>> {
        self.repository.all().await
    }

    async fn get(&self, id: &str) -> Result<Note> {
        self.repository.get(id).await
    }

    async fn create(&self, note: &NewNote) -> Result<Note> {
        note.validate()?;

        self.repository.create(note).await
    }

    async fn update(&self, id: &str, note: &UpdateNote) -> Result<Note> {
        note.validate()?;

        self.repository.update(id, note).await
    }

    async fn delete(&self, id: &str) -> Result<Note> {
        self.repository.delete(id).await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use futures_util::future::FutureExt;
    use mockall::predicate;

    mock! {
        Repository {}
        #[async_trait]
        impl db::NoteRepository for Repository {
            async fn all(&self) -> Result<Vec<Note>>;
            async fn get(&self, id: &str) -> Result<Note>;
            async fn create(&self, note: &NewNote) -> Result<Note>;
            async fn update(&self, id: &str, note: &UpdateNote) -> Result<Note>;
            async fn delete(&self, id: &str) -> Result<Note>;
        }
    }

    #[test]
    fn test_all() {
        let mut mock = MockRepository::new();
        mock.expect_all()
            .times(1)
            .returning(|| Ok(vec![Note {
                id: String::from("14322988-32fe-447c-ac38-06fb6c699b4a"),
                title: String::from("Note 1"),
                content: String::from("This is note #1."),
                created_at: String::from("2021-01-01T00:00:00Z"),
            }]));
        let service = NoteServiceImpl::new(mock);
        let notes = service.all().now_or_never().unwrap().unwrap();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].id, "14322988-32fe-447c-ac38-06fb6c699b4a")
    }

    #[test]
    fn test_get() {
        let mut mock = MockRepository::new();
        let expected_id = "some-id";
        mock.expect_get()
            .with(predicate::eq(expected_id))
            .times(1)
            .returning(|_| Ok(Note {
                id: String::from("some-id"),
                title: String::from("Note 1"),
                content: String::from("This is note #2."),
                created_at: String::from("2021-01-01T00:00:00Z"),
            }));
        let service = NoteServiceImpl::new(mock);
        let note = service.get(expected_id).now_or_never().unwrap().unwrap();
        assert_eq!(note.id, expected_id);
    }

    #[test]
    fn test_create() {
        let mut mock = MockRepository::new();
        let new_note = NewNote {
            id: String::from("new-id"),
            title: String::from("Note 1"),
            content: String::from("This is note #2."),
            created_at: String::from("2021-01-01T00:00:00Z"),
        };
        let new_note_test = new_note.clone();
        mock.expect_create()
            .with(predicate::eq(new_note_test.clone()))
            .times(1)
            .returning(move |_| Ok(Note {
                id: String::from("new-id"),
                title: new_note_test.title.clone(),
                content: new_note_test.content.clone(),
                created_at: new_note_test.created_at.clone(),
            }));
        let service = NoteServiceImpl::new(mock);
        let note = service.create(&new_note).now_or_never().unwrap().unwrap();
        assert_eq!(note.id, "new-id");
    }

    #[test]
    fn test_create_with_invalid_note() {
        let mock = MockRepository::new();
        let service = NoteServiceImpl::new(mock);
        let invalid_note = NewNote {
            id: String::from("new-id"),
            title: String::new(),
            content: String::from("This is a new note."),
            created_at: String::from("2021-01-01T00:00:00Z"),
        };
        let result = service.create(&invalid_note).now_or_never();
        assert!(result.is_some(), "Expected a synchronous result");
        assert!(result.unwrap().is_err(), "Expected an error due to validation");
    }

    #[test]
    fn test_update() {
        let mut mock = MockRepository::new();
        let note_id = "update-id";
        let update_note = UpdateNote {
            title: String::from("Note 1"),
            content: String::from("This is note #1."),
        };
        let update_note_test = update_note.clone();
        mock.expect_update()
            .with(predicate::eq(note_id), predicate::eq(update_note_test.clone()))
            .times(1)
            .returning(move |_, _| Ok(Note {
                id: String::from("update-id"),
                title: update_note_test.title.clone(),
                content: update_note_test.content.clone(),
                created_at: String::from("2021-01-01T00:00:00Z"),
            }));
        let service = NoteServiceImpl::new(mock);
        let note = service.update(note_id, &update_note).now_or_never().unwrap().unwrap();
        assert_eq!(note.id, note_id);
    }

    #[test]
    fn test_update_with_invalid_note() {
        let mock = MockRepository::new();
        let service = NoteServiceImpl::new(mock);
        let invalid_note = UpdateNote {
            title: String::new(),
            content: String::from("")
        };
        let result = service.update("id", &invalid_note).now_or_never();
        assert!(result.is_some(), "Expected a synchronous result");
        assert!(result.unwrap().is_err(), "Expected an error due to validation");
    }

    #[test]
    fn test_delete() {
        let mut mock = MockRepository::new();
        let delete_id = "delete-id";
        mock.expect_delete()
            .with(predicate::eq(delete_id))
            .times(1)
            .returning(|_| Ok(Note {
                id: String::from("delete-id"),
                title: String::from("Note 1"),
                content: String::from("This is note #1."),
                created_at: String::from("2021-01-01T00:00:00Z"),
            }));
        let service = NoteServiceImpl::new(mock);
        let note = service.delete(delete_id).now_or_never().unwrap().unwrap();
        assert_eq!(note.id, delete_id);
    }
}
