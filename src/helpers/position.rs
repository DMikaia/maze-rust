pub fn in_bounds(size: usize, point: (usize, usize)) -> bool {
    point.0 < size && point.1 < size
}
