extern crate algorithm;
pub use algorithm::search::top_three::*;
pub use algorithm::search::first_step::*;

fn main() {
    sort_first_step()
}

fn sort_first_step() {
    search_three_times();
    rate_too_slow();
    rate_fast_o_one();
    rate2();
    insert_sort()
}
