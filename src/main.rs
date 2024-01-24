#![windows_subsystem = "windows"]
use fltk::app::Sender;
use fltk::group::Group;
use fltk::{menu::MenuBar, prelude::*, *};

use fltk::window::*;
use minesweeper::fltfieldrender::{FltkRender, WinMessage};
use minesweeper::minesweeper::Minesweeper;

const MENU_HEIGHT: i32 = 20;
const MINE_SIZE: i32 = 20;
pub fn make_window(channel: &Sender<WinMessage>) -> (DoubleWindow, MenuBar, Group) {
    let mut main_window = Window::new(277, 266, 300, 300, None);
    main_window.set_label(r#"Fltk rust Minsweeper"#);

    main_window.set_type(WindowType::Double);
    let mut game_menu = MenuBar::new(0, 0, 700, MENU_HEIGHT, None);
    let idx = game_menu.add_choice(r#"Game/New"#);
    let channel1 = channel.clone();
    game_menu.at(idx).unwrap().set_callback({
        move |_| {
            channel1.send(WinMessage::NewGame);
        }
    });
    let idx = game_menu.add_choice(r#"Game/Close"#);
    let channel2 = channel.clone();
    game_menu.at(idx).unwrap().set_callback({
        move |_| {
            channel2.send(WinMessage::Close);
        }
    });
    game_menu.end();
    let  mine_fied = Group::new(0, MENU_HEIGHT, 500, 700, None);
    mine_fied.end();
    main_window.end();
    main_window.show();
    (main_window, game_menu, mine_fied)
}

fn main() {
    let app = app::App::default();
    let (win_mes_sender, win_mes_reciver) = app::channel::<WinMessage>();
    let (mut main_window, _game_menu, mut mine_fied) = make_window(&win_mes_sender);

    let width = 40;
    let height = 20;
    let mins = 20;
     let mut render = FltkRender::new(
        height as usize,
        width as usize,
        MENU_HEIGHT,
        win_mes_sender.clone(),
        MINE_SIZE,
        &mut mine_fied,
    );
    let mut game = Minesweeper::new(height as usize, width as usize, mins, &mut render);
    main_window.redraw();
    while app.wait() {
        if let Some(msg) = win_mes_reciver.recv() {
            match msg {
                WinMessage::Close => {
                    return;
                }
                WinMessage::NewGame => {
                   render.clearall();
                    main_window.set_size(width * MINE_SIZE, MENU_HEIGHT + height * MINE_SIZE);
                    render = FltkRender::new(
                        height as usize,
                        width as usize,
                        MENU_HEIGHT,
                        win_mes_sender.clone(),
                        MINE_SIZE,
                        &mut mine_fied,
                    );
                    main_window.redraw();
                    game = Minesweeper::new(height as usize, width as usize, mins, &mut render);
                }
                WinMessage::ClickOnCord(y, x) => {
                    if app::event_mouse_button() == app::MouseButton::Right {
                        dialog::alert(1200, 900, &format!("y={} x={}", y, x));
                    } else {
                        game.open(y, x);
                    }
                }
            }
        }
    }
}
