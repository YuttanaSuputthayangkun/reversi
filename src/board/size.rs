use super::Axis;

#[derive(Debug, PartialEq, Eq)]
pub struct Size {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SizeError {
    ZeroSize(Axis),
}

pub type SizeResult = Result<Size, SizeError>;

impl Size {
    pub fn new(x: usize, y: usize) -> SizeResult {
        match (x, y) {
            (0, _) => Err(SizeError::ZeroSize(Axis::X)),
            (_, 0) => Err(SizeError::ZeroSize(Axis::Y)),
            (x, y) => Ok(Size { x, y }),
        }
    }

    pub fn x(&self) -> usize {
        self.x
    }
    pub fn y(&self) -> usize {
        self.y
    }
}

impl TryFrom<(usize, usize)> for Size {
    type Error = SizeError;

    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        Size::new(value.0, value.1)
    }
}
