type Input = Vec<Option<u32>>;

pub fn parse(input: &str) -> Input {
    // Parse the disk map from the input string into a vector of blocks.
    let chars: Vec<char> = input.trim().chars().collect();
    let mut disk = Vec::new();
    let mut is_file = true;
    let mut file_id = 0;
    for ch in chars {
        let count = ch.to_digit(10).unwrap_or(0);
        if is_file {
            disk.extend(std::iter::repeat(Some(file_id)).take(count as usize));
            file_id += 1;
        } else {
            disk.extend(std::iter::repeat(None).take(count as usize));
        }
        is_file = !is_file;
    }
    disk
}

pub fn part1(input: &Input) -> usize {
    let mut disk = input.clone();
    // Compact the disk by moving file blocks from the end to the leftmost free block.
    while let Some(free_idx) = disk.iter().position(|b| b.is_none()) {
        if let Some(file_idx) = disk.iter().rposition(|b| b.is_some()) {
            if free_idx < file_idx {
                // - Move the file block at file_idx to free_idx.
                disk[free_idx] = disk[file_idx].take();
            } else {
                break;
            }
        } else {
            break;
        }
    }
    // Compute the checksum: sum(index * file id) for all file blocks.
    disk.into_iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|id| i * id as usize))
        .sum()
}

pub fn part2(input: &Input) -> usize {
    let mut disk = input.clone();
    // Determine the maximum file ID.
    let max_file = disk.iter().filter_map(|&b| b).max().unwrap_or(0);
    // Process files in descending order.
    for file_id in (0..=max_file).rev() {
        // Locate the file's contiguous segment.
        if let Some(start) = disk.iter().position(|&b| b == Some(file_id)) {
            let mut end = start;
            while end < disk.len() && disk[end] == Some(file_id) {
                end += 1;
            }
            let file_len = end - start;
            // Find the leftmost free span before the file's current start.
            let mut span_start = None;
            let mut current_start = None;
            for i in 0..start {
                if disk[i].is_none() {
                    if current_start.is_none() {
                        current_start = Some(i);
                    }
                } else if let Some(s) = current_start {
                    if i - s >= file_len {
                        span_start = Some(s);
                        break;
                    }
                    current_start = None;
                }
            }
            // Check at end of loop if a span was tentatively started.
            if span_start.is_none() {
                if let Some(s) = current_start {
                    if start - s >= file_len {
                        span_start = Some(s);
                    }
                }
            }
            // If a valid span is found, move the file.
            if let Some(new_start) = span_start {
                // Only consider spans entirely to the left.
                if new_start + file_len <= start {
                    // Place the file at the new span.
                    for i in new_start..new_start + file_len {
                        disk[i] = Some(file_id);
                    }
                    // Clear the old file segment.
                    for i in start..end {
                        disk[i] = None;
                    }
                }
            }
        }
    }
    // Compute and return the checksum.
    disk.into_iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|id| i * id as usize))
        .sum()
}
