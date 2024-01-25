#![windows_subsystem = "windows"]
use fltk::app::Sender;
use fltk::group::{Flex, FlexType, Group};
use fltk::{menu::MenuBar, prelude::*, *};

use fltk::window::*;
use minesweeper::fltfieldrender::{FltkRender, WinMessage};
use minesweeper::minesweeper::{Minesweeper, OpenResult};
use minesweeper::newfielddialog::NewGameDialog;

const MENU_HEIGHT: i32 = 20;
const MINE_SIZE: i32 = 20;
const DEFAULT_FIELD_WIDTH: i32 = 30;
const DEFAULT_FIELD_HEIGHT: i32 = 20;
const DEFAULT_MINS: usize = 60;

pub fn make_window(channel: &Sender<WinMessage>) -> (DoubleWindow, MenuBar, Group) {
    let mut main_window = Window::new(650, 450, DEFAULT_FIELD_WIDTH * MINE_SIZE,MENU_HEIGHT + DEFAULT_FIELD_HEIGHT * MINE_SIZE, None);
    main_window.set_label(r#"FLTK rust Minsweeper"#);
    main_window.set_type(WindowType::Double);
    let mut flex_frame = Flex::new(0, 0, DEFAULT_FIELD_WIDTH*MINE_SIZE, MENU_HEIGHT + DEFAULT_FIELD_HEIGHT * MINE_SIZE, None);
    flex_frame.set_type(FlexType::Column);
    let mut game_menu = MenuBar::new(0, 0, DEFAULT_FIELD_WIDTH*MINE_SIZE, MENU_HEIGHT, None);
    let idx = game_menu.add_choice(r#"Game/New"#);
    let channel1 = channel.clone();
    game_menu.at(idx).unwrap().set_callback({
        move |_| {
            channel1.send(WinMessage::UpdateGameParam);
        }
    });
    let idx = game_menu.add_choice(r#"Game/Reset"#);
    let channel2 = channel.clone();
    game_menu.at(idx).unwrap().set_callback({
        move |_| {
            channel2.send(WinMessage::NewGame);
        }
    });

    let idx = game_menu.add_choice(r#"Help/About"#);
    game_menu.at(idx).unwrap().set_callback({
        move |_| {
            dialog::alert_default(&format!("Created by Andrew Rose:)"));
        }
    });


    

    let idx = game_menu.add_choice(r#"Game/Close"#);
    let channel3 = channel.clone();
    game_menu.at(idx).unwrap().set_callback({
        move |_| {
            channel3.send(WinMessage::Close);
        }
    });
    game_menu.end();
    let mine_fied = Group::new(0, MENU_HEIGHT, DEFAULT_FIELD_WIDTH * MINE_SIZE, MENU_HEIGHT + DEFAULT_FIELD_HEIGHT * MINE_SIZE, None);
    mine_fied.end();
    flex_frame.end();
    flex_frame.fixed(&flex_frame.child(0).unwrap(), 20);
    flex_frame.recalc();
    main_window.make_resizable(true);
    main_window.end();
    main_window.show();
    (main_window, game_menu, mine_fied)
}

fn main() {
    let app = app::App::default();
    let (win_mes_sender, win_mes_reciver) = app::channel::<WinMessage>();
    let (mut main_window, _game_menu, mut mine_fied) = make_window(&win_mes_sender);

    let mut height = DEFAULT_FIELD_HEIGHT;
    let mut width = DEFAULT_FIELD_WIDTH;
    let mut mins = DEFAULT_MINS;
    let mut render = FltkRender::new(
        height as usize,
        width as usize,
        MENU_HEIGHT,
        win_mes_sender.clone(),
        MINE_SIZE,
        &mut mine_fied,
    );
    let mut game = Minesweeper::new(height as usize, width as usize, mins, &mut render);
    while app.wait() {
        if let Some(msg) = win_mes_reciver.recv() {
            match msg {
                WinMessage::Close => {
                    return;
                }
                WinMessage::NewGame => {
                    render.clearall();
                    main_window.set_size(width * MINE_SIZE, MENU_HEIGHT + height * MINE_SIZE);
                    mine_fied.set_size(width * MINE_SIZE, height * MINE_SIZE);
                    main_window.redraw();
                    render = FltkRender::new(
                        height as usize,
                        width as usize,
                        MENU_HEIGHT,
                        win_mes_sender.clone(),
                        MINE_SIZE,
                        &mut mine_fied,
                    );
                    game = Minesweeper::new(height as usize, width as usize, mins, &mut render);
                }
                WinMessage::ClickOnCord(y, x) => {
                    if app::event_mouse_button() == app::MouseButton::Right {
                        if game.mark(y, x) {
                            dialog::alert_default(&format!("You Win"));
                            win_mes_sender.send(WinMessage::NewGame);
                        }
                    } else {
                        if let Some(open_res) = game.open(y, x) {
                            if open_res == OpenResult::Explode {
                                game.open_all();
                                dialog::alert_default(&format!("Kaboom!!!!!!"));
                                win_mes_sender.send(WinMessage::NewGame);
                            }
                        }
                    }
                }
                WinMessage::UpdateGameParam => {
                    let newd = NewGameDialog::default();
                    (width, height, mins) = newd.value();
                    win_mes_sender.send(WinMessage::NewGame);
                    
                },
            }
        }
    }
}
