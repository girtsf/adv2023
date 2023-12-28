use std::env;

mod range;

pub use range::{Range, Ranges};

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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

    // pub fn wrap(&self, size: &Pos) -> Pos {
    //     Pos(self.y % size.y, self.x % size.x)
    // }
    //

    pub fn orthogonal_neighbors(&self) -> [Pos; 4] {
        [
            self + &Pos::left(),
            self + &Pos::up(),
            self + &Pos::right(),
            self + &Pos::down(),
        ]
    }

    pub fn ccw(&self) -> Self {
        // (0, 1) -> (-1, 0)
        // (-1, 0) -> (0, -1)
        // (0, -1) -> (1, 0)
        // (1, 0) -> (0, 1)
        let (y, x) = match (self.y, self.x) {
            (0, t) if t != 0 => (-t, 0),
            (t, 0) if t != 0 => (0, t),
            _ => panic!("not a single dir before: {:?}", self),
        };
        Pos::new(y, x)
    }

    pub fn cw(&self) -> Self {
        // (0, 1) -> (1, 0)
        // (1, 0) -> (0, -1)
        // (0, -1) -> (-1, 0)
        // (-1, 0) -> (0, 1)
        let (y, x) = match (self.y, self.x) {
            (0, t) if t != 0 => (t, 0),
            (t, 0) if t != 0 => (0, -t),
            _ => panic!("not a single dir before: {:?}", self),
        };
        Pos::new(y, x)
    }

    pub fn opposite(&self) -> Self {
        Pos::new(-self.y, -self.x)
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
