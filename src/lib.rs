#![feature(negative_impls)]
#![feature(auto_traits)]
// TODO: #![deny(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::trait_duplication_in_bounds)]

mod access;
mod generics_helpers;
mod get;
mod into_values;
mod list_based;
mod property;
mod select;
pub use access::Access;
pub use get::Get;
pub use property::{Property, P};

/// Everything here is completely safe and can't panic at runtime
#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use crate::{Access, Get, Property, P};

    struct Name;
    impl Property for Name {
        type Type = String;
    }

    fn shout_name<T: Get<Name>>(person: &T) -> String {
        person.get::<Name>().to_uppercase()
    }

    #[test]
    fn simple_example() {
        let mut john = (P::<Name>("John".into()),);
        john.get_mut::<Name>().push_str("son");
        assert_eq!(shout_name(&john), "JOHNSON");
    }

    struct Age;
    impl Property for Age {
        type Type = u8;
    }
    // Fathers have to be people (in this case, have Name and Age)
    struct Father<T: Person>(PhantomData<T>);
    impl<T: Person> Property for Father<T> {
        type Type = T;
    }

    impl<T: Get<Name> + Get<Age>> Person for T {}
    trait Person: Get<Name> + Get<Age> {
        fn say_hello(&self) -> String {
            let name = self.get::<Name>();
            let age = self.get::<Age>();
            format!("Hi! my name is {name} and I'm {age} years old.")
        }

        fn say_hello_with_father<T: Person>(&self) -> String
        where
            Self: Get<Father<T>>,
        {
            format!(
                "{}\n{}",
                self.say_hello(),
                self.get::<Father<_>>().say_hello()
            )
        }
    }

    #[test]
    fn nested_trait_example() {
        let mike = (
            P::<Father<_>>((P::<Name>("Nate".into()), P::<Age>(35))),
            P::<Age>(3),
            P::<Name>("Mike".into()),
        );
        assert_eq!(
            mike.say_hello_with_father(),
            "Hi! my name is Mike and I'm 3 years old.\n\
            Hi! my name is Nate and I'm 35 years old."
        );
        assert_eq!(
            mike.select::<(Name, Age)>().into_values(),
            ("Mike".into(), 3)
        );
    }
}
