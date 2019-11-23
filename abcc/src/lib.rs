// This file is a part of ABC.
// Copyright (C) 2019 Matthew Blount

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public
// License along with this program.  If not, see
// <https://www.gnu.org/licenses/.

//! The `abcc` crate provides a collection of compilers and
//! interpreters for ABC.
//!
//! ABC's main feature is evaluation by *rewriting*. This makes it
//! possible to pause & resume a computation, or to partially evaluate
//! a function with only some of its arguments.

use std::rc::Rc;

/// An error that may occur during a computation.
pub enum Error {
  Space,
  Time,
  Type,
  Assert,
  Syntax,
  Stub,
  Bug,
}

/// The result of a computation.
pub type Result<T> = std::result::Result<T, Error>;

/// The nine primitive combinators.
pub enum Constant {
  Apply,
  Quote,
  Compose,
  Copy,
  Drop,
  Swap,
  Shift,
  Reset,
  Bang,
}

/// A variable that may be replaced with a value.
pub struct Variable(pub Rc<str>);

impl Variable {
  /// Validate a variable name.
  pub fn read(name: Rc<str>) -> Result<Self> {
    // XXX TODO: Validate variables.
    return Ok(Variable(name));
  }
}

/// A place where a computation may occur.
pub trait Container {
  type Object;

  /// Deserialize an object from a string.
  fn read(&mut self, src: &str) -> Result<Self::Object>;
  /// Serialize an object to a string.
  fn show(&self, obj: Self::Object) -> Result<String>;

  /// Get the object associated with a variable.
  fn get(&self, key: Variable) -> Result<Option<Self::Object>>;
  /// Associate a value with a variable.
  fn put(&mut self, key: Variable, value: Self::Object) -> Result<Self::Object>;
  /// Remove the value associated with a variable.
  fn delete(&mut self, key: Variable) -> Result<Self::Object>;

  /// Rewrite an object until it reaches normal form, or until an
  /// effort quota is exhausted.
  fn normalize(&mut self, obj: Self::Object) -> Result<Self::Object>;

  /// Like `normalize`, but executes bangs using a handler.
  fn execute(&mut self, obj: Self::Object, ctx: dyn Handler<Self>) -> Result<Self::Object>;

  /// Extend an object using an analysis of the current environment.
  fn complete(&mut self, obj: Self::Object) -> Result<Self::Object>;

  /// Create an identity program.
  fn new_identity(&self) -> Result<Self::Object>;
  /// Create a new object for the given constant.
  fn new_constant(&mut self, name: Constant) -> Result<Self::Object>;
  /// Create a new object for the given variable.
  fn new_variable(&mut self, name: Variable) -> Result<Self::Object>;
  /// Create a new object for the given comment.
  fn new_comment(&mut self, body: Rc<str>) -> Result<Self::Object>;
  /// Create a new quotation with the given object as its body.
  fn new_quote(&mut self, body: Self::Object) -> Result<Self::Object>;
  /// Create a new sequential composition of the two objects given.
  fn new_sequence(
    &mut self,
    fst: Self::Object,
    snd: Self::Object) -> Result<Self::Object>;

  /// Returns true if this object is the identity function.
  fn is_identity(&self, obj: Self::Object) -> Result<bool>;
  /// Returns true if this object is a constant.
  fn is_constant(&self, obj: Self::Object) -> Result<bool>;
  /// Returns true if this object is a variable.
  fn is_variable(&self, obj: Self::Object) -> Result<bool>;
  /// Returns true if this object is a comment.
  fn is_comment(&self, obj: Self::Object) -> Result<bool>;
  /// Returns true if this object is a quotation.
  fn is_quote(&self, obj: Self::Object) -> Result<bool>;
  /// Returns true if this object is a sequence.
  fn is_sequence(&self, obj: Self::Object) -> Result<bool>;
  /// Returns true if this object is either `shift` or `reset`.
  fn is_prompt(&self, obj: Self::Object) -> Result<bool>;
  /// Returns true if this object is a bang.
  fn is_bang(&self, obj: Self::Object) -> Result<bool>;

  /// Returns true if `is_constant` returns true for any of this
  /// object's children. If this function returns false, then this
  /// object is open.
  fn has_constant(&self, obj: Self::Object) -> Result<bool>;
  /// Returns true if `is_variable` returns true for any of this
  /// object's children.
  fn has_variable(&self, obj: Self::Object) -> Result<bool>;
  /// Returns true if `is_comment` returns true for any of this
  /// object's children.
  fn has_comment(&self, obj: Self::Object) -> Result<bool>;
  /// Returns true if `is_quote` returns true for any of this object's
  /// children.
  fn has_quote(&self, obj: Self::Object) -> Result<bool>;
  /// Returns true if `is_prompt` returns true for any of this
  /// object's children.
  fn has_prompt(&self, obj: Self::Object) -> Result<bool>;
  /// Returns true if `is_bang` returns true for any of this object's
  /// children.
  fn has_bang(&self, obj: Self::Object) -> Result<bool>;

  /// Get the name associated with a constant.
  fn get_constant_name(&self, obj: Self::Object) -> Result<Constant>;
  /// Get the name associated with a variable.
  fn get_variable_name(&self, obj: Self::Object) -> Result<Variable>;
  /// Get the body of a comment.
  fn get_comment_body(&self, obj: Self::Object) -> Result<Rc<str>>;
  /// Get the body of a quotation.
  fn get_quote_body(&self, obj: Self::Object) -> Result<Self::Object>;
  /// Get the first element of a sequence.
  fn get_sequence_fst(&self, obj: Self::Object) -> Result<Self::Object>;
  /// Get the second element of a sequence.
  fn get_sequence_snd(&self, obj: Self::Object) -> Result<Self::Object>;

  /// Collect garbage, protecting the given objects and their
  /// children.
  fn collect(&mut self, xs: Vec<Self::Object>) -> Result<()>;
}

/// A delegate to provide an effectful interpretation of bangs, on
/// behalf of a `Container`.
pub trait Handler<C: Container> {
  fn execute(&mut self, args: Vec<C::Object>, ctx: &mut C) -> Result<Vec<C::Object>>;
}
