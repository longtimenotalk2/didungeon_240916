fn main() {
    println!("Hello, world!");

    trait MessageA {
        fn show(&self) -> &'static str {"aaa"}
    }

    struct SA {

    }

    impl MessageA for SA {}

    fn show(a : impl MessageA) -> &'static str {
        a.show()
    }

    let a = SA {};
    println!("{}", show(a));
}
