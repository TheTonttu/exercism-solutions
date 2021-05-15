use robot_name as robot;
//use serial_test::serial;

fn assert_name_matches_pattern(n: &str) {
    assert!(n.len() == 5, "name is exactly 5 characters long");
    assert!(
        n[0..2].chars().all(|c| ('A'..='Z').contains(&c)),
        "name starts with 2 uppercase letters"
    );
    assert!(
        n[2..].chars().all(|c| ('0'..='9').contains(&c)),
        "name ends with 3 numbers"
    );
}

fn assert_name_is_persistent(r: &robot::Robot) {
    // The type system already proves this, but why not.
    let n1 = r.name();
    let n2 = r.name();
    let n3 = r.name();
    assert_eq!(n1, n2);
    assert_eq!(n2, n3);
}

#[test]
fn test_name_should_match_expected_pattern() {
    let r = robot::Robot::new();
    assert_name_matches_pattern(r.name());
}

#[test]
fn test_name_is_persistent() {
    assert_name_is_persistent(&robot::Robot::new());
}

#[test]
fn test_different_robots_have_different_names() {
    let r1 = robot::Robot::new();
    let r2 = robot::Robot::new();
    assert_ne!(r1.name(), r2.name(), "Robot names should be different");
}

#[test]
fn test_new_name_should_match_expected_pattern() {
    let mut r = robot::Robot::new();
    assert_name_matches_pattern(r.name());
    r.reset_name();
    assert_name_matches_pattern(r.name());
}

#[test]
fn test_new_name_is_persistent() {
    let mut r = robot::Robot::new();
    r.reset_name();
    assert_name_is_persistent(&r);
}

#[test]
fn test_new_name_is_different_from_old_name() {
    let mut r = robot::Robot::new();
    let n1 = r.name().to_string();
    r.reset_name();
    let n2 = r.name().to_string();
    assert_ne!(n1, n2, "Robot name should change when reset");
}

// Additional tests

#[test]
fn test_unique_names() {
    use std::collections::HashSet;

    const ROBOT_COUNT: usize = 1000;

    let mut robots = Vec::with_capacity(ROBOT_COUNT);
    let mut names = HashSet::with_capacity(ROBOT_COUNT);

    for i in 0..ROBOT_COUNT {
        let r = robot::Robot::new();
        let name = r.name().to_string();
        robots.push(r);
        assert!(
            names.insert(name.clone()),
            "robot {} name '{}' was not unique",
            i,
            name
        );
    }
}

// Does not work because #[serial] doesn't seem to guarantee that non-serial test cases have finished
// #[test]
// #[serial]
// fn test_name_release() {
//     println!("init");
//     assert_registry_size(0);
//
//     {
//         println!("1");
//         let _r1 = robot::Robot::new();
//         assert_registry_size(1);
//
//         println!("2");
//         let _r2 = robot::Robot::new();
//         assert_registry_size(2);
//
//         println!("3");
//         let _r3 = robot::Robot::new();
//         assert_registry_size(3);
//     }
//
//     println!("finish");
//     assert_registry_size(0);
// }
//
// fn assert_registry_size(expected_size: usize) {
//     let registry = robot::NAME_REGISTRY.lock().unwrap();
//     let registry_size = registry.len();
//     drop(registry);
//     assert_eq!(registry_size, expected_size);
// }
