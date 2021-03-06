use sai::{Component, Injected};
use std::any::TypeId;

/*
 * Some testing component
 */
#[derive(Component)]
struct Foo {
    #[injected]
    a: Injected<Bar>,

    b: String
}

#[derive(Component)]
struct Bar {
    value: String
}


#[test]
fn test_build() {

    let mut repo = sai::ComponentRepository::new();
    let bar: Box<dyn Component> = (Bar::meta().build)(&repo);
    let injected_bar: Injected<dyn Component> = bar.into();

    assert_eq!(Bar::meta().type_id, TypeId::of::<Injected<Bar>>());

    repo.insert_with_typeid(
        Bar::meta().type_id,
        injected_bar
    );

    let foo = Foo::build(&repo);

    assert_eq!(foo.a.value, "");
    assert_eq!(foo.b, "");
}

#[test]
fn test_meta() {

    let meta = Foo::meta();

    assert_eq!(meta.type_id, TypeId::of::<Injected<Foo>>());
    assert_eq!(meta.depends_on, vec![TypeId::of::<Injected<Bar>>()]);
}
