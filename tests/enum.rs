use stable_typeid::{StableAny, StableAnyTrait as _, StableID};

#[derive(Debug, StableID)]
enum MyEnum {
    Foo,
    Bar,
}

fn foo(any: &dyn StableAny) {
    if let Some(my_enum) = any.downcast_ref::<MyEnum>() {
        println!("{:?} => {}", my_enum, MyEnum::_STABLE_ID);
    }
}

#[test]
fn _01() {
    let any = MyEnum::Bar;
    foo(&any);
}
