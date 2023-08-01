use super::Axis;

pub type SizeUnit = u16;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Size {
    x: SizeUnit,
    y: SizeUnit,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SizeError {
    ZeroSize(Axis),
}

pub type SizeResult = Result<Size, SizeError>;

impl Size {
    pub fn new(x: SizeUnit, y: SizeUnit) -> SizeResult {
        match (x, y) {
            (0, _) => Err(SizeError::ZeroSize(Axis::X)),
            (_, 0) => Err(SizeError::ZeroSize(Axis::Y)),
            (x, y) => Ok(Size { x, y }),
        }
    }

    pub fn x(&self) -> SizeUnit {
        self.x
    }
    pub fn y(&self) -> SizeUnit {
        self.y
    }
}

impl TryFrom<(SizeUnit, SizeUnit)> for Size {
    type Error = SizeError;

    fn try_from(value: (SizeUnit, SizeUnit)) -> Result<Self, Self::Error> {
        Size::new(value.0, value.1)
    }
}
