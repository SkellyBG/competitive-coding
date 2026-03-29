pub fn sortable_integers(nums: Vec<i32>) -> i32 {
    let mut total = 0;
    let n = nums.len();
    for i in 1..=n {
        if n % i != 0 {
            continue;
        }

        total += solve(&nums, i);
    }

    total
}

fn solve(nums: &[i32], i: usize) -> i32 {
    let mut prev_max = -1;
    // println!("{}", i);
    for chunk in nums.chunks(i) {
        if let Some((min, max)) = spin(chunk) {
            if min < prev_max {
                return 0;
            }
            prev_max = max;
        } else {
            return 0;
        }
    }

    i as i32
}

fn spin(nums: &[i32]) -> Option<(i32, i32)> {
    let mut min_idx = 0;
    for (idx, num) in nums.iter().enumerate() {
        if *num < nums[min_idx] {
            min_idx = idx;
        }
    }

    let original_min_idx = min_idx;
    while nums[(min_idx - 1 + nums.len()) % nums.len()] == nums[min_idx]
        && (min_idx - 1 + nums.len()) % nums.len() != original_min_idx
    {
        min_idx = (min_idx - 1 + nums.len()) % nums.len();
    }

    // println!("{}", &min_idx);

    let mut cur = min_idx;
    let max_idx = ((min_idx - 1) + nums.len()) % nums.len();
    while cur != max_idx {
        let next_cur = (cur + 1) % nums.len();
        if nums[cur] > nums[next_cur] {
            return None;
        }
        cur = next_cur;
    }

    Some((nums[min_idx], nums[max_idx]))
}
