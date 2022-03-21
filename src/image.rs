use exr::prelude::Error as ExrError;
use glam::Vec3;
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
            self.get(x, self.height - y - 1).as_tuple()
        })?;
        Ok(())
    }

    fn get(&self, x: usize, y: usize) -> Rgba {
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
pub struct Chunk([[Rgba; CHUNK_DIM]; CHUNK_DIM]);

impl std::ops::Index<usize> for Chunk {
    type Output = [Rgba; CHUNK_DIM];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Chunk {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn as_tuple(&self) -> (f32, f32, f32, f32) {
        (self.r, self.g, self.b, self.a)
    }
}

impl From<Vec3> for Rgba {
    fn from(vec: Vec3) -> Self {
        Self {
            r: vec.x,
            g: vec.y,
            b: vec.z,
            a: 1.0,
        }
    }
}

impl std::ops::Index<usize> for Rgba {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            3 => &self.a,
            _ => panic!(),
        }
    }
}

impl std::ops::IndexMut<usize> for Rgba {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            3 => &mut self.a,
            _ => panic!(),
        }
    }
}
