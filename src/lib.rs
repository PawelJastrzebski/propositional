#![allow(non_snake_case)]
#![doc = include_str!("../README.md")]

pub mod connectives;
pub mod core;
mod utils;

pub mod prelude {
    pub use crate::connectives::*;
    pub use crate::core::*;
}


#[cfg(test)]
pub mod examples {
    use super::prelude::*;

    #[test]
    fn rainy_day() {
        let rain = symbol!("it's raining");
        let cloud = symbol!("it's cloudy");

        let world = and!(
            implies!(rain, cloud),
            rain
        );

        println!("It is cloudy? {:?}", check(&world, &cloud));
        assert_eq!(Some(true), check(&world, &cloud));
    }

    #[test]
    fn harry_visit() {
        let rain = symbol!("It is raining.");
        let hagrid = symbol!("Harry visited Hagrid.");
        let dumbledore = symbol!("Harry visited Dumbledore.");

        let knowledge = and!(
            implies!(not!(rain), hagrid),
            or!(hagrid, dumbledore),
            not!(and!(hagrid, dumbledore)),
            dumbledore
        );

        println!("{}", knowledge);

        println!("It is raining? {:?}", check(&knowledge, &rain));
        assert_eq!(Some(true), check(&knowledge, &rain));
    }

}