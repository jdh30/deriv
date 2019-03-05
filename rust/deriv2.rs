use std::env;
use std::fmt;
use Expr::*;

struct Arena<'a>(typed_arena::Arena<Expr<'a>>);

enum Expr<'a> {
    Int(i64),
    Var(&'a str),
    Add(&'a Expr<'a>, &'a Expr<'a>),
    Mul(&'a Expr<'a>, &'a Expr<'a>),
    Pow(&'a Expr<'a>, &'a Expr<'a>),
    Ln(&'a Expr<'a>),
}
static MINUS_ONE: Expr = Expr::Int(-1);
static ZERO: Expr = Expr::Int(0);
static ONE: Expr = Expr::Int(1);

fn pown(base: i64, expt: i64) -> i64 {
    match expt {
        0 => 1,
        1 => base,
        _ => {
            let b = pown(base, expt / 2);
            b * b * if expt % 2 == 0 { 1 } else { base }
        }
    }
}

impl<'a> Arena<'a> {
    fn new() -> Self {
        Arena(typed_arena::Arena::new())
    }

    fn decompose_add(&'a self, expr: &'a Expr) -> (i64, Option<&'a Expr>) {
        match expr {
            Expr::Int(n) => (*n, None),
            Expr::Add(f, g) => match **f {
                Expr::Int(n) => (n, Some(g)),
                _ => (0, Some(expr)),
            },
            _ => (0, Some(expr)),
        }
    }

    fn decompose_mul(&'a self, expr: &'a Expr) -> (i64, Option<&'a Expr>) {
        match expr {
            Expr::Int(n) => (*n, None),
            Expr::Mul(f, g) => match **f {
                Expr::Int(n) => (n, Some(g)),
                _ => (1, Some(expr)),
            },
            _ => (1, Some(expr)),
        }
    }

    fn add(&'a self, lhs: &'a Expr, rhs: &'a Expr) -> &'a Expr {
        let (m, fo) = self.decompose_add(&lhs);
        let (n, go) = self.decompose_add(&rhs);

        let mn = m + n;

        let fg = match (fo, go) {
            (None, None) => return self.0.alloc(Int(mn)),
            (Some(f), None) => f,
            (None, Some(g)) => g,
            (Some(f), Some(g)) => self.0.alloc(Add(f, g)),
        };

        if mn == 0 {
            fg
        } else {
            self.0.alloc(Add(self.0.alloc(Int(mn)), fg))
        }
    }

    fn mul(&'a self, lhs: &'a Expr, rhs: &'a Expr) -> &'a Expr {
        let (m, fo) = self.decompose_mul(lhs);
        let (n, go) = self.decompose_mul(rhs);

        let mn = m * n;

        if mn == 0 {
            return &ZERO;
        }

        let fg = match (fo, go) {
            (None, None) => return self.0.alloc(Int(mn)),
            (Some(f), None) => f,
            (None, Some(g)) => g,
            (Some(f), Some(g)) => self.0.alloc(Mul(f, g)),
        };

        if mn == 1 {
            fg
        } else {
            self.0.alloc(Mul(self.0.alloc(Int(mn)), fg))
        }
    }

    fn pow(&'a self, lhs: &'a Expr, rhs: &'a Expr) -> &'a Expr {
        match (lhs, rhs) {
            (Int(m), Int(n)) => self.0.alloc(Int(pown(*m, *n))),
            (_, Int(0)) => &ONE,
            (_, Int(1)) => lhs,
            (Int(0), _) => &ZERO,
            _ => self.0.alloc(Pow(lhs, rhs)),
        }
    }

    fn ln(&'a self, expr: &'a Expr) -> &'a Expr {
        match expr {
            Int(1) => &ZERO,
            _ => self.0.alloc(Ln(expr)),
        }
    }

    fn d(&'a self, expr: &'a Expr, x: &str) -> &'a Expr {
        match expr {
            Var(y) => {
                if *x == **y {
                    &ONE
                } else {
                    &ZERO
                }
            }
            Int(_) => &ZERO,
            Add(f, g) => self.add(self.d(f, x), self.d(g, x)),
            Mul(f, g) => self.add(self.mul(f, self.d(g, x)), self.mul(g, self.d(f, x))),
            Pow(f, g) => self.mul(
                expr,
                self.add(
                    self.mul(self.mul(g, self.d(f, x)), self.pow(f, &MINUS_ONE)),
                    self.mul(self.ln(f), self.d(g, x)),
                ),
            ),
            Ln(f) => self.mul(self.d(f, x), self.pow(f, &MINUS_ONE)),
        }
    }
}

impl<'a> Expr<'a> {
    fn count(&self) -> usize {
        match &*self {
            Var(_) => 1,
            Int(_) => 1,
            Add(f, g) => f.count() + g.count(),
            Mul(f, g) => f.count() + g.count(),
            Pow(f, g) => f.count() + g.count(),
            Ln(f) => f.count(),
        }
    }

    fn format(&self, f: &mut fmt::Formatter, old_prec: usize) -> fmt::Result {
        fn bracket<F>(
            f: &mut fmt::Formatter,
            old_prec: usize,
            new_prec: usize,
            body: F,
        ) -> fmt::Result
        where
            F: FnOnce(&mut fmt::Formatter) -> fmt::Result,
        {
            if new_prec < old_prec {
                f.write_str("(")?;
            }
            body(f)?;
            if new_prec < old_prec {
                f.write_str(")")?;
            }
            Ok(())
        }

        match self {
            Var(name) => f.write_str(&**name),

            Int(n) => write!(f, "{}", n),

            Add(g, h) => bracket(f, old_prec, 1, |f| {
                g.format(f, 1)?;
                f.write_str(" + ")?;
                h.format(f, 1)
            }),

            Mul(g, h) => bracket(f, old_prec, 2, |f| {
                g.format(f, 2)?;
                f.write_str("*")?;
                h.format(f, 2)
            }),

            Pow(g, h) => bracket(f, old_prec, 3, |f| {
                g.format(f, 2)?;
                f.write_str("^")?;
                h.format(f, 3)
            }),

            Ln(g) => {
                f.write_str("ln(")?;
                g.format(f, 1)?;
                f.write_str(")")
            }
        }
    }
}

impl<'a> fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = self.count();
        if n > 100 {
            write!(f, "<<{}>>", n)
        } else {
            self.format(f, 1)
        }
    }
}

fn main() {
    let arena = Arena::new();
    let x = &Var("x");
    let mut f = arena.pow(&x, &x);
    let n = env::args().nth(1).expect("n").parse().unwrap();
    for _ in 0..n {
        let d = arena.d(f, "x");
        println!("D({}) = {}", f, d);
        f = d;
    }
}
