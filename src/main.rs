use std::time::Instant;
use std::thread::JoinHandle;
use std::thread;

fn add(somevalue: i128, num_threads: usize) -> i128 {
    let mut handles: Vec<JoinHandle<i128>> = vec![];
    let chunk_size: i128 = somevalue / num_threads as i128;
    let mut result: i128 = 0;

    for _ in 0..num_threads {
        let start: i128 = result + 1;
        let end: i128 = result + chunk_size;
        let handle = thread::spawn(move || {
            let mut sum: i128 = 0;
            for i in start..=end {
                sum += i;
            }
            sum
        });
        handles.push(handle);
        result = end;
    }

    let mut final_sum: i128 = 0;
    for handle in handles {
        final_sum += handle.join().unwrap();
    }

    final_sum
}

fn main() { 
    // let limit_value: i128 = 1000000000000; // 1,000,000,000,000 or 1000 billion. 172 sec on all 12 cores
    let limit_value: i128 = 10000000000; // 10,000,000,000 or 10 billion. roughly 15-16 seconds on all 12 cores
    // let limit_value: i128 = 1000000000; // 1,000,000,000 or 1 billion. less than a sec on all 12 cores
    let core_count: usize = 12; // Set the number of cores you want to use
    let start_time = Instant::now();
    let final_sum: i128 = add(limit_value, core_count);
    let end_time = Instant::now();
    println!("Rust result total: {}", final_sum);
    println!("Start Time: {:#?} seconds", start_time); //.elapsed().as_secs_f64()
    println!("Time taken to execute: {:#?} seconds", end_time); //.elapsed().as_secs_f64()
}



// Below program works well but is single-threaded. Above is modified version of below
/* 
use std::time::Instant;

fn adddd(somevalue: i128) -> i128 {
    let mut result: i128 = 0;
    for i in 1..=somevalue {
        result += i;
    }
    result
}

fn main() {
    let limit_value: i128 = 100000000000; // 100,000,000,000 or 100 billion
    let start_time = Instant::now();
    let final_sum: i128 = adddd(limit_value);
    let end_time = Instant::now();
    println!("Rust result total: {}", final_sum);
    println!("Start Time: {:#?} seconds", start_time);
    println!("Time taken to execute: {:#?} seconds", end_time);
}
*/





