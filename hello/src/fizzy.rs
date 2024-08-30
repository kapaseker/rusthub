// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

use std::fmt::Display;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T>(fn(T) -> bool, String);

impl<T> Matcher<T>
{
    pub fn new(_matcher: fn(T) -> bool, _subs: &str) -> Matcher<T> {
        Matcher(_matcher, _subs.to_string())
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T: Display + Copy>(Vec<Matcher<T>>);

impl<T: Display + Copy> Fizzy<T> {
    pub fn new() -> Self {
        Self(vec![])
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(self, _matcher: Matcher<T>) -> Self {
        let mut fizzy = Self(self.0);
        fizzy.0.push(_matcher);
        fizzy
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I: Iterator<Item=T>>(self, _iter: I) -> impl Iterator<Item=String> {
        _iter.map(move |i| {
            let mut str = String::new();

            self.0.iter().for_each(|s| {
                if s.0(i) {
                    str.push_str(s.1.as_str())
                }
            });

            if str.is_empty() {
                str.push_str(&(i.to_string()));
            }

            str
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T: Copy + Display>() -> Fizzy<T> {
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n.to_string().parse::<f64>().unwrap() % 3.0f64 == 0f64, "fizz"))
        .add_matcher(Matcher::new(|n: T| n.to_string().parse::<f64>().unwrap() % 5.0f64 == 0f64, "buzz"))
}


struct A();

impl Into<i32> for A {
    fn into(self) -> i32 {
        todo!()
    }
}

impl From<i32> for A {
    fn from(value: i32) -> Self {
        todo!()
    }
}

impl PartialEq for A {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }

    fn ne(&self, other: &Self) -> bool {
        todo!()
    }
}

mod test {
    use crate::fizzy::{fizz_buzz, Fizzy, Matcher};

    #[test]

    fn simple() {
        let actual = fizz_buzz::<i32>().apply(1..=16).collect::<Vec<_>>();


        let expected = [
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13", "14",
            "fizzbuzz", "16",
        ];


        assert_eq!(actual, expected);
    }


    #[test]

    fn u8() {
        let actual = fizz_buzz::<u8>().apply(1_u8..=16).collect::<Vec<_>>();


        let expected = [
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13", "14",
            "fizzbuzz", "16",
        ];


        assert_eq!(actual, expected);
    }


    #[test]

    fn u64() {
        let actual = fizz_buzz::<u64>().apply(1_u64..=16).collect::<Vec<_>>();


        let expected = [
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13", "14",
            "fizzbuzz", "16",
        ];


        assert_eq!(actual, expected);
    }


    #[test]

    fn nonsequential() {
        let collatz_12 = &[12, 6, 3, 10, 5, 16, 8, 4, 2, 1];


        let actual = fizz_buzz::<i32>()


            .apply(collatz_12.iter().cloned())


            .collect::<Vec<_>>();


        let expected = vec![
            "fizz", "fizz", "fizz", "buzz", "buzz", "16", "8", "4", "2", "1",
        ];


        assert_eq!(actual, expected);
    }


    #[test]

    fn custom() {
        let expected = vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "Bam", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "Bam", "BuzzFizz", "16",
        ];


        let fizzer: Fizzy<i32> = Fizzy::new()


            .add_matcher(Matcher::new(|n: i32| n % 5 == 0, "Buzz"))


            .add_matcher(Matcher::new(|n: i32| n % 3 == 0, "Fizz"))


            .add_matcher(Matcher::new(|n: i32| n % 7 == 0, "Bam"));


        let actual = fizzer.apply(1..=16).collect::<Vec<_>>();


        assert_eq!(actual, expected);
    }


    #[test]

    fn f64() {


        // a tiny bit more complicated becuase range isn't natively implemented on floats

        let actual = fizz_buzz::<f64>()


            .apply(std::iter::successors(Some(1.0), |prev| Some(prev + 1.0)))


            .take(16)


            .collect::<Vec<_>>();


        let expected = [
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13", "14",
            "fizzbuzz", "16",
        ];


        assert_eq!(actual, expected);
    }


    #[test]

    fn minimal_generic_bounds() {
        use std::fmt;

        use std::ops::{Add, Rem};

        #[derive(Clone, Copy, Debug, Default, PartialEq)]

        struct Fizzable(u8);

        impl From<u8> for Fizzable {
            fn from(i: u8) -> Fizzable {
                Fizzable(i)
            }
        }

        impl fmt::Display for Fizzable {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let Fizzable(ref n) = self;


                write!(f, "{n}")
            }
        }

        impl Add for Fizzable {
            type Output = Fizzable;


            fn add(self, rhs: Fizzable) -> Fizzable {
                let Fizzable(n1) = self;


                let Fizzable(n2) = rhs;


                Fizzable(n1 + n2)
            }
        }

        impl Rem for Fizzable {
            type Output = Fizzable;


            fn rem(self, rhs: Fizzable) -> Fizzable {
                let Fizzable(n1) = self;


                let Fizzable(n2) = rhs;


                Fizzable(n1 % n2)
            }
        }

        let actual = fizz_buzz::<Fizzable>()


            .apply(std::iter::successors(Some(Fizzable(1)), |prev| {
                Some(*prev + 1.into())
            }))


            .take(16)


            .collect::<Vec<_>>();

        let expected = [
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13", "14",
            "fizzbuzz", "16",
        ];

        assert_eq!(actual, expected);
    }
}
