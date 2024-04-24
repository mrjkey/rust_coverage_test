fn main() {
    println!("Hello, world!");
    let a = 2;
    let b = 3;
    let c = add(a, b);
    println!("{} + {} = {}", a, b, c);
}

// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 {
    if a < 0 {
        return 0;
    }
    return a + b;
}

pub fn sub(a: i32, b: i32) -> i32 {
    return a - b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        // assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_main() {
        main();
    }
}
