// TODO: #![deny(missing_docs)]
#![warn(clippy::pedantic)]
#![feature(type_changing_struct_update)]

mod access;
mod merge;
#[macro_use]
mod partial;
#[macro_use]
mod auto_impl;

#[cfg(test)]
mod tests {
    #![allow(dead_code)]

    use crate::{access::Access, merge::Merge};

    partial! {person {
        name: String,
        age: u8,
        height: f32,
    }}

    // Any person that has at least a name
    auto_impl! {trait Named for person (name) {
        fn shout_name(&self) -> String {
            // this borrows self as person::Struct, so that the fields can be accessed
            let borrowed = self.borrow();
            let uppercase = borrowed.name.to_uppercase();
            match borrowed.age.get() {
                Some(age) => format!("{uppercase} ({age})"),
                None => uppercase,
            }
        }
    }}

    // Any person that has at least a name and age
    auto_impl! {trait SayHello for person (name, age) {
        fn say_hello(&self) -> String {
            let borrowed = self.borrow();
            format!(
                "Hi! I'm {} and I'm {} years old",
                borrowed.name, borrowed.age
            )
        }
    }}

    #[test]
    fn methods_example() {
        // we don't know John's age yet
        let john = person::Struct {
            name: "John".to_string(),
            ..person::empty()
        };

        // we can call shout_name, but not say_hello
        assert_eq!(john.shout_name(), "JOHN");

        let john = john.merge(person::Struct {
            age: 26,
            ..person::empty()
        });

        // now that we know John's age, we can call say_hello
        assert_eq!(john.say_hello(), "Hi! I'm John and I'm 26 years old");

        // the behaviour of shout_name changes since john has an age
        assert_eq!(john.shout_name(), "JOHN (26)");
    }

    #[test]
    fn test_merge() {
        let josef = person::Struct {
            name: "Josef".to_string(),
            ..person::empty()
        };

        let josef = josef.merge(person::Struct {
            age: 37,
            ..person::empty()
        });
        assert_eq!(josef.name, "Josef");
        assert_eq!(josef.age, 37);
        let josefine = josef.merge(person::Struct {
            name: "Josefine".to_string(),
            ..person::empty()
        });
        assert_eq!(josefine.name, "Josefine");
        assert_eq!(josefine.age, 37);
    }

    // This will be generated by a future version of the partial! macro
    mod with {
        #![allow(non_camel_case_types)]
        #![allow(type_alias_bounds)]

        pub type age<T: super::person::Interface> = super::person::Struct<T::name, u8, T::height>;
    }

    fn with_age<T: person::Interface>(person: T, age: u8) -> with::age<T> {
        person.merge(person::Struct {
            age,
            ..person::empty()
        })
    }
}
