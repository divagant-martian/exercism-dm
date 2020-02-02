pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            // Find all max points in this row
            let mut max_pos = Vec::<(usize, usize)>::new();
            if let Some(mut local_max) = row.first() {
                for (j, elem) in row.iter().enumerate() {
                    if elem == local_max {
                        max_pos.push((i, j));
                    }
                    if elem > local_max {
                        local_max = elem;
                        max_pos.clear();
                        max_pos.push((i, j));
                    }
                }
            }
            max_pos
        })
        .filter(|&(i, j)| {
            // Check that the maxs of each row are also mins in their columns
            let sp = input[i][j];
            input.iter().all(|vec| sp <= vec[j])
        })
        .collect()
}
