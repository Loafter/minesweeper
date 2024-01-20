#[cfg(test)]
mod tests {
    use minesweeper::minesweeper::{EmptyCell, MinCell, MinedCell, Minesweeper, OpenResult};
    #[test]
    fn cell_empty_test() {
        let ec: Box<dyn MinCell> = EmptyCell::new();
        assert!(ec.to_string().eq("?"));
    }

    #[test]
    #[should_panic]
    fn newgame_panic_test() {
        let m = Minesweeper::new(10, 10, 101);
    }

    #[test]
    fn newgame_test() {
        Minesweeper::new(10, 10, 100);
        Minesweeper::new(1, 1, 1);
    }

    #[test]
    fn game_open_test() {
        let mut g = Minesweeper::new(30, 30, 60);
        let (height, width) = g.get_width_height();
        for y in 0..height {
            for x in 0..width {
                if let OpenResult::Explode = g.open(y, x) {
                    println!("info: it's exploded");
                    break;
                }
            }
        }
        println!("{}", g);
    }
    #[test]
    fn game_display_test() {
        let g = Minesweeper::new(30, 30, 60);
        println!("{}", g);
    }
    #[test]
    fn cell_minded_test() {
        let ec: Box<dyn MinCell> = MinedCell::new();
        assert!(ec.to_string().eq("?"));
    }
    #[test]
    fn emptycell_opened_test() {
        let mut ec: Box<dyn MinCell> = EmptyCell::new();
        if let OpenResult::Explode = ec.open() {
            panic!()
        }
        assert!(ec.to_string().eq("0"));
    }
    #[test]
    fn minedcell_opened_test() {
        let mut ec: Box<dyn MinCell> = MinedCell::new();
        if let OpenResult::Opening(_) = ec.open() {
            panic!()
        }
        assert!(ec.to_string().eq("*"));
    }
}
