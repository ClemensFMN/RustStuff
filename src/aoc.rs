fn aoc_1() {
    let lst = vec![1,1,2,1,2,9,9];
    let mut cnt = 0;
    let mut sum = 0;
    for i in 0..lst.len()-1 {
        println!("i: {}, lst[i]: {}, lst[i+1]:{}", i, lst[i], lst[i+1]);
        if lst[i] == lst[i+1] {
            cnt += 1;
            sum += lst[i];
        }
    }
    if lst[lst.len()-1] == lst[0] {
        cnt += 1;
        sum += lst[0];
    }
    println!("Count: {}, Sum: {}", cnt, sum)
}

fn aoc_2() {
    let s = "5 1 9 5 \n 7 5 3 \n 2 4 6 8";
    let mut chk_sum = 0;
    for ln in s.lines() {
        //println!("line: {}", ln);
        let mut ln_min = u32::max_value();
        let mut ln_max = u32::min_value();
        for num in ln.split_whitespace() {
            //println!("num: {}, parsed: {}", num, num.parse::<u32>().unwrap());
            let num_prsd = num.parse::<u32>().unwrap();
            if num_prsd > ln_max {
                ln_max = num_prsd;
            }
            if num_prsd < ln_min {
                ln_min = num_prsd;
            }
        }
        //println!("min: {}, max: {}", ln_min, ln_max);
        chk_sum += ln_max - ln_min;
    }
    println!("Checksum: {}", chk_sum);
}


pub fn run_me() {
    aoc_1();
    aoc_2();
}
