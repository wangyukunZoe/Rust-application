use integration_test;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, integration_test::add_two(2));
}

/*
集成测试形成的哈希码

     Running tests/integration_test.rs (target/debug/deps/integration_test-fde4485138e5f601)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests integration_test

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
*/
