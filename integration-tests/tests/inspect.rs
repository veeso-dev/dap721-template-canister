use candid::Encode;
use integration_tests::actor::{admin, alice};
use integration_tests::TestEnv;

#[test]
#[serial_test::serial]
fn test_should_inspect_is_custodian() {
    let env = TestEnv::init();

    assert!(env
        .update::<()>(
            env.dip721_id,
            admin(),
            "set_name",
            Encode!(&"test").unwrap()
        )
        .is_ok());

    assert!(env
        .update::<()>(
            env.dip721_id,
            alice(),
            "set_name",
            Encode!(&"test").unwrap()
        )
        .is_err());
}
