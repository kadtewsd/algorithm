// num を使うには、toml に依存関係を書かないといけない。
extern crate num;
extern crate std;

pub fn rate_too_slow() {
    let n = 6;
    let rate = [5, 3, 1, 3, 4, 3];

    let mut result = num::pow(-10, 9);
    for j in 1..n {
        for i in 0..(j - 1) {
            result = if result < (rate[j] - rate[i]) {
                (rate[j] - rate[i])
            } else {
                result
            };
        }
    }
    println!("rate's answer,,, max value is {}", result);
}

pub fn rate_fast_o_one() {
    let n = 6;
    let rate = [5, 3, 1, 3, 4, 3];
    let mut minimum = rate[0];
    let mut maximum = num::pow(-10, 9);

    for i in 1..n {
        maximum = std::cmp::max(rate[i] - minimum, maximum);
        minimum = std::cmp::min(rate[i], minimum);
    }
    println!("improved max value is {}", maximum);
}

pub fn rate2() {
    let rate = [5, 3, 1, 3, 4, 3];
    for x in rate.iter() {
        println!("iter result is ... {}", x);
    }
}

pub fn insert_sort() {
    let n = 8;
    let mut rate = [199, 39, 5, 3, 1, 3, 4, 3];

    let mut current;
//    let mut j: i32 = 0;
    let mut j;
    for x in 1..n {
        // index のアクセスは usize でないといけない。i32 だと、 slice indices are of type `usize` とでる。
        current = rate[x];
        j = (x - 1) as i32;
//        j = x - 1;
//        while j >= 0 && current < rate[j] {
//            rate[j + 1] = rate[j];
        while j >= 0 && current < rate[j as usize] {
            rate[(j + 1) as usize] = rate[j as usize];
            j -= 1;
            println!("x {} j {}", x, j);
        }
        rate[(j + 1) as usize] = current;
    }
    println!("{:#?}", rate);
}
