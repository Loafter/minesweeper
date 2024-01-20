use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt::Display;
use std::usize;
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
#[derive(Clone)]
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
    height: usize,
    width: usize,
    mine_field: Vec<Vec<Box<dyn MinCell>>>,
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.mine_field.iter() {
            for el in row {
                write!(f, "{}", el)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mut _count: usize) -> Minesweeper {
        let total_cells = height * width;
        if _count > total_cells {
            panic!("panic: to much mines")
        }

        let mut field = <Vec<Vec<Box<dyn MinCell>>>>::with_capacity(height);
        for _h in 0..field.capacity() {
            let mut row = <Vec<Box<dyn MinCell>>>::with_capacity(width);
            for _w in 0..row.capacity() {
                row.push(EmptyCell::new());
            }
            field.push(row);
        }
        let mut map_field = Vec::<(usize, usize)>::with_capacity(total_cells);
        for y in 0..height {
            for x in 0..width {
                map_field.push((y, x));
            }
        }

        map_field.shuffle(&mut thread_rng());
        while _count > 0 {
            let cord = map_field.last().unwrap();
            field[cord.0][cord.1] = MinedCell::new();
            map_field.pop();
            _count -= 1;
        }
        Minesweeper {
            mine_field: field,
            height: height,
            width: width,
        }
    }
    pub fn get_width_height(&self) -> (usize, usize) {
        ( self.height,self.width)
    }
    pub fn open(&mut self,  y: usize,x: usize) -> OpenResult {
        self.mine_field[y][x].open()
    }
}
