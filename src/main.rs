#![allow(dead_code)]
#![recursion_limit = "100000"]
use std::marker::PhantomData;

fn main() {}

//Truth types
type False = Zero;
type True = Succ<Zero>;

//arithmetic type constructor inspired by: [tylar](https://github.com/Boddlnagg/tylar)
//Nat Number types
struct Zero {}
struct Succ<A> {
    phantom: PhantomData<A>,
}
type I1 = Succ<Zero>;
type I2 = Succ<Succ<Zero>>;
type I3 = Succ<Succ<Succ<Zero>>>;
type I4 = Succ<Succ<Succ<Succ<Zero>>>>;
type I5 = Succ<Succ<Succ<Succ<Succ<Zero>>>>>;
type I6 = P5<I1>;
type I7 = P5<I2>;
type I8 = P5<I3>;
type I9 = P5<I4>;
type I10 = P10<Zero>;
type I11 = P10<Succ<Zero>>;
type I110 = P50<P50<P10<Zero>>>;

//Nat Number Constructors
type P5<N> = Succ<Succ<Succ<Succ<Succ<N>>>>>;
type P10<N> = P5<P5<N>>;
type P50<N> = P10<P10<P10<P10<P10<N>>>>>;

trait Number {
    fn repr() -> i32;
}

impl Number for Zero {
    fn repr() -> i32 {
        0
    }
}

impl<A> Number for Succ<A>
where
    A: Number,
{
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

impl<A> Incr for Succ<A>
where
    A: Number,
{
    type Out = Succ<Succ<A>>;
}

trait Decr {
    type Out: Number + Decr;
}

impl Decr for Zero {
    type Out = Zero;
}

impl<A> Decr for Succ<A>
where
    A: Number + Decr,
{
    type Out = A;
}

trait Add<RHS> {
    type Out: Number;
}

//f(x, 0) = x
impl<RHS> Add<RHS> for Zero
where
    RHS: Number,
{
    type Out = RHS;
}

// f(x, n + 1) -> f(x, n) + 1
impl<LHS, RHS> Add<RHS> for Succ<LHS>
where
    RHS: Add<LHS>,
{
    type Out = Succ<<RHS as Add<LHS>>::Out>;
}

// Subtraction only on natural numbers. Negative numbers are mapped to Zero.
trait Sub<RHS> {
    type Out: Number;
}

impl<LHS> Sub<LHS> for Zero
where
    LHS: Number,
{
    type Out = LHS;
}

impl<A, LHS> Sub<LHS> for Succ<A>
where
    A: Sub<<LHS as Decr>::Out>,
    LHS: Decr,
{
    type Out = <A as Sub<<LHS as Decr>::Out>>::Out;
}

trait Mul<RHS> {
    type Out: Number;
}

// f(x, 0) = 0
impl<RHS> Mul<RHS> for Zero {
    type Out = Zero;
}

// f(x, y + 1) = f(x, y) + x
impl<A, RHS> Mul<RHS> for Succ<A>
where
    A: Mul<RHS>,
    A::Out: Add<RHS>,
{
    type Out = <<A as Mul<RHS>>::Out as Add<RHS>>::Out;
}

trait Div<RHS> {
    type Out: Number;
}

impl<RHS> Div<RHS> for Zero {
    type Out = Zero;
}

impl<A, RHS> Div<RHS> for Succ<A>
where
    Succ<A>: Larger<RHS>,
    RHS: Sub<Succ<A>>,
    <RHS as Sub<Succ<A>>>::Out: Div<RHS>,
    //Compiler generated clauses start--------------------------------------------------------------
    <Succ<A> as Larger<RHS>>::Out: If<
        Succ<<<RHS as Sub<Succ<A>>>::Out as Div<RHS>>::Out>,
        <<RHS as Sub<Succ<A>>>::Out as Div<RHS>>::Out,
    >,
    <<Succ<A> as Larger<RHS>>::Out as If<
        Succ<<<RHS as Sub<Succ<A>>>::Out as Div<RHS>>::Out>,
        <<RHS as Sub<Succ<A>>>::Out as Div<RHS>>::Out,
    >>::Out: Number,
{
    type Out = <<Succ<A> as Larger<RHS>>::Out as If<
        Succ<<<RHS as Sub<Succ<A>>>::Out as Div<RHS>>::Out>,
        <<RHS as Sub<Succ<A>>>::Out as Div<RHS>>::Out,
    >>::Out;
    //Compiler generated clauses end----------------------------------------------------------------
}

trait Remainder<RHS> {
    type Out: Number;
}

// (a mod b) = a - (a/b)*b where (a/b) is integer division.
impl<RHS, A> Remainder<RHS> for Succ<A>
where
    Succ<A>: Larger<RHS>,
    RHS: Sub<Succ<A>>,
    //Compiler generated clauses start--------------------------------------------------------------
    <RHS as Sub<Succ<A>>>::Out: Div<RHS>,
    <Succ<A> as Larger<RHS>>::Out: If<
        Succ<<<RHS as Sub<Succ<A>>>::Out as Div<RHS>>::Out>,
        <<RHS as Sub<Succ<A>>>::Out as Div<RHS>>::Out,
    >,
    <<Succ<A> as Larger<RHS>>::Out as If<
        Succ<<<RHS as Sub<Succ<A>>>::Out as Div<RHS>>::Out>,
        <<RHS as Sub<Succ<A>>>::Out as Div<RHS>>::Out,
    >>::Out: Number,
    <<Succ<A> as Larger<RHS>>::Out as If<
        Succ<<<RHS as Sub<Succ<A>>>::Out as Div<RHS>>::Out>,
        <<RHS as Sub<Succ<A>>>::Out as Div<RHS>>::Out,
    >>::Out: Mul<RHS>,
    <<<Succ<A> as Larger<RHS>>::Out as If<
        Succ<<<RHS as Sub<Succ<A>>>::Out as Div<RHS>>::Out>,
        <<RHS as Sub<Succ<A>>>::Out as Div<RHS>>::Out,
    >>::Out as Mul<RHS>>::Out: Sub<Succ<A>>,
    //Compiler generated clauses end----------------------------------------------------------------
{
    type Out = <<<Succ<A> as Div<RHS>>::Out as Mul<RHS>>::Out as Sub<Succ<A>>>::Out;
}

impl<RHS> Remainder<RHS> for Zero {
    type Out = Zero;
}

trait Pow<A> {
    type Out: Number;
}

// f(x, 0) = 1
impl<A> Pow<A> for Zero {
    type Out = Succ<Zero>;
}

// f(x, y + 1) = x*f(x, y)
impl<A, E> Pow<A> for Succ<E>
where
    E: Pow<A>,
    E::Out: Mul<A>,
{
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
trait EqualFailing<A> {
    type Out;
}

impl EqualFailing<Zero> for Zero {
    type Out = Succ<Zero>;
}

impl<A, B> EqualFailing<Succ<B>> for Succ<A>
where
    B: EqualFailing<A>,
{
    type Out = <B as EqualFailing<A>>::Out;
}

trait Max<B> {
    type Out: Number;
}

//Recursion start
impl<A, B> Max<Succ<B>> for Succ<A>
where
    A: Max<B>,
{
    type Out = Succ<<A as Max<B>>::Out>;
}

//Recursion end: Type Self < B (trait parameter) are Equal
impl<A> Max<Succ<A>> for Zero
where
    A: Number,
{
    type Out = Succ<A>;
}
//Recursion end: Type Self > B (trait parameter) are Equal
impl<A> Max<Zero> for Succ<A>
where
    A: Number,
{
    type Out = Succ<A>;
}

//Recursion end: Type Self and B are Equal
impl Max<Zero> for Zero {
    type Out = Zero;
}

// Checks if Self is larger or equal to the Type Parameter. Returns a Boolean type True/False
trait Larger<B> {
    type Out: Number;
}

//Recursion start
impl<A, B> Larger<Succ<B>> for Succ<A>
where
    A: Larger<B>,
{
    type Out = <A as Larger<B>>::Out;
}

//Recursion end: Type Self < B (trait parameter) are Equal
impl<A> Larger<Succ<A>> for Zero
where
    A: Number,
{
    type Out = False;
}
//Recursion end: Type Self > B (trait parameter) are Equal
impl<A> Larger<Zero> for Succ<A>
where
    A: Number,
{
    type Out = True;
}

//Recursion end: Type Self and B are Equal
impl Larger<Zero> for Zero {
    type Out = True;
}

trait Less<A> {
    type Out: Number;
}

//Recursion start
impl<A, B> Less<Succ<B>> for Succ<A>
where
    A: Less<B>,
{
    type Out = <A as Less<B>>::Out;
}

//Recursion end: Type Self < B (trait parameter) are Equal
impl<A> Less<Succ<A>> for Zero
where
    A: Number,
{
    type Out = True;
}
//Recursion end: Type Self > B (trait parameter) are Equal
impl<A> Less<Zero> for Succ<A>
where
    A: Number,
{
    type Out = False;
}

//Recursion end: Type Self and B are Equal
impl Less<Zero> for Zero {
    type Out = False;
}

trait Equal<B> {
    type Out: Number;
}

//Recursion start
impl<A, B> Equal<Succ<B>> for Succ<A>
where
    A: Equal<B>,
{
    type Out = <A as Equal<B>>::Out;
}

//Recursion end: Type Self < B (trait parameter) are Equal
impl<A> Equal<Succ<A>> for Zero
where
    A: Number,
{
    type Out = False;
}
//Recursion end: Type Self > B (trait parameter) are Equal
impl<A> Equal<Zero> for Succ<A>
where
    A: Number,
{
    type Out = False;
}

//Recursion end: Type Self and B are Equal
impl Equal<Zero> for Zero {
    type Out = True;
}

trait Or<B> {
    type Out: Number;
}

impl Or<True> for False {
    type Out = True;
}

impl Or<True> for True {
    type Out = True;
}

impl Or<False> for True {
    type Out = True;
}

impl Or<False> for False {
    type Out = False;
}

// When calling the function the last generic Type (B) must be set to '_' in order for the compiler to infer the result type of the operation.
// If we set the B generic type and it is not the result type of the operation, the rust unification process will fail and the type checker will error out.
fn incr<A, B>() -> i32
where
    A: Incr<Out = B>,
    B: Number,
{
    B::repr()
}

fn decr<A, B>() -> i32
where
    A: Decr<Out = B>,
    B: Number + Decr,
{
    B::repr()
}

fn add<LHS, RHS, B>() -> i32
where
    LHS: Add<RHS, Out = B>,
    B: Number,
{
    B::repr()
}

fn sub<LHS, RHS, Result>() -> i32
where
    RHS: Sub<LHS, Out = Result>,
    Result: Number,
{
    Result::repr()
}

fn mul<LHS, RHS, Result>() -> i32
where
    LHS: Mul<RHS, Out = Result>,
    Result: Number,
{
    Result::repr()
}

fn div<LHS, RHS, Result>() -> i32
where
    LHS: Div<RHS, Out = Result>,
    Result: Number,
{
    Result::repr()
}

fn remainder<LHS, RHS, Result>() -> i32
where
    LHS: Remainder<RHS, Out = Result>,
    Result: Number,
{
    Result::repr()
}

fn pow<A, E, Result>() -> i32
where
    E: Pow<A, Out = Result>,
    Result: Number,
{
    Result::repr()
}

fn conditional_mul<A, B, C, Result>()
where
    A: If<<B as Mul<C>>::Out, Zero, Out = Result>,
    B: Mul<C>,
    Result: Number,
{
    println!("{}", Result::repr());
}

fn conditional_generic<A, B, C, Result>() -> i32
where
    A: If<B, C, Out = Result>,
    Result: Number,
{
    Result::repr()
}

fn equal_failing<A, B, C>()
where
    A: EqualFailing<B, Out = C>,
    C: Number,
{
    println!("{}", C::repr());
}

fn equal<A, B, Result>() -> i32
where
    A: Equal<B, Out = Result>,
    Result: Number,
{
    Result::repr()
}

fn max<A, B, C>() -> i32
where
    A: Max<B, Out = C>,
    C: Number,
{
    C::repr()
}

fn less<A, B, Result>() -> i32
where
    A: Less<B, Out = Result>,
    Result: Number,
{
    Result::repr()
}

fn larger_equal<A, B, Result>() -> i32
where
    A: Larger<B, Out = Result>,
    Result: Number,
{
    Result::repr()
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
        assert_eq!(remainder::<I3, I4, _>(), 3);
        assert_eq!(remainder::<P10<I1>, P5<I2>, _>(), 4);
        assert_eq!(pow::<I2, I5, _>(), 32);
        assert_eq!(max::<P10<Zero>, P50<Succ<Zero>>, _>(), 51);
        assert_eq!(max::<Zero, Zero, _>(), 0);
        assert_eq!(max::<I1, Zero, _>(), 1);
        assert_eq!(equal::<Zero, Zero, _>(), 1);
        assert_eq!(equal::<Zero, I1, _>(), 0);
        assert_eq!(equal::<I1, Zero, _>(), 0);
        assert_eq!(equal::<I4, I3, _>(), 0);
        assert_eq!(equal::<I4, I2, _>(), 0);
        assert_eq!(less::<Zero, Zero, _>(), 0);
        assert_eq!(less::<Zero, I1, _>(), 1);
        assert_eq!(less::<I3, P10<Zero>, _>(), 1);
        assert_eq!(less::<P10<Zero>, I3, _>(), 0);
        assert_eq!(larger_equal::<Zero, Zero, _>(), 1);
        assert_eq!(larger_equal::<Zero, I1, _>(), 0);
        assert_eq!(larger_equal::<I3, P10<Zero>, _>(), 0);
        assert_eq!(larger_equal::<P10<Zero>, I3, _>(), 1);
        assert_eq!(
            conditional_generic::<False, <I5 as Add<I2>>::Out, <I3 as Add<P50<I2>>>::Out, _>(),
            55
        );
        assert_eq!(
            conditional_generic::<True, <I5 as Add<I2>>::Out, <I3 as Add<P50<I2>>>::Out, _>(),
            7
        );
    }
}
