enum Expr {
  case Int(n: Int)
  case Var(x: String)
  indirect case Add(f: Expr, g: Expr)
  indirect case Mul(f: Expr, g: Expr)
  indirect case Pow(f: Expr, g: Expr)
  indirect case Ln(f: Expr)
}

func pown(a: Int, b: Int) -> Int {
  switch (b) {
    case 0 : return 1
    case 1 : return a
    case let n :
      let b = pown(a: a, b: n / 2)
      return b * b * (n % 2 == 0 ? 1 : a)
  }
}

func add(f: Expr, g: Expr) -> Expr {
  switch (f, g) {
    case (let .Int(n: m), let .Int(n: n)) : return .Int(n: m + n)
    case (.Int(n: 0), let f) : return f
    case (let f, .Int(n: 0)) : return f
    case (let f, let .Int(n)) : return add(f: .Int(n: n), g: f)
    case (let f, let .Add(.Int(n), g)) : return add(f: .Int(n: n), g: add(f: f, g: g))
    case (let .Add(f, g), let h) : return add(f: f, g: add(f: g, g: h))
    case (let f, let g) : return .Add(f: f, g: g)
  }
}

func mul(f: Expr, g: Expr) -> Expr {
  switch (f, g) {
    case (let .Int(n: m), let .Int(n: n)) : return .Int(n: m * n)
    case (.Int(n: 0), _) : return .Int(n: 0)
    case (_, .Int(n: 0)) : return .Int(n: 0)
    case (.Int(n: 1), let f) : return f
    case (let f, .Int(n: 1)) : return f
    case (let f, let .Int(n: n)) : return mul(f: .Int(n: n), g: f)
    case (let f, let .Mul(.Int(n: n), g)) : return mul(f: .Int(n: n), g: mul(f: f, g: g))
    case (let .Mul(f: f, g: g), let h) : return mul(f: f, g: mul(f: g, g: h))
    case (let f, let g) : return .Mul(f: f, g: g)
  }
}

func pow(f: Expr, g: Expr) -> Expr {
  switch (f, g) {
    case (let .Int(n: m), let .Int(n: n)) : return .Int(n: pown(a: m, b: n))
    case (_, .Int(n: 0)) : return .Int(n: 1)
    case (let f, .Int(n: 1)) : return f
    case (.Int(n: 0), _) : return .Int(n: 0)
    case (let f, let g) : return .Pow(f: f, g: g)
  }
}

func ln(f: Expr) -> Expr {
  switch (f) {
    case .Int(n: 1) : return .Int(n: 0)
    case let f : return .Ln(f: f)
  }
}

func d(x: String, f: Expr) -> Expr {
  switch (f) {
    case .Int(n: _) : return .Int(n: 0)
    case let .Var(x: y) : if x == y { return .Int(n: 1) } else { return .Int(n: 0) }
    case let .Add(f: f, g: g) : return add(f: d(x: x, f: f), g: d(x: x, f: g))
    case let .Mul(f: f, g: g) : return add(f: mul(f: f, g: d(x: x, f: g)), g: mul(f: g, g: d(x: x, f: f)))
    case let .Pow(f: f, g: g) : return mul(f: pow(f: f, g: g), g: add(f: mul(f: mul(f: g, g: d(x: x, f: f)), g: pow(f: f, g: .Int(n: -1))), g: mul(f: ln(f: f), g: d(x: x, f: g))))
    case let .Ln(f: f) : return mul(f: d(x: x, f: f), g: pow(f: f, g: .Int(n: -1)))
  }
}

func count(f: Expr) -> Int {
  switch (f) {
    case .Int(n: _) : return 1
    case .Var(x: _) : return 1
    case let .Add(f: f, g: g) : return count(f: f) + count(f: g)
    case let .Mul(f: f, g: g) : return count(f: f) + count(f: g)
    case let .Pow(f: f, g: g) : return count(f: f) + count(f: g)
    case let .Ln(f: f) : return count(f: f)
  }
}

func stringOfExpr(f: Expr) -> String {
  switch (f) {
    case let .Int(n: n) : return String(n)
    case let .Var(x: x) : return x
    case let .Add(f: f, g: g) : return "(" + stringOfExpr(f: f) + " + " + stringOfExpr(f: g) + ")"
    case let .Mul(f: f, g: g) : return "(" + stringOfExpr(f: f) + " * " + stringOfExpr(f: g) + ")"
    case let .Pow(f: f, g: g) : return "(" + stringOfExpr(f: f) + "^" + stringOfExpr(f: g) + ")"
    case let .Ln(f: f) : return "ln(" + stringOfExpr(f: f) + ")"
  }
}

func stringOf(f: Expr) -> String {
  let n = count(f: f)
  if n > 100 {
    return "<<" + String(n) + ">>"
  } else {
    return stringOfExpr(f: f)
  }
}

let x = Expr.Var(x: "x")

let f = pow(f: x, g: x)

func nest(n: Int, f: ((Expr) -> Expr), x: Expr) -> Expr {
  if n == 0 { return x } else {
    return nest(n: n-1, f: f, x: f(x))
  }
}

var dx = { (f: Expr) -> Expr in
  var df = d(x: "x", f: f)
  print("D(" + stringOf(f: f) + ") = " + stringOf(f: df))
  return df
}

var n = Int(CommandLine.arguments[1])

print(count(f: nest(n: n!, f: dx, x: f)))
