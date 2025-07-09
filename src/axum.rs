// Axum style function params
// Utilises type system to try and give a function overloading type thing

/// Our initial context for this example
#[derive(Clone)]
pub struct Person {
    name: String,
    age: u8,
}

/// Trait that extracts values from the Person struct
pub trait FromPerson {
    fn from_person(person: Person) -> Self;
}

pub struct Name(pub String);
pub struct Age(pub u8);

impl FromPerson for Name {
    fn from_person(person: Person) -> Self {
        Name(person.name.clone())
    }
}

impl FromPerson for Age {
    fn from_person(person: Person) -> Self {
        Age(person.age)
    }
}

pub trait Handler<T> {
    fn call(self, person: Person);
}

impl<F, T> Handler<T> for F
where
    F: Fn(T),
    T: FromPerson,
{
    fn call(self, person: Person) {
        (self)(T::from_person(person))
    }
}


impl<F, T1, T2> Handler<(T1, T2)> for F
where
    F: Fn(T1, T2),
    T1: FromPerson,
    T2: FromPerson,
{
    fn call(self, person: Person) {
        (self)(T1::from_person(person.clone()), T2::from_person(person.clone()))
    }
}
