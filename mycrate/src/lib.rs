use std::ffi::c_int;

#[allow(dead_code)]
extern "C" {
    fn open_coroutine_init() -> c_int;
}

#[cfg(test)]
mod tests {
    use crate::open_coroutine_init;

    #[test]
    fn test() {
        assert_eq!(0, unsafe { open_coroutine_init() });
        println!("Hello, world!");
    }

    #[cfg(unix)]
    #[test]
    fn test_hook() {
        use std::time::{Duration, Instant};

        let start = Instant::now();
        std::thread::sleep(Duration::MAX);
        let cost = Instant::now().duration_since(start);
        println!("cost: {:?}", cost);
    }
}
