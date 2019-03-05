use std::env;
use std::fmt;
use std::ops::{Add, Mul};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Expr {
    Int(i64),
    Var(Rc<str>),
    Add(Rc<Expr>, Rc<Expr>),
    Mul(Rc<Expr>, Rc<Expr>),
    Pow(Rc<Expr>, Rc<Expr>),
    Ln(Rc<Expr>),
}

fn pown(base: i64, expt: i64) -> i64 {
    match expt {
        0 => 1,
        1 => base,
        _ => {
            let b = pown(base, expt / 2);
            b * b * if expt % 2 == 0 {1} else {base}
        }
    }
}

fn decompose_add(expr: Expr) -> (i64, Option<Expr>) {
    match expr {
        Expr::Int(n) => (n, None),
        Expr::Add(f, g) => {
            match *f {
                Expr::Int(n) => (n, Some(g.clone_expr())),
                _ => (0, Some(Expr::Add(f, g)))
            }
        }
        _ => (0, Some(expr)),
    }
}

fn decompose_mul(expr: Expr) -> (i64, Option<Expr>) {
    match expr {
        Expr::Int(n) => (n, None),
        Expr::Mul(f, g) => {
            match *f {
                Expr::Int(n) => (n, Some(g.clone_expr())),
                _ => (1, Some(Expr::Mul(f, g))),
            }
        }
        _ => (1, Some(expr)),
    }
}

impl Expr {
    fn clone_expr(&self) -> Self {
        Expr::clone(self)
    }
}

impl Add<Expr> for Expr {
    type Output = Expr;

    fn add(self, other: Expr) -> Expr {
        use Expr::*;

        let (m, fo) = decompose_add(self);
        let (n, go) = decompose_add(other);

        let mn = m + n;

        let fg = match (fo, go) {
            (None, None) => return Int(mn),
            (Some(f), None) => f,
            (None, Some(g)) => g,
            (Some(f), Some(g)) => Add(Rc::new(f), Rc::new(g)),
        };

        if mn == 0 {
            fg
        } else {
            Add(Rc::new(Int(mn)), Rc::new(fg))
        }
    }
}

impl Mul<Expr> for Expr {
    type Output = Expr;

    fn mul(self, other: Expr) -> Expr {
        use Expr::*;

        let (m, fo) = decompose_mul(self);
        let (n, go) = decompose_mul(other);

        let mn = m * n;

        if mn == 0 {
            return Int(0);
        }

        let fg = match (fo, go) {
            (None, None) => return Int(mn),
            (Some(f), None) => f,
            (None, Some(g)) => g,
            (Some(f), Some(g)) => Mul(Rc::new(f), Rc::new(g)),
        };

        if mn == 1 {
            fg
        } else {
            Mul(Rc::new(Int(mn)), Rc::new(fg))
        }
    }
}

impl Expr {
    pub fn pow(self, other: Expr) -> Expr {
        use Expr::*;

        match (self, other) {
            (Int(m), Int(n)) => Int(pown(m, n)),
            (_, Int(0)) => Int(1),
            (a, Int(1)) => a,
            (Int(0), _) => Int(0),
            (a, b) => Pow(Rc::new(a), Rc::new(b)),
        }
    }

    pub fn ln(self) -> Expr {
        use Expr::*;

        match self {
            Int(1) => Int(0),
            a => Ln(Rc::new(a)),
        }
    }

    pub fn d(&self, x: &str) -> Expr {
        use Expr::*;

        match *self {
            Var(ref y) =>
                if *x == **y {
                    Int(1)
                } else {
                    Int(0)
                },
            Int(_) => Int(0),
            Add(ref f, ref g) => f.d(x) + g.d(x),
            Mul(ref f, ref g) =>
                f.clone_expr() * g.d(x) + g.clone_expr() * f.d(x),
            Pow(ref f, ref g) =>
                self.clone_expr() *
                    (g.clone_expr() * f.d(x) * f.clone_expr().pow(Int(-1)) +
                     f.clone_expr().ln() * g.d(x)),
            Ln(ref f) =>
                f.d(x) * f.clone_expr().pow(Int(-1)),
        }
    }

    pub fn count(&self) -> usize {
        use Expr::*;

        match *self {
            Var(_) => 1,
            Int(_) => 1,
            Add(ref f, ref g) => f.count() + g.count(),
            Mul(ref f, ref g) => f.count() + g.count(),
            Pow(ref f, ref g) => f.count() + g.count(),
            Ln(ref f) => f.count(),
        }
    }

    fn format(&self, f: &mut fmt::Formatter, old_prec: usize) -> fmt::Result {
        use Expr::*;

        fn bracket<F>(f: &mut fmt::Formatter,
                      old_prec: usize, new_prec: usize, body: F)
            -> fmt::Result
            where F: FnOnce(&mut fmt::Formatter) -> fmt::Result {

            if new_prec < old_prec { f.write_str("(")?; }
            body(f)?;
            if new_prec < old_prec { f.write_str(")")?; }
            Ok(())
        }

        match *self {
            Var(ref name) => f.write_str(&**name),

            Int(n) => write!(f, "{}", n),

            Add(ref g, ref h) => bracket(f, old_prec, 1, |f| {
                g.format(f, 1)?;
                f.write_str(" + ")?;
                h.format(f, 1)
            }),

            Mul(ref g, ref h) => bracket(f, old_prec, 2, |f| {
                g.format(f, 2)?;
                f.write_str("*")?;
                h.format(f, 2)
            }),

            Pow(ref g, ref h) => bracket(f, old_prec, 3, |f| {
                g.format(f, 2)?;
                f.write_str("^")?;
                h.format(f, 3)
            }),

            Ln(ref g) => {
                f.write_str("ln(")?;
                g.format(f, 1)?;
                f.write_str(")")
            }
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = self.count();
        if n > 100 {
            write!(f, "<<{}>>", n)
        } else {
            self.format(f, 1)
        }
    }
}

fn nest<A, F: Fn(&A) -> A>(n: usize, f: F, mut x: A) -> A {
    for _ in 0 .. n {
        x = f(&x);
    }

    x
}

fn deriv(f: &Expr) -> Expr {
    let d = f.d("x");
    println!("D({}) = {}", f, d);
    d
}

fn main() {
    let x = Expr::Var(Rc::from("x"));
    let f = x.clone().pow(x);
    let n = env::args().nth(1).expect("n").parse().unwrap();
    nest(n, deriv, f);
}
