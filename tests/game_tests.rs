#[cfg(test)]
mod tests {
    use minesweeper::minesweeper::{
        DummyRender, EmptyCell, FieldCell, MinedCell, Minesweeper, OpenResult, TextRender
    };
    #[test]
    fn cell_empty_test() {
        let ec: Box<dyn FieldCell> = EmptyCell::new();
        assert!(ec.to_string().eq("?"));
    }

    #[test]
    #[should_panic]
    fn newgame_panic_test() {
        Minesweeper::new(10, 10, 101, DummyRender {});
    }
   
    #[test]
    fn newgame_test() {
        Minesweeper::new(10, 10, 100,DummyRender {});
        Minesweeper::new(1, 1, 1,DummyRender {});
    }

    #[test]
    fn game_allopen_test() {
        let mut g = Minesweeper::new(40, 20, 20,DummyRender {});
        let (height, width) = g.get_width_height();
        let mut explode_count = 0;
        println!("{}", g);
        for y in 0..height {
            for x in 0..width {
                if let Some(OpenResult::Explode) = g.open(y, x) {
                    explode_count += 1;
                    println!("info: it's exploded total count {}", explode_count);
                }
            }
        }
        println!("{}", g);
        println!("With exploded mines={}", explode_count);
    }
    #[test]
    fn game_open_test() {
        let mut g = Minesweeper::new(20, 60, 3,DummyRender {});
        println!("{}", g);
        g.open(10, 10);
        println!("{}", g);
    }
    #[test]
    fn game_open_many_test() {
        let mut g = Minesweeper::new(20, 60, 100,DummyRender {});
        println!("{}", g);
        g.open(10, 10);
        println!("{}", g);
    }


    #[test]
    fn game_display_many_test() {
        let mut g = Minesweeper::new(20, 60, 100,TextRender {});
        println!();
        g.open(10, 10);
        println!();
        g.open(15 ,10);
        println!();
        g.open(19 ,30);
    }
    #[test]
    fn game_mark_test() {
        let mut g = Minesweeper::new(10, 15, 10,DummyRender {});
        let (height, width) = g.get_width_height();
        for y in 0..height / 2 {
            for x in 0..width / 2 {
                g.mark(y, x);
            }
        }
        println!("{}", g);
    }

    #[test]
    fn game_display_test() {
        let g = Minesweeper::new(15, 20, 60,DummyRender {});
        println!("{}", g);
    }
    #[test]
    fn cell_minded_test() {
        let ec: Box<dyn FieldCell> = MinedCell::new();
        assert!(ec.to_string().eq("?"));
    }
    #[test]
    fn emptycell_opened_test() {
        let mut ec: Box<dyn FieldCell> = EmptyCell::new();
        if let OpenResult::Explode = ec.open() {
            panic!()
        }
        assert!(ec.to_string().eq("0"));
    }
    #[test]
    fn minedcell_opened_test() {
        let mut ec: Box<dyn FieldCell> = MinedCell::new();
        if let OpenResult::Opening(_) = ec.open() {
            panic!()
        }
        assert!(ec.to_string().eq("*"));
    }
 
}
