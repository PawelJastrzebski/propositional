use super::core::*;
use std::rc::Rc;

// Not
#[derive(Clone)]
pub struct Not {
    sentence: Rc<dyn Sentence>,
}

impl Not {
    pub fn new<A>(sentence: &A) -> Self
    where
        A: Sentence + Clone,
    {
        Self {
            sentence: Rc::new(sentence.clone()),
        }
    }
}

pub fn Not<A>(sentence: &A) -> Not
where
    A: Sentence + Clone,
{
    Not::new(sentence)
}

impl Sentence for Not {
    // Evaluate against model
    fn eval(&self, model: &Model) -> Option<bool> {
        let Some(a) = self.sentence.eval(model) else {
            return None;
        };

        Some(!a)
    }

    fn symbols(&self) -> Vec<&Symbol> {
        self.sentence.symbols()
    }
}

#[macro_export]
macro_rules! not {
    ($e:expr) => {
        Not(&$e)
    };
}
pub use not;

// And

#[derive(Clone)]
pub struct And {
    sentences: Vec<Rc<dyn Sentence>>,
}

impl And {
    pub fn new() -> Self {
        Self {
            sentences: Vec::new(),
        }
    }

    pub fn And<T: Sentence + Clone>(mut self, sentence: &T) -> Self {
        self.sentences.push(Rc::new(sentence.clone()));
        self
    }
}

impl Sentence for And {
    fn eval(&self, model: &Model) -> Option<bool> {
        for s in self.sentences.iter() {
            if let Some(value) = s.eval(model) {
                if value {
                    continue;
                } else {
                    return Some(false);
                }
            }
        }
        Some(true)
    }

    fn symbols(&self) -> Vec<&Symbol> {
        self.sentences.iter().flat_map(|s| s.symbols()).collect()
    }
}

pub fn And<T: Sentence + Clone>(sentence: &T) -> And {
    And::new().And(sentence)
}

#[macro_export]
macro_rules! and {
    () => { And::new() };
    ($e:expr) => { And(&$e) };
    ($e:expr, $($es:expr),+) => {
        And(&$e)
            $(.And(&$es))
            *
    };
}

#[allow(unused_imports)]
pub use and;

// Or

#[derive(Clone)]
pub struct Or {
    sentences: Vec<Rc<dyn Sentence>>,
}

impl Or {
    pub fn new() -> Self {
        Self {
            sentences: Vec::new(),
        }
    }

    pub fn Or<T: Sentence + Clone>(mut self, sentence: &T) -> Self {
        self.sentences.push(Rc::new(sentence.clone()));
        self
    }
}

impl Sentence for Or {
    fn eval(&self, model: &Model) -> Option<bool> {
        for s in self.sentences.iter() {
            if let Some(value) = s.eval(model) {
                if value {
                    return Some(true);
                } else {
                    continue;
                }
            }
        }
        Some(false)
    }

    fn symbols(&self) -> Vec<&Symbol> {
        self.sentences.iter().flat_map(|s| s.symbols()).collect()
    }
}

pub fn Or<T: Sentence + Clone>(sentence: &T) -> Or {
    Or::new().Or(sentence)
}

#[macro_export]
macro_rules! or {
    () => { Or::new() };
    ($e:expr) => { Or(&$e) };
    ($e:expr, $($es:expr),+) => {
        Or(&$e)
            $(.Or(&$es))
            *
    };
}

#[allow(unused_imports)]
pub use or;

// Implication

#[derive(Clone)]
pub struct Implies {
    antecedent: Rc<dyn Sentence>,
    consequent: Rc<dyn Sentence>,
}

impl Sentence for Implies {
    fn eval(&self, model: &Model) -> Option<bool> {
        let Some(left_eval) = self.antecedent.eval(model) else {
            return None;
        };

        let Some(right_eval) = self.consequent.eval(model) else {
            return None;
        };

        Some(!left_eval || right_eval)
    }

    fn symbols(&self) -> Vec<&Symbol> {
        vec![self.antecedent.symbols(), self.consequent.symbols()]
            .into_iter()
            .flat_map(|f| f)
            .collect()
    }
}

impl Implies {
    pub fn new<A, C>(antecedent: &A, consequent: &C) -> Self
    where
        A: Sentence + Clone,
        C: Sentence + Clone,
    {
        Self {
            antecedent: Rc::new(antecedent.clone()),
            consequent: Rc::new(consequent.clone()),
        }
    }
}

pub fn Implies<A, C>(antecedent: &A, consequent: &C) -> Implies
where
    A: Sentence + Clone,
    C: Sentence + Clone,
{
    Implies::new(antecedent, consequent)
}

#[macro_export]
macro_rules! implies {
    ($l:expr, $r:expr) => {
        Implies(&$l, &$r)
    };
}

pub use implies;

// Biconditional

#[derive(Clone)]
pub struct Biconditional {
    left: Rc<dyn Sentence>,
    right: Rc<dyn Sentence>,
}

impl Sentence for Biconditional {
    fn eval(&self, model: &Model) -> Option<bool> {
        let Some(left_eval) = self.left.eval(model) else {
            return None;
        };

        let Some(right_eval) = self.right.eval(model) else {
            return None;
        };

        Some(left_eval == right_eval)
    }

    fn symbols(&self) -> Vec<&Symbol> {
        vec![self.left.symbols(), self.right.symbols()]
            .into_iter()
            .flat_map(|f| f)
            .collect()
    }
}

impl Biconditional {
    pub fn new<L, R>(left: &L, right: &R) -> Self
    where
        L: Sentence + Clone,
        R: Sentence + Clone,
    {
        Self {
            left: Rc::new(left.clone()),
            right: Rc::new(right.clone()),
        }
    }
}

pub fn Biconditional<L, R>(left: &L, right: &R) -> Biconditional
where
    L: Sentence + Clone,
    R: Sentence + Clone,
{
    Biconditional::new(left, right)
}

#[macro_export]
macro_rules! biconditional {
    ($l:expr, $r:expr) => {
        Biconditional(&$l, &$r)
    };
}

pub use biconditional;

#[cfg(test)]
pub mod tests {
    use crate::prelude::*;

    #[test]
    fn not_true_table() {
        let p = symbol!("P");

        // ¬P is equivalent to !P
        model!(p => true).assert_false(&not!(p));
        model!(p => false).assert_true(&not!(p));
    }

    #[test]
    fn or_true_table() {
        let a = symbol!("A");
        let b = symbol!("B");

         // A ∨ B is equivalent to A || B
        model!(a => false, b => false).assert_false(&or!(a, b));
        model!(a => false, b => true).assert_true(&or!(a, b));
        model!(a => true, b => false).assert_true(&or!(a, b));
        model!(a => true, b => true).assert_true(&or!(a, b));
    }

    #[test]
    fn and_true_table() {
        let a = symbol!("A");
        let b = symbol!("B");

        // A ∧ B is equivalent to A && B
        model!(a => false, b => false).assert_false(&and!(a, b));
        model!(a => false, b => true).assert_false(&and!(a, b));
        model!(a => true, b => false).assert_false(&and!(a, b));
        model!(a => true, b => true).assert_true(&and!(a, b));
    }

    #[test]
    fn implies_true_table() {
        let a = symbol!("A");
        let b = symbol!("B");

        // A => B is equivalent to !A || B
        model!(a => false, b => false).assert_true(&implies!(a, b));
        model!(a => false, b => true).assert_true(&implies!(a, b));
        model!(a => true, b => false).assert_false(&implies!(a, b));
        model!(a => true, b => true).assert_true(&implies!(a, b));
    }


    #[test]
    fn biconditional_true_table() {
        let a = symbol!("A");
        let b = symbol!("B");

        // A <=> B is equivalent to (A => B) && (B => A)
        model!(a => false, b => false).assert_true(&biconditional!(a, b));
        model!(a => false, b => true).assert_false(&biconditional!(a, b));
        model!(a => true, b => false).assert_false(&biconditional!(a, b));
        model!(a => true, b => true).assert_true(&biconditional!(a, b));
    }
}
