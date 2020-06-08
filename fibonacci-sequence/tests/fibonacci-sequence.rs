use fibonacci_sequence::fibonacci;

#[test]
fn fibonacci_zero() {
    assert_eq!(fibonacci(0), 0);
}

#[test]
fn fibonacci_one() {
    assert_eq!(fibonacci(1), 1);
}

#[test]
fn fibonacci_two() {
    assert_eq!(fibonacci(2), 1);
}

#[test]
fn fibonacci_three() {
    assert_eq!(fibonacci(3), 2);
}

#[test]
fn fibonacci_four() {
    assert_eq!(fibonacci(4), 3);
}

#[test]
fn fibonacci_five() {
    assert_eq!(fibonacci(5), 5);
}

#[test]
fn fibonacci_six() {
    assert_eq!(fibonacci(6), 8);
}

#[test]
fn fibonacci_ten() {
    assert_eq!(fibonacci(10), 55);
}

#[test]
fn fibonacci_twenty() {
    assert_eq!(fibonacci(20), 6765);
}

#[test]
fn fibonacci_forty() {
    assert_eq!(fibonacci(40), 102_334_155);
}

#[test]
fn fibonacci_fifty() {
    assert_eq!(fibonacci(50), 12586269025);
}
