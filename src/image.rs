use exr::prelude::Error as ExrError;
use std::collections::btree_map::IterMut;
use std::io::Error as IoError;
use std::path::Path;
use std::result::Result;
use std::sync::{PoisonError, RwLock, RwLockReadGuard};
use thiserror::Error;

const CHUNK_DIM: usize = 32;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    ExrSave(#[from] IoError),
    #[error("Unhandled EXR error")]
    ExrOther,
    #[error("Could not read lock")]
    ReadLock,
}

impl From<PoisonError<RwLockReadGuard<'_, Chunk>>> for Error {
    fn from(_: PoisonError<RwLockReadGuard<'_, Chunk>>) -> Self {
        Error::ReadLock
    }
}

impl From<ExrError> for Error {
    fn from(error: ExrError) -> Self {
        match error {
            ExrError::Io(error) => error.into(),
            _ => Self::ExrOther,
        }
    }
}

#[derive(Debug, Default)]
pub struct Image {
    data: Vec<RwLock<Chunk>>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let chunks_x = width / CHUNK_DIM + 1;
        let chunks_y = height / CHUNK_DIM + 1;
        let chunks = chunks_x * chunks_y;
        let mut data = Vec::with_capacity(chunks);
        for _ in 0..chunks {
            data.push(RwLock::new(Chunk::default()));
        }
        Self {
            data,
            width,
            height,
        }
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        self.save_inner(path.as_ref())
    }

    #[inline(never)]
    fn save_inner(&self, path: &Path) -> Result<(), Error> {
        exr::prelude::write_rgba_file(path, self.width, self.height, |x, y| {
            self.get(x, y).unwrap_or(Rgba::default()).as_tuple()
        })?;
        Ok(())
    }

    fn get(&self, x: usize, y: usize) -> Result<Rgba, Error> {
        let chunks_x = self.width / CHUNK_DIM;
        let chunk_x = x / CHUNK_DIM;
        let chunk_y = y / CHUNK_DIM;
        let index = chunk_y * chunks_x + chunk_x;
        let chunk = self.data[index].read()?;
        let local_x = x % CHUNK_DIM;
        let local_y = y % CHUNK_DIM;
        Ok(chunk.0[local_y][local_x])
    }

    pub fn chunks_mut(&mut self) -> std::slice::IterMut<RwLock<Chunk>> {
        self.data.iter_mut()
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Chunk([[Rgba; CHUNK_DIM]; CHUNK_DIM]);

#[derive(Debug, Default, Copy, Clone)]
pub struct Rgba([f32; 4]);

impl Rgba {
    pub fn as_tuple(&self) -> (f32, f32, f32, f32) {
        (self.0[0], self.0[1], self.0[2], self.0[3])
    }
}
