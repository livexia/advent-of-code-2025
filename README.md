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
