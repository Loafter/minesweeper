#![windows_subsystem = "windows"]
use fltk::app::Sender;
use fltk::button::Button;
use fltk::draw::height;
use fltk::group::{Flex, FlexType, Group};
use fltk::menu::MenuBar;
use fltk::{prelude::*, *};

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
    let mut mine_fied = Group::new(0, MENU_HEIGHT, 0, 0, None);
    mine_fied.end();
    main_window.end();
    main_window.show();
    (main_window, game_menu, mine_fied)
}

fn main() {
    let app = app::App::default();
    let (win_mes_sender, win_mes_reciver) = app::channel::<WinMessage>();
    let (mut main_window, mut game_menu, mut mine_fied) = make_window(&win_mes_sender);
    let mut game = None;
    let width = 40;
    let height = 20;
    let mins = 20;
    while app.wait() {
        if let Some(msg) = win_mes_reciver.recv() {
            match msg {
                WinMessage::Close => {
                    return;
                }
                WinMessage::NewGame => {
                    let win_mes_sender = win_mes_sender.clone();
                    main_window.set_size(width * MINE_SIZE, MENU_HEIGHT + height * MINE_SIZE);
                    mine_fied.set_size(width * MINE_SIZE, MENU_HEIGHT + height * MINE_SIZE);
                    let render = FltkRender::new(
                        height as usize,
                        width as usize,
                        MENU_HEIGHT,
                        win_mes_sender,
                        MINE_SIZE,
                        &mut mine_fied,
                    );
                    game = Some(Minesweeper::new(
                        height as usize,
                        width as usize,
                        mins,
                        render,
                    ));
                    main_window.redraw();
                }
                WinMessage::ClickOnCord(y, x) => {
                    if app::event_mouse_button() == app::MouseButton::Right {
                        dialog::alert(1200, 900, &format!("y={} x={}", y, x));
                    } else {
                        if let Some(ref mut _g) = game {
                            _g.open(y, x);
                        }
                 
                    }
                }
            }
        }
    }
}
