#![warn(missing_docs)]
//! This is a demo file for using concurrency and multithreaded
//! calculation in rust
//!
//! 
// import rust functions from library
//
use rust_concurrency_playground_lib;
// these crates are for timing
extern crate stopwatch;
use stopwatch::Stopwatch;

// these are for parallel
//
use std::thread;

// this is to move data between
// threads
use std::sync::mpsc;



/// The main function here just runs the comparison code
pub fn main() {
    println!("Hello, world! \n");
    println!("We are summing to one thousand for first test");

    // serial demonstration
    let mut stopwatch_serial = Stopwatch::start_new();
    let serial_sum: f64;
    serial_sum = serial_sum_to_one_thousand();

    println!("serial sum = {:?}", serial_sum);
    stopwatch_serial.stop();

    println!("serial calc took {:?} \n \n \n",
             stopwatch_serial.elapsed());

    //parallel demonstration
    // it will be slow as the time cost taken to parallelise
    // the threads.

    let mut stopwatch_parallel = Stopwatch::start_new();
    let parallel_sum: f64;
    parallel_sum = parallel_sum_to_one_thousand();

    println!("parallel sum = {:?}", parallel_sum);
    stopwatch_parallel.stop();

    println!("parallel calc took {:?} \n \n \n",
             stopwatch_parallel.elapsed());

    // serial demonstration sum to 1 billion
    let mut stopwatch_serial_1b = Stopwatch::start_new();
    let serial_sum: f64;
    serial_sum = serial_sum_to_one_billion();

    println!("serial sum = {:?}", serial_sum);
    stopwatch_serial_1b.stop();

    println!("sum to one billion serial calc took {:?} \n \n \n",
             stopwatch_serial_1b.elapsed());

    // parallel demonstration sum to 1 billion
    let mut stopwatch_parallel_1b = Stopwatch::start_new();
    let parallel_sum: f64;
    parallel_sum = rust_concurrency_playground_lib::
        parallel_sum_to_one_billion();

    println!("parallel sum = {:?}", parallel_sum);
    stopwatch_parallel_1b.stop();

    println!("sum to one billion parallel calc took {:?} \n \n \n",
             stopwatch_parallel_1b.elapsed());

}

pub fn serial_sum_to_one_thousand() -> f64{
    let mut sum: f64 = 0.0;
    for i in 1..1001 {
        sum = sum + i as f64;
    }

    return sum;
}

pub fn parallel_sum_to_one_thousand() -> f64 {
    let sum: f64;


    // lets sum to 500 in one thread
    // and sum 500 to 1000 in the other thread

    // we will need to use a safe way to move
    // data between threads, ie message passing
    //
    let (tx, rx) = mpsc::channel();

    let _handle = thread::spawn(move || {
        let mut thread_two_inner_sum = 0.0;

        for i in 501..1001 {
            thread_two_inner_sum =
                thread_two_inner_sum + i as f64;
        }
        tx.send(thread_two_inner_sum).unwrap();

    });

    let mut thread_one_sum: f64 = 0.0;


    for i in 1..501 {
        thread_one_sum = thread_one_sum + i as f64;
    }


    let thread_two_sum = rx.recv().unwrap();

    sum = thread_one_sum + thread_two_sum;

    return sum;
}

/// In this function, I sum approx one billion
/// integers using a for loop   
///
/// For this function, I sum it using one thread
/// only; it is a serial calculation
// to see where parallel computation can shine
// we might want to sum to a bigger number, eg
// 1 billion
pub fn serial_sum_to_one_billion() -> f64 {

    let mut sum: f64 = 0.0;
    for i in 1..1000000001 {
        sum = sum + i as f64;
    }

    return sum;
}

