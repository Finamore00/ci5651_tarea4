fn good_subarrays(a: Vec<i32>) -> u32 {
    let mut prev: Vec<u32> = vec![1; a.len()];
    let mut res: u32 = a.len() as u32;

    for len in 2..(a.len() + 1) {
        let mut curr: Vec<u32> = vec![0; a.len()];
        let mut accum: u32 = 0;
        for index in (len-1)..a.len() {
            accum += prev[index-1];
            if a[index] % (len as i32) == 0 {
                curr[index] += accum;
                res += curr[index];
            }
        }
        prev = curr;
    }
    res
 
}

fn main() {
    let res = good_subarrays(vec![2, 2, 1, 22, 15]);
    println!("number of good subarrays: {}", res);
}
