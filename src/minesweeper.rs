use std::{fmt::Display, hash, u32};

pub enum OpenResult {
    Opening(u32),
    Explode,
}

pub trait MinCell: Display {
    fn open(&mut self) -> OpenResult;
}
pub struct MinedCell {
    open: bool,
}
impl MinedCell {
    pub fn new() -> Box<dyn MinCell> {
        return Box::new(MinedCell { open: false });
    }
}

impl MinCell for MinedCell {
    fn open(&mut self) -> OpenResult {
        self.open = true;
        OpenResult::Explode
    }
}

impl Display for MinedCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.open {
            write!(f, "*")
        } else {
            write!(f, "?")
        }
    }
}
pub struct EmptyCell {
    open: bool,
    mines_around: u32,
}
impl EmptyCell {
    pub fn new() -> Box<dyn MinCell> {
        Box::new(EmptyCell {
            mines_around: 0,
            open: false,
        })
    }
}
impl Default for EmptyCell {
    fn default() -> Self {
        Self {
            open: false,
            mines_around: Default::default(),
        }
    }
}

impl MinCell for EmptyCell {
    fn open(&mut self) -> OpenResult {
        self.open = true;
        OpenResult::Opening(self.mines_around)
    }
}

impl Display for EmptyCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.open {
            write!(f, "{}", self.mines_around)
        } else {
            write!(f, "?")
        }
    }
}

pub struct Minesweeper {
    mine_field: Vec<Vec<Box<dyn MinCell>>>,
}

impl Minesweeper {
    pub fn new(width: usize, _height: usize, _countt: usize) {
        let mut field = <Vec<Vec<Box<dyn MinCell>>>>::with_capacity(width + 2);
        for _w in 0..field.capacity() {
            let mut row = <Vec<Box<dyn MinCell>>>::with_capacity(width + 2);
            for w in 0..row.capacity() {
                row.push(EmptyCell::new());
            }
            field.push(row);
        }
        
    }
}
