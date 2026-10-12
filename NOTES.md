# PerfReport (Rust) — IS2083 Lab 3

Rust migration of the Falcon Health Systems C++ PerfReport tool. The C++ version used the Windows API (`GetSystemInfo`, `GlobalMemoryStatusEx`, `GetTickCount64`, `GetDiskFreeSpaceExA`), so it only ran on Windows. This version uses the `sysinfo` crate and runs on Windows, Linux, macOS and in a Docker container from one code base.

## What it reports
Logical processors, memory total/free/percent in use, uptime in minutes, first disk total/free/percent used, and an efficiency score (0 to 100, GOOD/FAIR/POOR) using the same rule as the C++ version.

## How to run
    cargo run            # report
    cargo test           # scoring tests
    docker build -t perfreport .
    docker run --rm perfreport

## CI/CD
`.github/workflows/ci.yml` checks formatting, runs clippy, builds, and runs tests. The Docker build job only runs after that job passes (`needs: build-and-test`).

## Differences from the C++ version
- Memory percent is computed from used/total bytes; the C++ version read Windows' own `dwMemoryLoad`, so the numbers can differ by a point or two.
- Disk reads the first disk sysinfo lists instead of hard-coding `C:\`.

## AI help
I used an AI assistant to help fill in the sysinfo readings, write the scoring tests, and write this notes file. I reviewed and tested every line.
