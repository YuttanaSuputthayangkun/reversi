use super::{Board, Size};

#[derive(Debug, Default)]
pub struct Builder {
    size: Option<Size>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BuildError {
    MissingSize,
}

pub type BuildResult = Result<Board, BuildError>;

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, s: Size) -> Self {
        self.size = Some(s);
        self
    }

    pub fn build(self) -> BuildResult {
        let board = Board {
            size: self.size.ok_or_else(|| BuildError::MissingSize)?,
        };
        BuildResult::Ok(board)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn missing_size() {
        let result = Builder::new().build();
        assert_eq!(result, BuildResult::Err(BuildError::MissingSize));
    }
}
