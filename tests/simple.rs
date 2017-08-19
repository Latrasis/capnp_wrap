#[macro_use]
extern crate capnp_wrap;
#[macro_use]
extern crate capnp;

pub mod simple_capnp {
  include!(concat!(env!("OUT_DIR"), "/tests/simple_capnp.rs"));
}

trait Schema {
    fn noop() {}
}

#[derive(Schema)]
#[capnp(schema = "/tests/simple_capnp.rs")]
#[capnp(alias = "Person")]
struct Person {
    #[capnp(alias = "firstname")]
    first_name: String,
    middlename: String,
    lastname: String,
}

#[test]
fn should_impl_hello_world() {
    Person::noop();
}