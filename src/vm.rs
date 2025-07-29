//! shared VM implementation

#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use crate::config;

/// atom/primitive types
#[derive(Copy, Clone)]
pub enum Value {
    /// signed integer
    Int(i32),
    /// floating point
    Float(f32),
    /// text char
    Char(char),
    /// boolean
    Bool(bool),
    /// empty element
    Nil,
}

impl Default for Value {
    fn default() -> Self {
        Value::Nil
    }
}

/// Virtual Machine command
pub type Cmd = fn();

pub fn trace(msg: &str) {
    eprintln!("{}", msg);
}

/// `( -- )` do nothing
fn nop() {
    trace("nop");
}

/// `( -- )` stop system
fn halt() {
    trace("halt");
    std::process::exit(0);
}

/// low-level bytecode
pub enum ByteCode {
    /// primitives can be used as is
    Prim(Value),
    /// functions can be used as VM command
    Cmd(Cmd),
}

/// executable sequence
pub type Seq = Vec<ByteCode>;

/// generic fixed size stack
/// `T` elements type
/// `S:usize` fixed size
pub struct Stack<T, const S: usize> {
    data: [T; S],
    pointer: usize,
}

impl<T: Default + Copy, const S: usize> Stack<T, S> {
    pub fn new() -> Self {
        Stack {
            data: [T::default(); S],
            pointer: 0,
        }
    }

    /// `( -- item )`
    pub fn push(&mut self, item: T) -> &mut Self {
        assert!(self.pointer < S);
        self.data[self.pointer] = item;
        self.pointer += 1;
        self
    }

    /// `( item -- )`
    pub fn pop(&mut self) -> T {
        assert!(self.pointer > 0);
        self.pointer -= 1;
        self.data[self.pointer]
    }

    /// `( item -- item )`
    pub fn top(&self) -> T {
        assert!(self.pointer > 0);
        self.data[self.pointer - 1]
    }

    /// `( ... -- )`
    pub fn clean(&mut self) -> &Self {
        self.pointer = 0;
        self
    }

    pub fn empty(&self) -> bool {
        self.pointer == 0
    }

    pub fn full(&self) -> bool {
        self.pointer >= S
    }

    /// `( a -- a a )`
    pub fn dup(&mut self) -> &Self {
        let a = self.top();
        self.push(a)
    }

    /// `( a -- )`
    pub fn drop(&mut self) -> &Self {
        self.pop();
        self
    }

    /// `( a b -- b a )`
    pub fn swap(&mut self) -> &Self {
        let b = self.pop();
        let a = self.pop();
        self.push(b).push(a)
    }

    /// `( a b -- a b a )`
    pub fn over(&mut self) -> &Self {
        let b = self.pop();
        let a = self.top();
        self.push(b).push(a)
    }

    /// `( a b c -- b c a )`
    pub fn rot(&mut self) -> &Self {
        let c = self.pop();
        let b = self.pop();
        let a = self.pop();
        self.push(b).push(c).push(a)
    }
}

/// Virtual Machine execution context
pub struct VM {
    /// data stack
    data: Stack<Value, { config::vm::Dsz }>,
    /// return stack
    ret: Stack<usize, { config::vm::Rsz }>,
    /// active sequence
    seq: Seq,
    /// execution pointer
    ip: usize,
}

impl VM {
    pub fn new() -> Self {
        VM {
            data: Stack::new(),
            ret: Stack::new(),
            seq: Vec::new(),
            ip: 0,
        }
    }
    pub fn run(&self) {}
}
