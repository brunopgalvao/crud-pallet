use crate::{mock::*, *};
use frame_support::assert_ok;

#[test]
fn test_something1_with_getter() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Crud::do_something1(RuntimeOrigin::signed(1), 42));
		assert_eq!(Crud::something1(), Some(42));
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}

#[test]
fn test_something2_without_getter() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Crud::do_something2(RuntimeOrigin::signed(1), 42));
		assert_eq!(Something2::<Test>::get(), Some(42));
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}

#[test]
fn test_something3_with_value_query() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_eq!(Something3::<Test>::get(), 3);
		assert_ok!(Crud::do_something3(RuntimeOrigin::signed(1), 42));
		assert_eq!(Something3::<Test>::get(), 42);
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}