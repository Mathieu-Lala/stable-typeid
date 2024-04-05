use stable_typeid::{StableAny, StableAnyTrait as _, StableID};

#[derive(Debug, StableID)]
struct MyStruct {
    anything: String,
}

#[derive(Debug, StableID)]
struct MyUnamedUnionStruct((Vec<u8>, bool));

fn foo<T: StableID + std::fmt::Debug>(any: &dyn StableAny) {
    if let Some(my_struct) = any.downcast_ref::<T>() {
        println!("{:?} => {}", my_struct, T::_STABLE_ID);
    }
}

#[test]
fn _01() {
    let any = MyStruct {
        anything: "Hello".to_string(),
    };
    foo::<MyStruct>(&any);
}

#[test]
fn _02() {
    let any = MyUnamedUnionStruct((vec![0, 1, 2, 3], false));
    foo::<MyUnamedUnionStruct>(&any);
}
