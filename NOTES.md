Two Pointers

Concept: Use two indices to traverse a data structure in a coordinated way, typically from both ends or as fast/slow pointers. Ideal for sorted arrays, linked list cycles, or removing duplicates.

```Rust
fn two_sum_sorted(arr: &[i32], target: i32) -> Option<(usize, usize)> {
    let (mut left, mut right) = (0, arr.len().wrapping_sub(1));
    while left < right {
        let sum = arr[left] + arr[right];
        if sum == target {
            return Some((left, right));
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    None
}
```

Sliding Window

Concept: Maintain a subset (window) of the array that moves as you iterate. Use it to optimize subarray or substring problems.
```Rust
fn min_subarray_len(target: i32, nums: &[i32]) -> usize {
    let mut min_len = usize::MAX;
    let (mut current_sum, mut left) = (0, 0);
    for right in 0..nums.len() {
        current_sum += nums[right];
        while current_sum >= target {
            min_len = min_len.min(right - left + 1);
            current_sum -= nums[left];
            left += 1;
        }
    }
    if min_len == usize::MAX { 0 } else { min_len }
}
```
Binary Search

Concept: Halve the search space to find a target in sorted data. Time complexity is O(log n).
```Rust
fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
    let (mut lo, mut hi) = (0, nums.len().wrapping_sub(1));
    while lo <= hi {
        let mid = (lo + hi) / 2;
        if nums[mid] == target {
            return Some(mid);
        } else if nums[mid] < target {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    None
}
```
Breadth-First Search (BFS)

Concept: Traverse level-by-level using a queue. Great for shortest path problems and level order traversals.
```Rust
use std::collections::VecDeque;
fn shortest_path_in_grid(grid: &[Vec<u8>]) -> Option<usize> {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut visited = vec![vec![false; cols]; rows];
    let mut queue = VecDeque::new();
    if grid[0][0] == 1 { return None; }
    visited[0][0] = true;
    queue.push_back((0, 0, 0));
    let directions = [(1,0), (-1,0), (0,1), (0,-1)];
    while let Some((r, c, dist)) = queue.pop_front() {
        if r == rows - 1 && c == cols - 1 {
            return Some(dist);
        }
        for (dr, dc) in directions {
            let (nr, nc) = (r as isize + dr, c as isize + dc);
            if nr >= 0 && nc >= 0 && nr < rows as isize && nc < cols as isize {
                let (nr, nc) = (nr as usize, nc as usize);
                if !visited[nr][nc] && grid[nr][nc] == 0 {
                    visited[nr][nc] = true;
                    queue.push_back((nr, nc, dist + 1));
                }
            }
        }
    }
    None
}
```
Depth-First Search (DFS)

Concept: Explore as deep as possible before backtracking. Use recursion or a stack. Works well for trees and islands.
```Rust
fn count_islands(grid: &mut Vec<Vec<u8>>) -> usize {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut count = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 1 {
                count += 1;
                dfs_fill(grid, i, j);
            }
        }
    }
    count
}

fn dfs_fill(grid: &mut Vec<Vec<u8>>, r: usize, c: usize) {
    if r >= grid.len() || c >= grid[0].len() || grid[r][c] != 1 {
        return;
    }
    grid[r][c] = 0;
    dfs_fill(grid, r+1, c);
    dfs_fill(grid, r.wrapping_sub(1), c);
    dfs_fill(grid, r, c+1);
    dfs_fill(grid, r, c.wrapping_sub(1));
}
```
Backtracking

Concept: Try all possible options recursively and backtrack when needed. Used for permutations, combinations, and puzzles.
```Rust
fn permutations(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();
    let mut result = Vec::new();
    let mut current = Vec::new();
    let mut used = vec![false; n];
    fn backtrack(nums: &Vec<i32>, current: &mut Vec<i32>,
                 used: &mut Vec<bool>, result: &mut Vec<Vec<i32>>) {
        if current.len() == nums.len() {
            result.push(current.clone());
            return;
        }
        for i in 0..nums.len() {
            if used[i] { continue; }
            used[i] = true;
            current.push(nums[i]);
            backtrack(nums, current, used, result);
            current.pop();
            used[i] = false;
        }
    }
    backtrack(&nums, &mut current, &mut used, &mut result);
    result
}
```
Merge Intervals

Concept: Sort intervals and merge overlapping ones.
```Rust
fn merge_intervals(mut intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    if intervals.is_empty() { return intervals; }
    intervals.sort_by_key(|interval| interval.0);
    let mut merged = Vec::with_capacity(intervals.len());
    merged.push(intervals[0]);
    for &(start, end) in &intervals[1..] {
        let last_end = merged.last().unwrap().1;
        if start <= last_end {
            let last = merged.last_mut().unwrap();
            last.1 = last_end.max(end);
        } else {
            merged.push((start, end));
        }
    }
    merged
}
```
Greedy

Concept: Choose the best local option hoping it leads to global optimum. Great for interval selection, coin change (with canonical coins), etc.
```Rust
fn select_activities(mut intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    intervals.sort_by_key(|activity| activity.1);
    let mut selected = Vec::new();
    let mut last_finish = i32::MIN;
    for &(start, finish) in &intervals {
        if start >= last_finish {
            selected.push((start, finish));
            last_finish = finish;
        }
    }
    selected
}
```
Dynamic Programming

Concept: Store results of subproblems to avoid recomputation. Ideal for optimization and counting problems.
```Rust
fn climb_stairs(n: u32) -> u32 {
    if n <= 1 { return 1; }
    let mut dp = vec![0; (n+1) as usize];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..=n as usize {
        dp[i] = dp[i-1] + dp[i-2];
    }
    dp[n as usize]
}
```
Union Find (Disjoint Set)

Concept: Track connected components. Useful in graphs and networks for detecting cycles and grouping.
```Rust
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    fn union(&mut self, a: usize, b: usize) {
        let root_a = self.find(a);
        let root_b = self.find(b);
        if root_a != root_b {
            if self.rank[root_a] < self.rank[root_b] {
                self.parent[root_a] = root_b;
            } else if self.rank[root_a] > self.rank[root_b] {
                self.parent[root_b] = root_a;
            } else {
                self.parent[root_b] = root_a;
                self.rank[root_a] += 1;
            }
        }
    }
}
```
Monotonic Stack

Concept: Stack that keeps elements in increasing or decreasing order. Common in "next greater" or "histogram" problems.
```Rust
fn next_greater_elements(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![-1; n];
    let mut stack: Vec<usize> = Vec::new();
    for i in 0..n {
        while let Some(&last_idx) = stack.last() {
            if nums[i] <= nums[last_idx] {
                break;
            }
            result[last_idx] = nums[i];
            stack.pop();
        }
        stack.push(i);
    }
    result
}
```
Heaps (Priority Queue)

Concept: Efficient access to the max or min element. Great for top-k problems or scheduling.
```Rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;
fn kth_largest(nums: &[i32], k: usize) -> i32 {
    let mut min_heap = BinaryHeap::new();
    for &num in nums {
        min_heap.push(Reverse(num));
        if min_heap.len() > k {
            min_heap.pop();
        }
    }
    min_heap.peek().unwrap().0
}
```

