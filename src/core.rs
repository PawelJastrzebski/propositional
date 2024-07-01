use std::rc::Rc;

pub trait Sentence: 'static {
    fn eval(&self, model: &Model) -> Option<bool>;
    fn symbols(&self) -> Vec<&Symbol>;
    fn formula(&self) -> String;
    fn symbols_unique(&self) -> Vec<&Symbol> {
        crate::utils::unique(self.symbols())
    }
}

impl std::fmt::Display for dyn Sentence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formula())
    }
}

// Model
#[derive(Clone)]
pub struct Model {
    symbols: std::collections::BTreeMap<Symbol, bool>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            symbols: std::collections::BTreeMap::new(),
        }
    }

    pub fn set(&mut self, symbol: &Symbol, value: bool) {
        self.symbols.insert(symbol.clone(), value);
    }

    pub fn get(&self, symbol: &Symbol) -> Option<&bool> {
        self.symbols.get(&symbol)
    }

    pub fn eval<S: Sentence>(&self, sentence: &S) -> Option<bool> {
        sentence.eval(&self)
    }

    #[track_caller]
    pub fn assert_true<S: Sentence>(&self, sentence: &S) {
        assert_eq!(Some(true), self.eval(sentence))
    }

    #[track_caller]
    pub fn assert_false<S: Sentence>(&self, sentence: &S) {
        assert_eq!(Some(false), self.eval(sentence))
    }
}

#[macro_export]
macro_rules! model {
    () => { Model::new() };
    ($($e:expr => $es:expr),* $(,)?) => {
            {
                let mut model = Model::new();
                $(model.set(&$e, $es);)
                *
                model
            }
    };
}
pub use model;

// Symbol
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Symbol {
    label: Rc<String>,
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formula())
    }
}

impl Symbol {
    pub fn new<S: Into<String>>(label: S) -> Self {
        Self {
            label: Rc::new(label.into()),
        }
    }

    pub fn name(&self) -> &str {
        &self.label
    }
}

pub fn Symbol<S: Into<String>>(label: S) -> Symbol {
    Symbol::new(label)
}

#[macro_export]
macro_rules! symbol {
    ($e:expr) => {
        Symbol($e)
    };
}
pub use symbol;

impl Sentence for Symbol {
    fn eval(&self, model: &Model) -> Option<bool> {
        return model.get(&self).copied();
    }

    fn symbols(&self) -> Vec<&Symbol> {
        vec![&self]
    }
    
    fn formula(&self) -> String {
        self.name().to_owned()
    }
}

fn model_check<K, Q>(knoledge: &K, query: &Q, symbols: &Vec<&Symbol>, model: Model) -> Option<bool>
where
    K: Sentence + 'static,
    Q: Sentence + 'static,
{
    if symbols.is_empty() {
        let Some(knoledge) = knoledge.eval(&model) else {
            return None;
        };

        if knoledge {
            return query.eval(&model);
        }
        Some(true)
    } else {
        // Choose one of the remaining unused symbols
        let mut remaining = symbols.clone();
        let p = remaining.pop().expect("Not empty");

        // Create a model where the symbol is true
        let mut model_true = model.clone();
        model_true.set(p, true);

        // Create a model where the symbol is false
        let mut model_false = model.clone();
        model_false.set(p, false);

        // Ensure entailment holds in both models

        let Some(left) = model_check(knoledge, query, &remaining, model_true) else {
            return None;
        };

        let Some(right) = model_check(knoledge, query, &remaining, model_false) else {
            return None;
        };

        Some(left && right)
    }
}

pub fn check<K, Q>(knoledge: &K, query: &Q) -> Option<bool>
where
    K: Sentence + 'static,
    Q: Sentence + 'static,
{
    let mut symbols = knoledge.symbols();
    symbols.append(&mut query.symbols());
    symbols = crate::utils::unique(symbols);
    model_check(knoledge, query, &symbols, Model::new())
}
