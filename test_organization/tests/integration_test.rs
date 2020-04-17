use test_organization; //We’ve added use test_organization at the top of
// the code, which we didn’t need in the unit tests. 
//The reason is that each file in the tests directory 
//is a separate crate, so we need to bring our 
//library into each test crate’s scope.

//We don’t need to annotate any code in tests/integration_test.rs 
//with #[cfg(test)]. 
//Cargo treats the tests directory specially and compiles 
//files in this directory only when we run cargo test.

#[test]
fn it_adds_two() {
    assert_eq!(4, test_organization::add_two(2));
}

/*
$  cargo test --test integration_test
runs all the tests in a particular integration test file
*/

