#![allow(dead_code)]
#![recursion_limit="500"]
use std::marker::PhantomData;

struct Zero{}
struct Succ<A> {
    phantom: PhantomData<A>
}

trait Number {
    fn repr() -> i32;
}

impl Number for Zero {
    fn repr() -> i32 {
        0
    }
}

impl<A> Number for Succ<A> where A: Number {
    fn repr() -> i32 {
        1 + A::repr()
    }
}

trait Incr {
    type Out: Number;
}


impl Incr for Zero {
    type Out = Succ<Zero>;
}


impl<A> Incr for Succ<A> where A: Number {
    type Out = Succ<Succ<A>>;
}

trait Decr {
    type Out: Number;
}

impl Decr for Zero {
    type Out = Zero;
}

impl<A> Decr for Succ<A> where A: Number {
    type Out = A;
}

trait Add<RHS> {
    type Out: Number;
}

//f(x, 0) = x
impl<RHS> Add<RHS> for Zero where RHS: Number {
    type Out = RHS;
}

// f(x, n + 1) -> f(x, n) + 1
impl<LHS, RHS> Add<RHS> for Succ<LHS> where RHS: Add<LHS> {
    type Out = Succ<<RHS as Add<LHS>>::Out>;
}

// Subtraction only on natural numbers. Negative numbers are mapped to Zero.
trait Sub<RHS> {
    type Out: Number;
}

impl<LHS> Sub<LHS> for Zero where LHS: Number {
    type Out = LHS;
}

impl<A, LHS> Sub<LHS> for Succ<A> where A: Sub<<LHS as Decr>::Out>, LHS: Decr {
    type Out =  <A as Sub<<LHS as Decr>::Out>>::Out;
}

trait Mul<RHS> {
    type Out: Number;
}

// f(x, 0) = 0
impl<RHS> Mul<RHS> for Zero {
    type Out = Zero;
}

// f(x, y + 1) = f(x, y) + x
impl <A, RHS> Mul<RHS> for Succ<A> where A: Mul<RHS>, A::Out: Add<RHS> {
    type Out = <<A as Mul<RHS>>::Out as Add<RHS>>::Out;
}

trait Div<RHS> {
    type Out: Number;
}

impl<RHS> Div<RHS> for Zero {
    type Out = Zero;
}

impl<A, RHS> Div<RHS> for Succ<A> where RHS: Sub<Succ<A>>,  <RHS as Sub<Succ<A>>>::Out: Div<RHS> {
    type Out = Succ<<<RHS as Sub<Succ<A>>>::Out as Div<RHS>>::Out>;
}

trait Pow<A> {
    type Out: Number;
}

// f(x, 0) = 1
impl<A> Pow<A> for Zero {
    type Out =  Succ<Zero>;
}

// f(x, y + 1) = x*f(x, y)
impl<A, E> Pow<A> for Succ<E> where E: Pow<A>, E::Out: Mul<A> {
    type Out = <<E as Pow<A>>::Out as Mul<A>>::Out;
}

trait If<A, B> {
    type Out;
}

impl<A, B> If<A, B> for Zero {
    type Out = B;
}

impl<A, B> If<A, B> for Succ<Zero> {
    type Out = A;
}


//Check if type Self and A are equal: if they are return Succ<Zero>, if not unification fails -> compiler error
trait Equal<A> {
    type Out;
}

impl Equal<Zero> for Zero {
    type Out = Succ<Zero>;
}

impl<A, B> Equal<Succ<B>> for Succ<A> where B: Equal<A> {
    type Out = <B as Equal<A>>::Out;
}

trait Max<B> {
    type Out: Number;
}

//Recursion start
impl<A, B> Max<Succ<B>> for Succ<A> where A: Max<B> {
    type Out = Succ<<A as Max<B>>::Out>;
}

//Recursion end: Type Self < B (trait parameter) are Equal
impl<A> Max<Succ<A>> for Zero where A: Number {
    type Out = Succ<A>;
}
//Recursion end: Type Self > B (trait parameter) are Equal
impl<A> Max<Zero> for Succ<A> where A: Number {
    type Out = Succ<A>;
}

//Recursion end: Type Self and B are Equal
impl Max<Zero> for Zero {
    type Out = Zero;
}

// When calling the function the last generic Type (B) must be set to '_' in order for the compiler to infer the result type of the operation.
// If we set the B generic type and it is not the result type of the operation, the rust unification process will fail and the type checker will error out.
fn incr<A, B>() -> i32 where A: Incr<Out=B>, B: Number {
    B::repr()
}

fn decr<A, B>() -> i32 where A: Decr<Out=B>, B: Number {
    B::repr()
}

fn add<LHS, RHS, B>() -> i32 where LHS: Add<RHS, Out=B>, B: Number {
    B::repr()
}

fn sub<LHS, RHS, Result>() -> i32 where RHS: Sub<LHS, Out=Result>, Result: Number {
    Result::repr()
}

fn mul<LHS, RHS, Result>() -> i32 where LHS: Mul<RHS, Out=Result>, Result: Number {
    Result::repr()
}

fn div<LHS, RHS, Result>() -> i32 where LHS: Div<RHS, Out=Result>, Result: Number {
    Result::repr()
}

fn pow<A, E, Result>() -> i32 where E: Pow<A, Out=Result>, Result: Number {
    Result::repr()
}

fn conditional_mul<A, B, C, Result>() where A: If<<B as Mul<C>>::Out, Zero, Out=Result>, B: Mul<C>, Result: Number {
    println!("{}", Result::repr());
}

fn conditional_generic<A, B, C, Result>() -> i32 where A: If<B, C, Out=Result>, Result: Number {
    Result::repr()
}

fn equal<A, B, C>() where A: Equal<B, Out=C>, C: Number {
    println!("{}", C::repr());
}

fn max<A, B, C>() -> i32 where A: Max<B, Out=C>, C: Number {
    C::repr()
}

type False = Zero;
type True = Succ<Zero>;

type I1 = Succ<Zero>;
type I2 = Succ<Succ<Zero>>;
type I3 = Succ<Succ<Succ<Zero>>>;
type I4 = Succ<Succ<Succ<Succ<Zero>>>>;
type I5 = Succ<Succ<Succ<Succ<Succ<Zero>>>>>;
type P5<N> = Succ<Succ<Succ<Succ<Succ<N>>>>>;
type P10<N> = P5<P5<N>>;
type P50<N> = P10<P10<P10<P10<P10<N>>>>>;

// Red Black Tree Type Checking START -------------------------------------------
struct BNode<V, L, R> {
    value: V,
    left: PhantomData<L>,
    right: PhantomData<R>,
}
struct RNode<V, L, R> {
    value: V,
    left: PhantomData<L>,
    right: PhantomData<R>,
}
struct Leaf {

}

trait Node where {}
impl<V, L, R> Node for BNode<V, L, R>  where L: Height, L::Out: Equal<<R as Height>::Out>, R: Height {}
impl<V, L, R> Node for RNode<V, L, R>  where L: Black + Height, L::Out: Equal<<R as Height>::Out>,  R: Black + Height {}

trait Black{}
trait Red{}

impl<V, L, R> Black for BNode<V, L, R>{}
impl Black for Leaf{}
impl<V, L, R> Red for RNode<V, L, R>{}

trait Height { type Out: Number; }

impl Height for Leaf {
    type Out = Zero;
}

impl<V, L, R> Height for BNode<V, L, R> where L: Height, R: Height, <L as Height>::Out: Max<<R as Height>::Out> {
    type Out = Succ<<<L as Height>::Out as Max<<R as Height>::Out>>::Out>;
}

impl<V, L, R> Height for RNode<V, L, R>  where L: Height, R: Height, <L as Height>::Out: Max<<R as Height>::Out> {
    type Out = <<L as Height>::Out as Max<<R as Height>::Out>>::Out;
}

fn check_tree<A>() where A: Node {

}

// Red Black Tree Type Checking END -------------------------------------------

struct HCons<Head, Tail> {
    h: PhantomData<Head>,
    t: PhantomData<Tail>
}

struct HNil{}

struct Alive{}
struct Dead{}

type EMPTY5<A> = HCons<Dead, HCons<Dead, HCons<Dead, HCons<Dead, HCons<Dead, A>>>>>;
type EMPTY10<A> = EMPTY5<EMPTY5<A>>;
type ARRAY = HCons<Alive, HNil>;
type I11 = P10<P10<Succ<Zero>>>;


trait Pretty<Index> where Index: Number {
    fn repr() -> String;
}

impl<Index> Pretty<Index> for HNil where Index: Number {
    fn repr() -> String {
        return String::from("");
    }
}

impl<A, Index> Pretty<Index> for HCons<Dead, A> where A: Pretty<Index>, Index: Number {
    fn repr() -> String {
        format!("- {}", A::repr())
    }
}

impl<A, Index> Pretty<Index> for HCons<Alive, A> where A: Pretty<Index>, Index: Number {
    fn repr() -> String {
        format!("+ {}", A::repr())
    }
}

fn main() {
    //tree well formed: compiles
    //check_tree::<BNode<i32, BNode<i32, Leaf, Leaf>, BNode<i32, RNode<i32, Leaf, Leaf>, Leaf>>>();
    //tree not well formed: doesn't compile
    //check_tree::<BNode<i32, BNode<i32, Leaf, Leaf>, BNode<i32, BNode<i32, Leaf, Leaf>, Leaf>>>();

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arithmetic_works() {
        assert_eq!(incr::<I3, _>(), 4);
        assert_eq!(decr::<Zero, _>(), 0);
        assert_eq!(decr::<I5, _>(), 4);
        assert_eq!(add::<P10<Zero>, P5<Zero>, _>(), 15);
        assert_eq!(sub::<P10<Zero>, P5<Zero>, _>(), 5);
        assert_eq!(sub::<P10<Zero>, P10<I1>, _>(), 0);
        assert_eq!(mul::<I4, P10<Zero>, _>(), 40);
        assert_eq!(div::<P10<Zero>, I5, _>(), 2);
        assert_eq!(div::<P50<Zero>, I5, _>(), 10);
        assert_eq!(div::<I3, I4, _>(), 0);
        assert_eq!(pow::<I2, I5, _>(), 32);
        assert_eq!(max::<P10<Zero>, P50<Succ<Zero>>, _>(), 51);
        assert_eq!(max::<Zero, Zero, _>(), 0);
        assert_eq!(max::<I1, Zero, _>(), 1);
        assert_eq!(conditional_generic::<False, <I5 as Add<I2>>::Out, <I3 as Add<P50<I2>>>::Out, _>(), 55);
        assert_eq!(conditional_generic::<True, <I5 as Add<I2>>::Out, <I3 as Add<P50<I2>>>::Out, _>(), 7);
    }
}


