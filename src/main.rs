use lipsum::lipsum_words_with_rng;
use rand::thread_rng;
use std::thread;
use std::time;

const SLEEP_TIME: u64 = 3;
const NUM_WORDS: usize = 200;
const NUM_LOOPS: u64 = 4;

fn main() {
    let duration = time::Duration::from_secs(SLEEP_TIME);
    for i in 0..NUM_LOOPS {
        thread::sleep(duration);
        println!("@loop {i}");
        println!("{}", lipsum_words_with_rng(thread_rng(), NUM_WORDS));
    }
}
