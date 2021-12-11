pub fn get_adjacent_indices(i: usize, size: usize, length: usize, diagonal: bool) -> Vec<usize> {
    let mut indices = Vec::new();
    let ad = [
        top(i, size),
        left(i, size),
        right(i, size),
        bottom(i, size, length),
    ];
    if ad[0] { indices.push(i - size) }
    if ad[1] { indices.push(i - 1) }
    if ad[2] { indices.push(i + 1) }
    if ad[3] { indices.push(i + size) }
    if diagonal {
        if ad[0] && ad[1] { indices.push(i - size - 1) }
        if ad[3] && ad[1] { indices.push(i + size - 1) }
        if ad[0] && ad[2] { indices.push(i - size + 1) }
        if ad[3] && ad[2] { indices.push(i + size + 1) }
    }
    indices
}

fn top(i: usize, size: usize) -> bool {
    i > size - 1
}

fn left(i: usize, size: usize) -> bool {
    i % size > 0
}

fn right(i: usize, size: usize) -> bool {
    i % size != size - 1
}

fn bottom(i: usize, size: usize, map_size: usize) -> bool {
    i < map_size - size
}
