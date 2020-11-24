//! Boolean expressions using [Reverse Polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation).

use std::{convert::TryFrom, ops::Index};

use serde::{Deserialize, Serialize};

/// Implemented by data stuctures containing variable data for [`BoolExpr`].
///
/// # Example
/// ```
/// use std::convert::TryFrom;
/// # use trailblazer::bool_expr::{BoolExpr, BoolLookup};
///
/// struct Id(u8);
///
/// impl TryFrom<&str> for Id {
///     type Error = String;
///
///     fn try_from(value: &str) -> Result<Self, Self::Error> {
///         match value {
///             "a" => Ok(Id(0)),
///             "b" => Ok(Id(1)),
///             v => Err(format!("Unknown variable {}.", v)),
///         }
///     }
/// }
///
/// struct Eval;
///
/// impl BoolLookup<&Id> for Eval {
///     fn lookup_bool(&self, id: &Id) -> bool {
///         id.0 == 0
///     }
/// }
///
/// assert_eq!(BoolExpr::<Id>::try_from("a b &").unwrap().eval(&Eval), false);
/// assert_eq!(BoolExpr::<Id>::try_from("a b |").unwrap().eval(&Eval), true);
/// ```
pub trait BoolLookup<T> {
    /// Get the value of the variable identified by `id`.
    fn lookup_bool(&self, id: T) -> bool;
}

impl<T, I: Index<T, Output = bool>> BoolLookup<T> for I {
    fn lookup_bool(&self, id: T) -> bool {
        self[id]
    }
}

// impl<'a, T: 'a + Copy, I: Index<T, Output = bool>> BoolLookup<'a, T> for I {
//     fn lookup_bool(&self, id: &'a T) -> bool {
//         self[*id]
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
enum BoolExprElement<T> {
    Not,
    And,
    Or,
    Const(bool),
    Value(T),
}

impl<'a, T: TryFrom<&'a str, Error = String>> TryFrom<&'a str> for BoolExprElement<T> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "1" => Ok(Self::Const(true)),
            "0" => Ok(Self::Const(false)),
            "!" => Ok(Self::Not),
            "&" => Ok(Self::And),
            "|" => Ok(Self::Or),
            value => T::try_from(value).map(Self::Value),
        }
    }
}

/// Boolean expression.
///
/// Values of type parameter `T` should uniquely identify a variable via implementations of
/// [`BoolLookup`]. Enums or [newtypes](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
/// of unsinged integers recommended.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BoolExpr<T>(Vec<BoolExprElement<T>>);

impl<T> BoolExpr<T> {
    fn validate(self) -> Result<Self, String> {
        let mut stack = 0;

        for item in &self.0 {
            stack += match item {
                BoolExprElement::Not => 0,
                BoolExprElement::And | BoolExprElement::Or => -1,
                BoolExprElement::Const(_) | BoolExprElement::Value(_) => 1,
            };
        }

        if stack <= 1 {
            Ok(self)
        } else {
            Err(format!(
                "Evaluation will result in stack size of {}.",
                stack
            ))
        }
    }

    /// Evaluate a boolean expression.
    ///
    /// The parameter `vars` must be a function mapping variable ids to values.
    ///
    /// # Example
    /// ```
    /// use std::{collections::HashMap, convert::TryFrom};
    /// # use trailblazer::bool_expr::{BoolExpr, BoolLookup};
    ///
    /// #[derive(PartialEq, Eq, Hash)]
    /// struct Id(String);
    ///
    /// impl TryFrom<&str> for Id {
    ///     type Error = String;
    ///
    ///     fn try_from(value: &str) -> Result<Self, Self::Error> {
    ///         Ok(Id(value.to_string()))
    ///     }
    /// }
    ///
    /// let mut vars: HashMap<Id, bool> = HashMap::new();
    /// let expr = BoolExpr::<Id>::try_from("a b &").unwrap();
    ///
    /// vars.insert(Id("a".to_string()), true);
    /// vars.insert(Id("b".to_string()), false);
    /// assert_eq!(expr.eval(&vars), false);
    ///
    /// vars.insert(Id("b".to_string()), true);
    /// assert_eq!(expr.eval(&vars), true);
    /// ```
    pub fn eval<'a, L: BoolLookup<&'a T>>(&'a self, lookup: &'a L) -> bool {
        let mut stack: Vec<bool> = Vec::new();

        for item in &self.0 {
            let res = match item {
                BoolExprElement::Not => !stack.pop().unwrap(),
                BoolExprElement::And => stack.pop().unwrap() & stack.pop().unwrap(),
                BoolExprElement::Or => stack.pop().unwrap() | stack.pop().unwrap(),
                BoolExprElement::Const(c) => *c,
                BoolExprElement::Value(v) => lookup.lookup_bool(v),
            };
            stack.push(res);
        }

        debug_assert!(stack.len() <= 1);
        *stack.first().unwrap_or(&true)
    }
}

impl<'a, T: TryFrom<&'a str, Error = String>> TryFrom<&'a str> for BoolExpr<T> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let elements: Result<Vec<BoolExprElement<T>>, Self::Error> = value
            .split(char::is_whitespace)
            .filter(|s| !s.is_empty())
            .map(BoolExprElement::try_from)
            .collect();
        elements.map(Self).and_then(Self::validate)
    }
}
