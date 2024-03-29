use rand::seq::SliceRandom;
use rand::thread_rng;
use std::any::Any;
use std::collections::VecDeque;
use std::fmt::Display;
use std::usize;

#[derive(PartialEq)]
pub enum OpenResult {
    Opening(usize),
    Explode,
}

pub type MineField = Vec<Vec<Box<dyn FieldCell>>>;
pub trait GameRender {
    fn render(&mut self, _field: &MineField);
}
pub struct TextRender {}

impl GameRender for TextRender {
    fn render(&mut self, _field: &MineField) {
        for row in _field.iter() {
            for el in row {
                print!("{}", el);
            }
            println!("");
        }
    }
}

pub struct DummyRender {}

impl GameRender for DummyRender {
    fn render(&mut self, _field: &MineField) {}
}

pub trait FieldCell: Display {
    fn open(&mut self) -> OpenResult;
    fn mark(&mut self) -> bool;
    fn is_marked(&self) -> bool;
    fn is_mine(&self) -> bool;
    fn is_open(&self) -> bool;
    fn mines_arround(&self) -> usize;
    fn as_any(&mut self) -> &mut dyn Any;
}
pub struct MinedCell {
    open: bool,
    marked: bool,
}
impl MinedCell {
    pub fn new() -> Box<dyn FieldCell> {
        return Box::new(MinedCell {
            open: false,
            marked: false,
        });
    }
}

impl FieldCell for MinedCell {
    fn open(&mut self) -> OpenResult {
        self.open = true;
        OpenResult::Explode
    }

    fn mark(&mut self) -> bool {
        self.marked = !self.marked;
        return self.marked;
    }

    fn is_mine(&self) -> bool {
        true
    }

    fn is_marked(&self) -> bool {
        self.marked
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn mines_arround(&self) -> usize {
        1
    }
}

impl Display for MinedCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.open {
            write!(f, "*")
        } else if self.marked {
            write!(f, "!")
        } else {
            write!(f, "?")
        }
    }
}
#[derive(Clone)]
pub struct EmptyCell {
    open: bool,
    marked: bool,
    mines_around: usize,
}
impl EmptyCell {
    pub fn new() -> Box<dyn FieldCell> {
        Box::new(EmptyCell {
            mines_around: 0,
            marked: false,
            open: false,
        })
    }
}
impl Default for EmptyCell {
    fn default() -> Self {
        Self {
            open: false,
            marked: false,
            mines_around: Default::default(),
        }
    }
}

impl FieldCell for EmptyCell {
    fn open(&mut self) -> OpenResult {
        self.open = true;
        OpenResult::Opening(self.mines_around)
    }

    fn mark(&mut self) -> bool {
        self.marked = !self.marked;
        self.marked
    }

    fn is_mine(&self) -> bool {
        false
    }

    fn is_marked(&self) -> bool {
        self.marked
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn mines_arround(&self) -> usize {
        self.mines_around
    }
}

impl Display for EmptyCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.open {
            write!(f, "{}", self.mines_around)
        } else if self.marked {
            write!(f, "!")
        } else {
            write!(f, "?")
        }
    }
}

pub struct Minesweeper<'a, T: GameRender> {
    height: usize,
    width: usize,
    total_mines: i64,
    unarmed_mines: i64,
    placed_flags: i64,
    mine_field: MineField,
    render: &'a mut T,
}

impl<'a, T: GameRender> Display for Minesweeper<'a, T> {
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

impl<'a, T: GameRender> Minesweeper<'a, T> {
    pub fn new(
        height: usize,
        width: usize,
        mut _count: usize,
        render: &'a mut T,
    ) -> Minesweeper<'a, T> {
        let total_cells = height * width;
        if _count > total_cells {
            panic!("panic: to much mines")
        }

        let field = gen_field(height, width);
        let mut map_field = create_vec_cord(total_cells, height, width);
        let mut ret_field = Minesweeper {
            mine_field: field,
            height: height,
            width: width,
            total_mines: _count as i64,
            unarmed_mines: _count as i64,
            placed_flags: 0,
            render: render,
        };

        map_field.shuffle(&mut thread_rng());
        place_mine(_count, map_field, &mut ret_field);
        ret_field.refresh();
        ret_field
    }
    pub fn get_width_height(&self) -> (usize, usize) {
        (self.height, self.width)
    }

    fn check_border(&self, y: i64, x: i64) -> bool {
        x >= 0 && y >= 0 && x != self.width as i64 && y != self.height as i64
    }

    pub fn open(&mut self, y: usize, x: usize) -> Option<OpenResult> {
        if self.mine_field[y][x].is_marked() && !self.mine_field[y][x].is_open() {
            return None;
        }
        let res = self.mine_field[y][x].open();
        if let OpenResult::Opening(0) = res {
            let mut deq: VecDeque<(usize, usize)> = VecDeque::new();
            deq.push_back((y, x));
            while !deq.is_empty() {
                let cur_cell = deq.pop_front().unwrap();
                let (y, x) = (cur_cell.0 as i64, cur_cell.1 as i64);
                for j in y - 1..=y + 1 {
                    for i in x - 1..=x + 1 {
                        if self.check_border(j, i) {
                            let (j, i) = (j as usize, i as usize);
                            if !self.mine_field[j][i].is_mine() && !self.mine_field[j][i].is_open()
                            {
                                self.mine_field[j][i].open();
                                if self.mine_field[j][i].mines_arround() == 0 {
                                    deq.push_front((j, i));
                                }
                            }
                        }
                    }
                }
            }
        }
        self.refresh();
        return Some(res);
    }
    pub fn open_all(&mut self) {
        for y in 0..self.mine_field.len() {
            for x in 0..self.mine_field[0].len() {
                if self.mine_field[y][x].is_mine() {
                    self.mine_field[y][x].open();
                }
            }
        }
        self.refresh();
    }
    fn checkwin(&self) -> bool {
        self.unarmed_mines == 0 && self.placed_flags == self.total_mines.try_into().unwrap()
    }

    pub fn mark(&mut self, y: usize, x: usize) -> bool {
        let cell = &mut self.mine_field[y][x];
        if !cell.is_open() {
            if cell.mark() {
                self.placed_flags += 1;
                if cell.is_mine() {
                    self.unarmed_mines -= 1;
                }
                self.refresh();
                return self.checkwin();
            } else {
                if cell.is_mine() {
                    self.unarmed_mines += 1;
                }
                self.placed_flags -= 1;
                self.refresh();
                return self.checkwin();
            }
        }
        return false;
    }
    pub fn refresh(&mut self) {
        self.render.render(&self.mine_field);
    }
}

fn place_mine<T: GameRender>(
    mut _count: usize,
    mut map_field: Vec<(i64, i64)>,
    ret_field: &mut Minesweeper<T>,
) {
    while _count > 0 {
        let (j, i) = map_field.pop().unwrap();
        ret_field.mine_field[j as usize][i as usize] = MinedCell::new();
        _count -= 1;
        for y in j - 1..=j + 1 {
            for x in i - 1..=i + 1 {
                if ret_field.check_border(y, x) {
                    if !ret_field.mine_field[y as usize][x as usize].is_mine() {
                        let cell = ret_field.mine_field[y as usize][x as usize]
                            .as_any()
                            .downcast_mut::<EmptyCell>()
                            .unwrap();
                        cell.mines_around += 1;
                    }
                }
            }
        }
    }
}

fn create_vec_cord(total_cells: usize, height: usize, width: usize) -> Vec<(i64, i64)> {
    let mut map_field = Vec::<(i64, i64)>::with_capacity(total_cells);
    for y in 0..height as i64 {
        for x in 0..width as i64 {
            map_field.push((y, x));
        }
    }
    map_field
}

fn gen_field(height: usize, width: usize) -> MineField {
    let mut field = MineField::with_capacity(height);
    for _h in 0..field.capacity() {
        let mut row = <Vec<Box<dyn FieldCell>>>::with_capacity(width);
        for _w in 0..row.capacity() {
            row.push(EmptyCell::new());
        }
        field.push(row);
    }
    field
}
