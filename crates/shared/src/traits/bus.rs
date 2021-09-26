// A bus is a communication system that transfers data between components inside a computer.
//
// The structs holding memory should implement this trait to manage the storage and retrieval of data.
//
// Functions:
//  - get: return {Item} at location {T} in &self.
//  - set: Set {Data} at location {T} in &self, return {Result}
//
// Types:
//  - T: The type of data the bus is indexed by.
//  - Item: The type of data returned by a get on the Bus.
//  - Result: The type of data return by a set on the Bus.
//  - Data: The type of Data set by the bus.

use std::fmt::Debug;

pub trait Bus<T> : Debug {
    type Item;
    type Result;
    type Data;

    fn get(&self, _: T) -> Self::Item;
    fn set(&mut self, _: T, data: Self::Data) -> Self::Result;
}

pub trait Memory {}

impl<T, A> Bus<&T> for A
where
    A: Bus<T>,
    A: Memory,
    T: Copy,
{
    type Item = A::Item;
    type Result = A::Result;
    type Data = A::Data;

    fn get(&self, id: &T) -> Self::Item {
        self.get(*id)
    }

    fn set(&mut self, id: &T, data: Self::Data) -> Self::Result {
        self.set(*id, data)
    }
}
