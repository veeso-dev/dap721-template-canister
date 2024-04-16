use integration_tests::actor::{admin, alice, bob};
use integration_tests::client::Dip721Client;
use integration_tests::TestEnv;

#[test]
#[serial_test::serial]
fn test_should_mint() {
    let env = TestEnv::init();
    let client = Dip721Client::new(&env);

    assert!(client.mint(admin(), alice(), 1_u64.into(), vec![]).is_ok());

    let token = client.token_metadata(1u64.into()).unwrap();
    assert_eq!(token.owner, Some(alice()));
}

#[test]
#[serial_test::serial]
fn test_should_transfer() {
    let env = TestEnv::init();
    let client = Dip721Client::new(&env);

    assert!(client.mint(admin(), admin(), 1_u64.into(), vec![]).is_ok());

    assert!(client.transfer(admin(), alice(), 1_u64.into()).is_ok());
    let token = client.token_metadata(1u64.into()).unwrap();
    assert_eq!(token.owner, Some(alice()));
}

#[test]
#[serial_test::serial]
fn test_should_not_allow_transfer() {
    let env = TestEnv::init();
    let client = Dip721Client::new(&env);

    assert!(client.mint(admin(), admin(), 1_u64.into(), vec![]).is_ok());

    assert!(client.transfer(alice(), alice(), 1_u64.into()).is_err());
}

#[test]
#[serial_test::serial]
fn test_should_approve_transfer_from() {
    let env = TestEnv::init();
    let client = Dip721Client::new(&env);

    assert!(client.mint(admin(), admin(), 1_u64.into(), vec![]).is_ok());

    assert!(client.approve(admin(), bob(), 1_u64.into()).is_ok());

    let token = client.token_metadata(1u64.into()).unwrap();
    assert_eq!(token.operator, Some(bob()));

    assert!(client
        .transfer_from(bob(), admin(), alice(), 1_u64.into())
        .is_ok());
    let token = client.token_metadata(1u64.into()).unwrap();
    assert_eq!(token.owner, Some(alice()));
}

#[test]
#[serial_test::serial]
#[should_panic]
fn test_should_not_approve_transfer_from() {
    let env = TestEnv::init();
    let client = Dip721Client::new(&env);

    assert!(client.mint(admin(), admin(), 1_u64.into(), vec![]).is_ok());

    assert!(client.approve(bob(), bob(), 1_u64.into()).is_err());
}

#[test]
#[serial_test::serial]
fn test_should_burn() {
    let env = TestEnv::init();
    let client = Dip721Client::new(&env);

    assert!(client.mint(admin(), admin(), 1_u64.into(), vec![]).is_ok());

    assert!(client.burn(admin(), 1_u64.into()).is_ok());

    let token = client.token_metadata(1u64.into()).unwrap();

    assert!(token.is_burned);
}
