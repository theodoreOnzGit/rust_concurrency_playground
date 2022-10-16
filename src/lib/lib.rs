/// sum approx one billion
/// integers using a for loop   
///
/// For this function, I sum it using two threads
/// only; it is a parallel calculation
///
/// ```rust
/// let parallel_sum = rust_concurrency_playground_lib::
/// parallel_sum_to_one_billion();
///
/// println!("parallel sum = {:?}", parallel_sum);
///
/// ```
///
pub fn parallel_sum_to_one_billion() -> f64 {
    // these are for parallel
    //
    use std::thread;

    // this is to move data between
    // threads
    use std::sync::mpsc;
    let sum: f64;


    // lets sum to 500 million in one thread
    // and sum 500 million to 1billion in the other thread

    // we will need to use a safe way to move
    // data between threads, ie message passing
    //
    let (tx, rx) = mpsc::channel();

    let _handle = thread::spawn(move || {
        let mut thread_two_inner_sum = 0.0;

        for i in 500000001..1000000001 {
            thread_two_inner_sum =
                thread_two_inner_sum + i as f64;
        }
        tx.send(thread_two_inner_sum).unwrap();

    });

    let mut thread_one_sum: f64 = 0.0;


    for i in 1..500000001 {
        thread_one_sum = thread_one_sum + i as f64;
    }


    let thread_two_sum = rx.recv().unwrap();

    sum = thread_one_sum + thread_two_sum;

    return sum;
}
