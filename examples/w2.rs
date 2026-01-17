use kompost::compounds::*;
use kompost::*;

fn window_2d<'a, T, Iter1, Iter2>(
    size_x: usize,
    size_y: usize,
) -> Box<dyn FnOnce(Iter1) -> impl Iterator<Item = impl Iterator<Item = T>>>
where
    T: 'a,
    Iter1: ExactSizeIterator<Item = &'a [T]> + Clone,
    Iter2: Iterator<Item = T>,
{
    Box::new(move |it| {
        it.composed(|it| periodic_windows(size_x, it))
            .map(|row_window| {
                row_window.map(|row| {
                    row.iter()
                        .composed(|it| periodic_windows(size_y, it))
                        .composed(transpose2)
                })
            })
            .flatten()
    })
}

fn main() {
    [1, 2, 3, 4, 5, 6, 7, 8, 9]
        .chunks(3)
        .for_each(|i| println!("{i:?}"));
}
