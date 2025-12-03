# Advent of Code 2025

- https://adventofcode.com/
- https://github.com/livexia/advent-of-code-2025

## Day 1

今天的问题算是轻松，输入的每一行决定旋钮的转向和旋转的距离，输入的处理也不复杂，但是要注意在解析数字时的错误处理，输入处理完成后，如果旋钮是向左旋转则距离为负数，向右则为正数。

第一部分只需要计算旋转之后的刻度位置是否为 0 即可，可以简单的求 100 的余数即可。

第二部分稍微复杂一些，需要计算每次旋转过程中经过了多少次 0 刻度位置，给出的测试用例较小，实际的输入旋转的距离可能是多圈的，要考虑到这一点。首先计算每一次旋转的距离至少有多少圈，每转一圈密码加一。接着计算整圈之外的旋转距离，如果起始点不在 0 ，那么旋转结束后，如果刻度跨过 0 刻度，密码加一。因为向左旋转距离是负数，如果当前位置不为 0 ，同时当前位置加上旋转距离为负数，则旋钮指针一定向左跨过一次 0 。同理如果当前位置不为 0 （不为 100），同时向右旋转，而旋转后的刻度数超过 100 ，那么指针一定向右旋转过一次 0。根据这个逻辑对 password 进行加一即可。

第二部分代码

```rust
fn part2(rotations: &[i32]) -> Result<u32> {
    let _start = Instant::now();

    let mut dial = 50;
    let mut password = 0;

    for rot in rotations {
        // 计算一定会经过 0 刻度的整圈次数
        password += rot.unsigned_abs() / 100;

        // 扣除整圈的旋转距离，
        // 可以规避例如从 0 刻度旋转距离 100 的边界情况
        let rot = rot % 100;

        let temp = dial + rot;
        // 当起点不是 0 刻度时，
        // 向左或向右旋转超过或位于刻度边界 0 或 100 时，
        // 一定经过一次 0 刻度
        password += (dial != 0 && (temp >= 100 || temp <= 0)) as u32;

        // 计算旋转后刻度的真实位置，刻度值一定大于 0
        dial = temp.rem_euclid(100);
    }

    println!("part2: {password}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(password)
}
```

## Day 2

今天要求找出区间内存在重复模式的数字，第一部分要求找出左右两个部分相同的数字，第二部分要求找出数字经过 N 等分后，每个部分都相同的数字。第二部分其实是第一部分的衍生，解题思路在两个部分是一致的。我首先利用暴力法得出题解，通过取余的方法不断的分割数字，对比分割的结果，即可确定是否存在重复模式。暴力法效率不高，因为我遍历了区间内的所有数字，依次检查数字，这样实际效率很慢。

### 渐进寻找

暴力法中区间内的数字是依次递增的，但是应该有更加高效的方法确定符合重复模式的数字。考虑区间 565653-565659 ，可见这个区间中所有的数字都是 6 位等长的，我们先考虑第一部分的重复模式，即数字的前半部分和后半部分应当相同。考虑区间起点 565653 ，直接将区间按照重复模式进行分割，可得到两个数字 565 和 653 ，同样的将结尾也进行分割得到 565 和 659 ，可见两个数字的前半部分相同，那么这个部分就不能变动，那么 653 和 659 就应该变化为 565 ，得到的数字是 565565 不在区间内。所以按照第一部分的匹配模式，无法从区间内寻找到符合的数字。那么考虑第二部分的匹配模式，这个区间的数字长度都为 6 ，那么存在长度为 1、2 和 3 的三种分割模式。确定区间起点和结尾数字中，共同的部分为 56565，那么根据这个共同部分进行分割，可以发现长度 1 或 3 的分割模式是不可能的。考虑分割长度为 2 ，那么分割后每个部分都需要是 56 才行，同时 56 刚好落入起点和结尾 53 - 59 之间，那么长度为 2 的分割可行。

这个方法存在一种情况，那就是如果区间的数字长度不一致，比如区间 95-115 就不容易寻找了，当然可以把区间进行拆分，95-115 变成 95-99 和 100-115 两个区间，分开寻找即可。

**效率对比**

```
part1: 26255179562
> Time elapsed is: 22.953333ms
part2: 31680313976
> Time elapsed is: 37.9495ms
part1 by step: 26255179562
> Time elapsed is: 58.208µs
part2 by step: 31680313976
> Time elapsed is: 70.834µs
```

渐进查找主要代码
```rust
fn split_range(start: usize, end: usize) -> Vec<(usize, usize)> {
    let (start_l, end_l) = (start.ilog10(), end.ilog10());
    if start_l < end_l {
        let mut ranges = vec![];
        let mut start = start;
        for i in start_l..=end_l {
            let new_end = 10usize.pow(i + 1) - 1;
            ranges.push((start, new_end.min(end)));
            start = new_end + 1;
        }
        ranges
    } else {
        vec![(start, end)]
    }
}

fn find_invalid(start: usize, end: usize, base: u32) -> Vec<usize> {
    assert_eq!(start.ilog10(), end.ilog10());
    let l = start.ilog10() + 1;
    if !l.is_multiple_of(base) {
        return vec![];
    }
    let (start_left, end_left) = (start / 10usize.pow(l - base), end / 10usize.pow(l - base));
    let mut invalids = Vec::new();
    for s in start_left..=end_left {
        let n = (0..l)
            .step_by(base as usize)
            .fold(0, |n, i| n + s * 10usize.pow(i));
        if start <= n && n <= end {
            invalids.push(n);
        }
    }
    invalids
}
```

## Day 3

今天要求计算一个数字序列的最大子串（数字），第一部分限定子串长度为2，第二部分则限定长度为12，同时子串的顺序不变。输入的处理不复杂，思路也很简单，就是遍历数字序列，依次寻找最大值即可。

考虑数字序列 96781 ，需要寻找长度为 2 的最大子串，假设子串为 ab ，那么优先从给定序列中确定最大的 a ，同时确保能找到 b 即可。搜索 a 时从左到右进行搜索，而搜索到最后一个数字之前就需要停止，也就是搜索 9678 即可。在搜索 a 的过程中不必考虑是否可能会导致 b 的值不是最大，因为最后的要求的是 ab 最大即可。假设搜索过程中为了确保 b 的值为最大的 d ，而导致 a 取得了较小的值 c，即 c < a 且 d > b ，那么 10 * c + d 一定小于 10 * a + b 。

两个部分对于子串的长度要求不同，但是思路是一致的。从左到右依次搜索字串最大值的过程中，需要记录第一次遇到最大值的元素位置，而非其他可能遇到最大值的位置，这样可以避免影响后续元素最大值搜索。参考序列 98975，需要寻找长度为 2 的最大子串，a 确定搜索到的最大值为 9，如果 a 记录的最大值位置不为 0 而为 2 ，那么搜索 b 时就会从位置 3 开始搜索，最后得到子串为 97 ，是错误结果。

核心代码

```rust
fn find_largest_joltage(battery: &[usize], number: usize) -> usize {
    let length = battery.len();
    let mut joltage = 0;
    let mut next_battery = 0;
    for l in (0..number).rev() {
        let mut max_battery = 0;
        (next_battery..(length - l)).for_each(|left| {
            if battery[left] > max_battery {
                max_battery = battery[left];
                next_battery = left + 1;
            }
        });
        joltage = joltage * 10 + max_battery;
    }
    joltage
}
```
