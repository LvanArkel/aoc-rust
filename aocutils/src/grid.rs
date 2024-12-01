use std::{fmt::Debug, slice::Iter};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn delta(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    pub fn clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn counter_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South, 
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn values() -> [Direction; 4] {
        [Direction::North, Direction::East, Direction::South, Direction::West]
    }
}

pub struct Grid<T> {
    contents: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn new(contents: Vec<T>, height: usize, width: usize) -> Self { 
        assert_eq!(contents.len(), width * height);
        if contents.len() != width * height {
            panic!("Contents size differs from bounds: {} != {}*{}", contents.len(), width, height);
        }
        Self { contents, width, height }
    }

    pub fn from_2d(contents: Vec<Vec<T>>) -> Self {
        if contents.is_empty() {
            panic!("Contents cannot be empty");
        }
        let height = contents.len();
        let width = contents[0].len();
        if width == 0 || contents.iter().any(|line| line.len() != width) {
            panic!("Widths are zero not not equal to each other");
        }
        Self {
            contents : contents.into_iter().flatten().collect(),
            width, height
        }
    }

    pub fn in_bounds(&self, row: i32, col: i32) -> bool {
        row >= 0 && row < self.height as i32 && col >= 0 && col < self.width as i32
    }

    pub fn get(&self, row: i32, col: i32) -> Option<&T> {
        if self.in_bounds(row, col) {
            Some(&self.contents[row as usize*self.width + col as usize])
        } else {
            None
        }
    }

    pub fn get_element(&self, i: usize) -> Option<&T> {
        self.contents.get(i)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height (&self) -> usize {
        self.height
    }

    pub fn set(&mut self, row: i32, col: i32, val: T) -> bool {
        if self.in_bounds(row, col) {
            self.contents[row as usize * self.width + col as usize] = val;
            return true;
        }
        return false;
    }
}

impl<T> Grid<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        self.contents.iter()
    }
}

impl<T: Copy> Grid<T> {
    pub fn from_value(height: usize, width: usize, value: T) -> Self {
        let contents = vec![value; width*height];
        Self { contents, width, height }
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Grid {}x{}\n", self.height, self.width)?;
        for row in 0..self.height {
            let slice = &self.contents[(row*self.width)..((row+1)*self.width)];
            write!(f, "{slice:?}\n")?;
        }
        Ok(())
    }
}
