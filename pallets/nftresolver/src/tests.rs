use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_creates_a_project() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_project(Origin::signed(100), 101));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::projects(101), 1);
		assert_ok!(TemplateModule::create_project(Origin::signed(100), 102));
		assert_eq!(TemplateModule::projects(102), 2);
	});
}

#[test]
fn add_pub_key() {
	new_test_ext().execute_with(|| {
        let b = vec![4, 5, 6];
		assert_ok!(TemplateModule::create_project(Origin::signed(100), 101));
		assert_ok!(TemplateModule::set_project_key(Origin::signed(101), 1, b.clone()));
        assert_noop!(TemplateModule::set_project_key(Origin::signed(103), 1, b.clone()), Error::<Test>::NotAvailableProjectId);
        assert_noop!(TemplateModule::set_project_key(Origin::signed(101), 2, b.clone()), Error::<Test>::NotTheOwner);
    });
}
