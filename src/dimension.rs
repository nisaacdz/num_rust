pub struct Dimension(isize, isize);

impl Dimension {
    pub fn width(&self) -> isize {
        self.0
    }

    pub fn height(&self) -> isize {
        self.1
    }

    pub fn new(width: isize, height: isize) -> Self {
        Self(width, height)
    }

    pub fn len(&self) -> isize {
        self.width() * self.height()
    }
}