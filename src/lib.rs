use std::env;

pub fn read_input() -> String {
    if env::var("RUST_BACKTRACE").is_err() {
        env::set_var("RUST_BACKTRACE", "1");
    }
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    let path = std::env::args().nth(1).expect("pls provide input file");
    std::fs::read_to_string(path).expect("read failed")
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub y: isize,
    pub x: isize,
}

impl Pos {
    pub fn new<T>(y: T, x: T) -> Self
    where
        T: TryInto<isize>,
    {
        let y = y.try_into().unwrap_or_else(|_| panic!());
        let x = x.try_into().unwrap_or_else(|_| panic!());
        Self { y, x }
    }

    pub fn up() -> Self {
        Pos::new(-1, 0)
    }

    pub fn down() -> Self {
        Pos::new(1, 0)
    }

    pub fn left() -> Self {
        Pos::new(0, -1)
    }

    pub fn right() -> Self {
        Pos::new(0, 1)
    }

    pub fn check_bounds(&self, size: &Pos) -> bool {
        self.x >= 0 && self.x < size.x && self.y >= 0 && self.y < size.y
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}

impl std::ops::Add for &Pos {
    type Output = Pos;

    fn add(self, rhs: &Pos) -> Self::Output {
        Pos::add(*self, *rhs)
    }
}

impl std::ops::Add<&Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: &Pos) -> Self::Output {
        Pos::add(self, *rhs)
    }
}

impl std::ops::AddAssign<&Pos> for Pos {
    fn add_assign(&mut self, rhs: &Pos) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
