use crate::{mock::*, *};
use frame_support::assert_ok;

#[test]
fn test_something1() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Crud::do_something1(RuntimeOrigin::signed(1), 42));
		assert_eq!(Crud::something1(), Some(42));
		assert_eq!(Something1::<Test>::get(), Some(42));
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}
