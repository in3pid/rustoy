use fx::Iter;

mod fx {
    pub trait Iter<T>  {
        fn next(&mut self) -> Option<T>;

        fn map<'a, B> (self, f: |T|:'a -> B) -> Map<'a, T, B, Self> {
            Map{it: self, f: f}
        }

        fn take(self, n: int) -> Take<T, Self> {
            Take{n: n, it: self}
        }

        fn fold<'a, A>(&mut self, init: A, f: |A, T|:'a -> A) -> A {
            let mut init = init;
            loop {
                match self.next() {
                    None => break,
                    Some(x) => init = f(init, x),
                }
            }
            return init;
        }
    }



    #[deriving(Clone)]
    pub struct Range<T> { start: T, stop: T }

    impl<T: PartialOrd + Add<T, T> + Clone + ::std::num::One> Iter<T> for Range<T> {
        fn next(&mut self) -> Option<T> {
            if self.stop <= self.start {
                return None;
            }
            let x = self.start.clone();
            self.start = self.start + ::std::num::One::one();
            return Some(x)
        }
    }

    pub fn range<T>(start: T, stop: T) -> Range<T> { Range{start: start, stop: stop} }


    // begränsa längden av en iterator

    struct Take<A, T: Iter<A>> { n: int, it: T }

    impl<A, T: Iter<A>> Iter<A> for Take<A, T> {
        fn next(&mut self) -> Option<A> {
            if self.n <= 0 {
                return None;
            }
            self.n -= 1;
            return (self.it).next();
        }
    }


    // generera x, f(x), f(f(x)), f(f(f(x)), ...

    pub struct Iterate<'a, T> { x: T, f: |T|:'a -> T }

    impl<'a, T: Clone> Iter<T> for Iterate<'a, T> {
        fn next(&mut self) -> Option<T> {
            let x = self.x.clone();
            self.x = (self.f)(self.x.clone());
            return Some(x);
        }
    }

    pub fn iterate<'a, T>(x: T, f: |T|:'a -> T) -> Iterate<'a, T> {
        Iterate{x: x, f: f}
    }    

    //

    struct Map<'a, X, Y, T> { it: T, f: |X|:'a -> Y }

    impl<'a, X, Y, T: Iter<X>> Iter<Y> for Map<'a, X, Y, T> {
        fn next(&mut self) -> Option<Y> {
            match (self.it).next() {
                None => None,
                Some(x) => Some((self.f)(x))
            }
        }
    } 

    pub fn print<T: ::std::fmt::Show>(it: &mut Iter<T>) {
        loop {
            match it.next() {
                Some(x) => println!("{}", x),
                None => break,
            }
        }
    }



}


fn main() {
    let mut t = fx::iterate(0u, |x| { x + 1})
        .map(|x| { x + 1})
        .take(10)
        .fold(0, |x, y| { x + y });
    println!("{}", t);
}
