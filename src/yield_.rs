//! # yeild
//!
//! generator yield implmentation
//!

use std::any::Any;
use generator::Generator;
use rt::{Context, ContextStack};
use reg_context::Context as RegContext;

/// yield error types
#[allow(dead_code)]
pub enum Error {
    Cancel,
    StackErr,
    ContextErr,
}

/// switch back to parent context
#[inline]
pub fn yield_now() {
    let env = ContextStack::current();
    let mut cur = env.top();
    let ref sp = cur.stack;
    // judge if this is root context
    if !sp.is_empty() {
        env.pop();
        let parent = env.top();
        RegContext::swap(&mut cur.regs, &parent.regs);
    }
}

/// raw yiled without catch passed in para
#[inline]
fn raw_yield<T: Any>(context: &mut Context, v: T) {
    // check the context
    if !context.is_generator() {
        panic!(Error::ContextErr);
    }

    context.set_ret(v);
    context._ref -= 1;
    yield_now();

    // here we just panic to exit the func
    if context._ref != 1 {
        panic!(Error::Cancel);
    }
}

/// yiled something without catch passed in para
#[inline]
pub fn yield_with<T: Any>(v: T) {
    raw_yield(ContextStack::current().top(), v);
}

/// yiled with something and return the passed in para
#[inline]
pub fn get_yield<A: Any, T: Any>(v: T) -> Option<A> {
    let context = ContextStack::current().top();
    let p = context.get_para();
    raw_yield(context, v);
    p
}

/// yiled_from
pub fn yield_from<'a, A: Any, T: Any>(g: Box<Generator<A, Output = T> + 'a>) {
    let context = ContextStack::current().top();
    let mut g = g;
    while !g.is_done() {
        let p = context.get_para();
        let r = g.raw_send(p).unwrap();
        raw_yield(context, r);
    }
}

/// yiled and get the send para
#[macro_export]
macro_rules! _yield {
    // `(para)`
    // val: the value that need to be yield
    // and got the send para from context
    ($val:expr) => (generator::get_yield($val).unwrap());

    () => (_yield!(()));
}

/// yield without get the passed in para
#[macro_export]
macro_rules! _yield_ {
    // `(para)`
    // val: the value that need to be yield
    // and got the send para from context
    ($val:expr) => (generator::yield_with($val));

    () => (_yield_!(()));
}
