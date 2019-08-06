// vim: tw=80
//! A struct with a method that returns an immutable reference

use mockall::*;

mock! {
    Foo {
        fn foo(&self) -> &u32;
        fn bar(&self) -> &u32;
    }
}

mod never {
    use super::*;

    #[test]
    #[should_panic(expected = "Expectation should not have been called")]
    fn fail() {
        let mut mock = MockFoo::new();
        mock.expect_bar()
            .return_const(0)
            .never();
        mock.bar();
    }

    #[test]
    fn ok() {
        let mut mock = MockFoo::new();
        mock.expect_foo()
            .never();
    }
}

#[test]
fn return_const() {
    let mut mock = MockFoo::new();
    mock.expect_foo()
        .return_const(5u32);
    assert_eq!(5, *mock.foo());
}

mod sequence {
    use super::*;

    #[test]
    #[should_panic(expected = "exact call count")]
    fn ambiguous() {
        let mut seq = Sequence::new();
        let mut mock = MockFoo::new();
        mock.expect_foo()
            .times(1..3)
            .in_sequence(&mut seq);
        mock.foo();
    }

    #[test]
    #[should_panic(expected = "Method sequence violation")]
    fn fail() {
        let mut seq = Sequence::new();
        let mut mock = MockFoo::new();
        mock.expect_bar()
            .times(1)
            .return_const(0)
            .in_sequence(&mut seq);

        mock.expect_foo()
            .times(1)
            .return_const(0)
            .in_sequence(&mut seq);

        mock.foo();
        mock.bar();
    }

    #[test]
    fn ok() {
        let mut seq = Sequence::new();
        let mut mock = MockFoo::new();
        mock.expect_foo()
            .times(1)
            .return_const(0)
            .in_sequence(&mut seq);

        mock.expect_bar()
            .times(1)
            .return_const(0)
            .in_sequence(&mut seq);

        mock.foo();
        mock.bar();
    }
}