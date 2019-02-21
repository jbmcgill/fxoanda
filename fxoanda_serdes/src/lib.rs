
pub mod serfloats;
pub use self::serfloats::*;
pub mod serdates;
pub use self::serdates::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
