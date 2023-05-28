use super::{Board, Size};

#[derive(Debug, Default)]
pub struct Builder {
    size: Option<Size>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BuildError {
    MissingSize,
}

pub type BuildResult<Cell> = Result<Board<Cell>, BuildError>;

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, s: Size) -> Self {
        self.size = Some(s);
        self
    }

    pub fn build<Cell: Default>(self) -> BuildResult<Cell> {
        let size = self.size.ok_or_else(|| BuildError::MissingSize)?;
        let board = Board::<Cell>::new(size);
        BuildResult::Ok(board)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok() {
        assert!(Builder::new()
            .size(Size::new(1, 1).unwrap())
            .build::<()>()
            .is_ok());
    }

    #[test]
    fn missing_size() {
        let result = Builder::new().build::<()>();
        assert_eq!(result, BuildResult::Err(BuildError::MissingSize));
    }
}
