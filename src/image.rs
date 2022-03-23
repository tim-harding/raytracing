use exr::prelude::Error as ExrError;
use glam::Vec4;
use std::io::Error as IoError;
use std::path::Path;
use std::result::Result;
use thiserror::Error;

pub const CHUNK_DIM: usize = 32;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    ExrSave(#[from] IoError),
    #[error("Unhandled EXR error")]
    ExrOther,
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
    data: Vec<Chunk>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let chunks_x = width / CHUNK_DIM + 1;
        let chunks_y = height / CHUNK_DIM + 1;
        let chunks = chunks_x * chunks_y;
        let data = vec![Chunk::default(); chunks];
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
            let vec = self.get(x, self.height - y - 1);
            (vec.x, vec.y, vec.z, vec.w)
        })?;
        Ok(())
    }

    fn get(&self, x: usize, y: usize) -> Vec4 {
        let chunks_x = self.width / CHUNK_DIM;
        let chunk_x = x / CHUNK_DIM;
        let chunk_y = y / CHUNK_DIM;
        let index = chunk_y * chunks_x + chunk_x;
        let chunk = self.data[index];
        let local_x = x % CHUNK_DIM;
        let local_y = y % CHUNK_DIM;
        chunk.0[local_y][local_x]
    }

    pub fn chunks_mut(&mut self) -> std::slice::IterMut<Chunk> {
        self.data.iter_mut()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Chunk([[Vec4; CHUNK_DIM]; CHUNK_DIM]);

impl std::ops::Index<usize> for Chunk {
    type Output = [Vec4; CHUNK_DIM];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Chunk {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
