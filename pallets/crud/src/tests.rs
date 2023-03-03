use crate::{mock::*, *};
use frame_support::assert_ok;

#[test]
fn test_number_with_getter() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Crud::set_number(RuntimeOrigin::signed(1), 42));
		assert_eq!(Crud::number(), Some(42));
		System::assert_last_event(Event::NumberStored { number: 42, who: 1 }.into());
	});
}

#[test]
fn test_number_without_getter() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Crud::set_number(RuntimeOrigin::signed(1), 42));
		assert_eq!(Number::<Test>::get(), Some(42));
		System::assert_last_event(Event::NumberStored { number: 42, who: 1 }.into());
	});
}

#[test]
fn test_number_with_default_with_value_query() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_eq!(NumberWithDefault::<Test>::get(), 3);
		assert_ok!(Crud::set_number_with_default(RuntimeOrigin::signed(1), 42));
		assert_eq!(NumberWithDefault::<Test>::get(), 42);
		System::assert_last_event(Event::NumberWithDefaultStored { number: 42, who: 1 }.into());
	});
}

#[test]
fn test_number_with_result_query() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_eq!(NumberResultQuery::<Test>::get().unwrap(), Ok::<u32, Error<Test>>(0).unwrap());
		assert_ok!(Crud::set_number_result_query(RuntimeOrigin::signed(1), 42));
		assert_eq!(NumberResultQuery::<Test>::get().unwrap(), Ok::<u32, Error<Test>>(42).unwrap());
		System::assert_last_event(Event::NumberResultQueryStored { number: 42, who: 1 }.into());
	});
}

#[test]
fn test_number_with_option_query() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_eq!(NumberOptionQuery::<Test>::get(), None);
		assert_ok!(Crud::set_number_option_query(RuntimeOrigin::signed(1), 42));
		assert_eq!(NumberOptionQuery::<Test>::get(), Some(42));
		System::assert_last_event(Event::NumberOptionQueryStored { number: 42, who: 1 }.into());
	});
}

#[test]
fn test_set_account_data() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_eq!(AccountData::<Test>::get(), None);
		assert_ok!(Crud::set_account_data(RuntimeOrigin::signed(1), 1));
		assert_eq!(AccountData::<Test>::get(), Some(1));
		System::assert_last_event(Event::AccountDataStored { account: 1, who: 1 }.into());
	});
}

#[test]
fn test_something7_with_struct() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Crud::set_client_data(RuntimeOrigin::signed(1), 2, b"Alex".to_vec()));
		assert_eq!(ClientData::<Test>::get().unwrap().id, 2);
		assert_eq!(ClientData::<Test>::get().unwrap().name, b"Alex".to_vec());
		System::assert_last_event(Event::ClientDataStored { client: 2, who: 1 }.into());
	});
}

#[test]
fn test_set_counted_map() {
    new_test_ext().execute_with(|| {
        assert_eq!(CountedMap::<Test>::count(), 0);
		assert_ok!(Crud::set_counted_map(RuntimeOrigin::signed(1), 42));
		assert_eq!(CountedMap::<Test>::count(), 1);
        assert_eq!(CountedMap::<Test>::get(0), Some(42));
    })
}