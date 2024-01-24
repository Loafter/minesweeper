use std::usize;

use crate::minesweeper::{GameRender, MineField};
use fltk::{
    app::Sender,
    button::Button,
    group::Group,
    prelude::{GroupExt, WidgetBase, WidgetExt},
};

pub enum WinMessage {
    Close,
    NewGame,
    ClickOnCord(usize, usize),
}

pub struct FltkRender<'a> {
    button_field: Vec<Vec<Button>>,
    mine_fied: &'a mut Group,
}



impl  FltkRender<'_>  {
    pub fn new(
        height: usize,
        width: usize,
        menu_height: i32,
        sender: Sender<WinMessage>,
        mine_size: i32,
        mine_fied: &mut Group,
    ) -> FltkRender {
        let mut butt_field = Vec::<Vec<Button>>::with_capacity(height);
        for _h in 0..butt_field.capacity() {
            let mut row = <Vec<Button>>::with_capacity(width);
            for _w in 0..row.capacity() {
                let y = _h as i32 * mine_size + menu_height;
                let x = _w as i32 * mine_size;
                let mut b = Button::new(x, y, mine_size, mine_size, "");
                let sender = sender.clone();
                b.set_callback(move |_| {
                    sender.send(WinMessage::ClickOnCord(_h, _w));
                });
                mine_fied.add(&b);
                row.push(b);
            }
            butt_field.push(row);
        }
        FltkRender {
            button_field: butt_field,
            mine_fied:mine_fied
        }
    }
    pub fn clearall(&mut self){
        for ele in &self.button_field {
            for e in ele {
                self.mine_fied.remove(e)
            }
        }
    }
}

impl GameRender for FltkRender<'_> {
    fn render(&mut self, _field: &MineField) {
        for y in 0.._field.len() {
            for x in 0.._field[y].len() {
                let cell = _field[y][x].as_ref();
                if cell.is_marked() {
                    self.button_field[y][x].set_label("!");
                } else if cell.is_open() {
                    if cell.is_mine() {
                        self.button_field[y][x].set_label("*");
                    } else if cell.mines_arround() > 0 {
                        self.button_field[y][x].set_label(&cell.mines_arround().to_string());
                    } else {
                        self.button_field[y][x].hide();
                    }
                }
            }
        }
    }
}
