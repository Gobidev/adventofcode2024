# Advent of Code 2024

## Benchmarks

I am not always optimizing for speed, but here are some benchmarks anyway.

_Measured with `hyperfine -N --warmup 20 <binary>`. Results include both parts.
Input is included in the binary during compilation, but parsed during runtime._

CPU: Ryzen 5 3600 (6c/12t)

| Day     |   Mean [ms] | Min [ms] | Max [ms] |
| :------ | ----------: | -------: | -------: |
| `day1`  |   0.7 ± 0.0 |      0.6 |      1.0 |
| `day2`  |   1.3 ± 0.0 |      1.2 |      1.6 |
| `day3`  |   1.5 ± 0.0 |      1.5 |      1.9 |
| `day4`  |   0.9 ± 0.0 |      0.8 |      1.1 |
| `day5`  |   1.2 ± 0.0 |      1.2 |      1.9 |
| `day6`  |  59.0 ± 0.4 |     58.4 |     60.4 |
| `day8`  |   0.7 ± 0.0 |      0.7 |      1.0 |
| `day9`  | 475.3 ± 0.9 |    474.2 |    476.4 |
| `day10` |   1.7 ± 0.0 |      1.6 |      2.5 |
| `day11` |  14.2 ± 0.4 |     13.6 |     15.3 |
| `day12` |  54.0 ± 0.4 |     53.0 |     55.3 |
| `day13` |   1.3 ± 0.0 |      1.2 |      1.7 |
| `day14` |  55.3 ± 0.3 |     54.9 |     56.3 |
| `day15` |   8.1 ± 0.1 |      7.9 |      8.7 |
| `day16` |  63.1 ± 0.3 |     62.6 |     63.8 |

