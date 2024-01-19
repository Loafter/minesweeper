#[cfg(test)]
mod tests {
    use minesweeper::minesweeper::{EmptyCell, MinCell, MinedCell, OpenResult};
    #[test]
    fn cell_empty_test() {
        let ec: Box<dyn MinCell> = EmptyCell::new();
        assert!(ec.to_string().eq("?"));
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
