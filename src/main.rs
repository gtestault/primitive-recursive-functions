#![allow(dead_code)]
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

trait Add<RHS> {
    type Out: Number;
}

//f(x, 0) = x
impl<RHS> Add<RHS> for Zero where RHS: Number {
    type Out = RHS;
}

// f(x, y + 1) -> f(x, y) + x
impl<LHS, RHS> Add<RHS> for Succ<LHS> where RHS: Add<LHS> {
    type Out = Succ<<RHS as Add<LHS>>::Out>;
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

trait Pow<A> {
    type Out: Number;
}

// f(x, 0) = 1
impl<A> Pow<A> for Zero {
    type Out =  Succ<Zero>;
}

// f(x, y + 1) = xf(x, y)
impl<A, E> Pow<A> for Succ<E> where E: Pow<A>, E::Out: Mul<A> {
    type Out = <<E as Pow<A>>::Out as Mul<A>>::Out;
}

fn incr<A, B>() where A: Incr<Out=B>, B: Number {
    println!("{}", B::repr());
}

fn add<LHS, RHS, B>() where LHS: Add<RHS, Out=B>, B: Number {
    println!("{}", B::repr());
}

fn mul<LHS, RHS, B>() where LHS: Mul<RHS, Out=B>, B: Number {
    println!("{}", B::repr());
}

fn pow<A, E, B>() where E: Pow<A, Out=B>, B: Number {
    println!("{}", B::repr());
}

type I1 = Succ<Zero>;
type I2 = Succ<Succ<Zero>>;
type I3 = Succ<Succ<Succ<Zero>>>;
type I4 = Succ<Succ<Succ<Succ<Zero>>>>;
type I5 = Succ<Succ<Succ<Succ<Succ<Zero>>>>>;
type P5<N> = Succ<Succ<Succ<Succ<Succ<N>>>>>;
type P10<N> = P5<P5<N>>;
type P50<N> = P10<P10<P10<P10<P10<N>>>>>;

fn main() {
    //add::<P10<Zero>, P5<Zero>, _ >();
    //mul::<I4, P10<Zero>, _>();
    pow::<I2, I5, _>();
}
