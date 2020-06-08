use tower_of_hanoi::hanoi_target;

#[test]
fn assert() {
    assert!(true);    
}

#[test]
fn one_disc() {
    assert_eq!(hanoi_target(1), [1])
}

#[test]
fn three_discs() {
    assert_eq!(hanoi_target(3), [1,2,3])
}

#[test]
fn four_discs() {
    assert_eq!(hanoi_target(4), [1,2,3,4])
}
#[test]
fn five_discs() {
    assert_eq!(hanoi_target(5), [1,2,3,4,5])
}
#[test]
fn six_discs() {
    assert_eq!(hanoi_target(6), [1,2,3,4,5,6])
}
#[test]
fn ten_discs() {
    assert_eq!(hanoi_target(10), [1,2,3,4,5,6,7,8,9,10])
}

#[test]
fn twenty_discs() {
    assert_eq!(hanoi_target(20), [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20])
}
