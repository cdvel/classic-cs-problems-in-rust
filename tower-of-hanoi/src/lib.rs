pub fn hanoi(origin: &mut Vec<i32>, target: &mut std::vec::Vec<i32>, temp: &mut Vec<i32>, n: i32) {

    if n == 1{
        target.push(origin.pop().unwrap());
    }else{
        hanoi(origin, temp, target, n - 1);
        hanoi(origin, target, temp, 1);
        hanoi(temp, target, origin, n - 1);
    }
}

pub fn hanoi_target(n: i32) -> std::vec::Vec<i32>{
    
    let (mut a, mut b, mut c) = init(n);
    hanoi(&mut a, &mut b, &mut c, n);
    b.to_vec()
}

pub fn init(n: i32) -> (Vec<i32>, Vec<i32>, Vec<i32>){
    let mut a: Vec<i32> = Vec::new();
    let b: Vec<i32> = Vec::new();
    let c: Vec<i32> = Vec::new();
    
    for i in 1..n+1{
        a.push(i);
    }

    (a, b, c)

}