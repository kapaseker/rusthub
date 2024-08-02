pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut num_counts: [i32; 3] = [0; 3];
    for x in nums.iter() {
        num_counts[*x as usize] += 1;
    }
    let mut i = 0;
    let mut j = 0;
    for a in 0..3 {
        let x = num_counts[j];
        for y in 0..x {
            nums[i] = j as i32;
            i += 1;
        }
        j += 1;
    }
}