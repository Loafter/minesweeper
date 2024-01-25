use fltk::{
    app, button,

    frame, group, input,
    prelude::*,
    window,
};

pub fn show_dialog() -> NewGameDialog {
    NewGameDialog::default()
}

pub struct NewGameDialog {
    winp: input::Input,
    hinp: input::Input,
    minp: input::Input,
}

impl NewGameDialog {
    pub fn default() -> Self {
        let mut win = window::Window::default()
            .with_size(560, 50)
            .with_label("New Game");
        let mut pack = group::Pack::default()
            .with_size(100, 30)

            .with_type(group::PackType::Horizontal);
        pack.set_spacing(20);
        frame::Frame::default()
            .with_size(80, 0)
            .with_label("Game Param:");
        let mut winp = input::Input::default().with_size(100, 0);
        let mut hinp = input::Input::default().with_size(100, 0);
        let mut minp = input::Input::default().with_size(100, 0);
        winp.set_value("60");
        hinp.set_value("40");
        minp.set_value("150");
        let mut ok = button::Button::default()
            .with_size(80, 0)
            .with_label("Start!");

        pack.end();
        win.end();
        win.make_modal(true);
        win.show();
        ok.set_callback({
            let mut win = win.clone();
            move |_| {
                win.hide();
            }
        });
        while win.shown() {
            app::wait();
        }
        Self {
            winp: winp,
            hinp: hinp,
            minp: minp,
        }
    }

    pub fn value(&self) -> (i32,i32,usize){
        let w: i32 = self.winp.value().parse().unwrap();
        let h: i32 = self.hinp.value().parse().unwrap();
        let m: usize = self.minp.value().parse().unwrap();
        (w,h,m)
    } 
}
