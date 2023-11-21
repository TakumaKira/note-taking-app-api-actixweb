use anyhow::Result;
use async_trait::async_trait;
use db::NewNote;
use db::Note;
use db::NoteRepository;
use db::UpdateNote

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
