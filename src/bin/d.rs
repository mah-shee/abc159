#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut list: Vec<Vec<usize>> = vec![vec![0; 2]; n + 1];
    for i in 0..n {
        list[a[i]][0] += 1;
    }
    let mut sum = 0;
    for i in 0..n {
        if list[a[i]][0] > 1 && list[a[i]][1] == 0 {
            sum += list[a[i]][0] * (list[a[i]][0] - 1) / 2;
            list[a[i]][1] = 1;
        }
    }
    for i in 0..n {
        if list[a[i]][0] <= 1 {
            println!("{}", sum);
        } else {
            let tmp = sum - (list[a[i]][0] * (list[a[i]][0] - 1) / 2)
                + ((list[a[i]][0] - 1) * (list[a[i]][0] - 2) / 2);
            println!("{}", tmp);
        }
    }
}
