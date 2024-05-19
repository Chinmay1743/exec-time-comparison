# Execution Time Comparison in Rust

Nothing fancy. Just for fun.
<br>
The program logic only does "Add" operation at its core ion a loop to perform something meaningful and display the time taken to do so.
<br>
You can change the hardcoded `limit_value` variable in code to change loop limit and `core_count` variable to change the number of threads to be spawned at runtime.
<br>
Primary purpose for such a simple code is to see how much time will it take for same core logic to complete execution in other programming languages (that's why many limits are in billions).
<br>

## Vague time metrics but they give you an Idea:

|Hardware       (OS: Windows)                |Limit value|Cores|Time Taken|
|:------------------------------------------:|:-------------------------------:|:---:|:---------:|
|CPU: Ryzen 5 5500U, <br> RAM: 16x2GB 3200MHz DDR4|1,000,000,000,000 or 1000 billion|12   |172 seconds|
|                                            |10,000,000,000 or 10 billion     |12   |~15 seconds|
|                                            |1,000,000,000 or 1 billion       |12   | ~<1 Second|

<br>
For faster results, please run build and run your program in `--release` mode. If you compile and run in `--debug`, the time taken will be way too long.
<br>

### Sample Output (for the value: 10,000,000,000):
<br>

Debug mode:
```
Rust result total: 49999999965000000006
Start Time: Instant {
    t: 2928.233354s,
} seconds
Time taken to execute: Instant {
    t: 2949.702924s,
} seconds
```

<br>

Release Mode:
```
Rust result total: 49999999965000000006
Start Time: Instant {
    t: 2902.4152618s,
} seconds
Time taken to execute: Instant {
    t: 2904.2918899s,
} seconds
```

<br>

#### You can send your own values in PR or can provide suggestions (if any)

<br>

***

## Pre-requisites: Need to install Rust toolchain into your system:
Visit: https://www.rust-lang.org/tools/install
<br>
<br>
Or you can run this on your system:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

<br>

***

<br>

## Installation:
```
git clone https://github.com/Chinmay1743/exec-time-comparison.git
```
<br>

```
cd exec-time-comparison
```
<br>

```
cargo build --release
```
<br>

```
cargo run --release
```

<br>


***


### Hardware Configuration for reference:

```
CPU: Ryzen 5 5500U
RAM: 16x2GB 3200MHz DDR4
```

