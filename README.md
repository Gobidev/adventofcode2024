# Advent of Code 2024

## Benchmarks

I am not always optimizing for speed, but here are some benchmarks anyway.

_Measured with `hyperfine -N --warmup 20 <binary>`. Results include both parts.
Input is included in the binary during compilation, but parsed during runtime._

CPU: Ryzen 5 3600 (6c/12t)

| Day     |   Mean [ms] | Min [ms] | Max [ms] |
| :------ | ----------: | -------: | -------: |
| `day1`  |   0.6 ± 0.0 |      0.6 |      1.6 |
| `day2`  |   1.2 ± 0.0 |      1.2 |      1.5 |
| `day3`  |   1.5 ± 0.0 |      1.5 |      1.8 |
| `day4`  |   0.8 ± 0.0 |      0.8 |      1.1 |
| `day5`  |   1.2 ± 0.0 |      1.1 |      2.0 |
| `day6`  |  59.0 ± 1.3 |     57.6 |     63.4 |
| `day8`  |   0.7 ± 0.0 |      0.7 |      0.9 |
| `day9`  | 474.1 ± 0.6 |    473.1 |    475.4 |
| `day10` |   1.6 ± 0.0 |      1.6 |      1.9 |
| `day11` |  13.9 ± 0.4 |     13.4 |     15.1 |
| `day12` |  53.6 ± 0.4 |     52.6 |     54.8 |
| `day13` |   1.2 ± 0.0 |      1.2 |      1.5 |
| `day14` |  55.0 ± 0.3 |     54.8 |     57.0 |
| `day15` |   8.0 ± 0.2 |      7.9 |      8.7 |
| `day16` |  62.7 ± 0.1 |     62.4 |     63.1 |
| `day17` |   0.7 ± 0.0 |      0.7 |      1.9 |
| `day18` | 106.4 ± 0.3 |    106.0 |    107.7 |
| `day19` |  51.9 ± 0.4 |     51.6 |     53.7 |
| `day20` |   5.4 ± 0.2 |      5.0 |      6.4 |
| `day21` |   1.3 ± 0.0 |      1.2 |      1.5 |
| `day22` | 302.3 ± 0.8 |    301.3 |    304.1 |
