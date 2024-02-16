fn good_subarrays(a: Vec<i32>) -> u32 {
    let mut prev: Vec<u32> = vec![1; a.len()];
    let mut accum: u32 = a.len() as u32;

    for len in 2..(a.len() + 1) {
        let mut curr: Vec<u32> = vec![0; a.len()];
        for index in (len-1)..a.len() {
            if a[index] % ((len) as i32) == 0 {
                curr[index] += prev[0..index].iter().sum::<u32>(); //tecnicamente O(n^3) matenme plz
            }
        }
        accum += curr.iter().sum::<u32>();
        prev = curr;
    }
    accum
 
}

fn main() {
    let res = good_subarrays(vec![2, 2, 1, 22, 15]);
    println!("number of good subarrays: {}", res);
}
