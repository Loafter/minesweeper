fn main() {
    let mut ve: Vec<Vec<usize>> =<Vec<Vec<usize>>>::with_capacity(10);
    for _w in 0..ve.capacity() {
        let mut row=<Vec<usize>>::with_capacity(10);
        for w in 0..row.capacity() {
            row.push(w)
        }
        ve.push(row);

     }
     println!("names: {:?}", ve);
 }