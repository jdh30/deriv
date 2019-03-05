open Printf

type expr =
  | Int of int
  | Var of string
  | Add of expr * expr
  | Mul of expr * expr
  | Pow of expr * expr
  | Ln of expr

let rec pown a = function
  | 0 -> 1
  | 1 -> a
  | n ->
    let b = pown a (n / 2) in
    b * b * (if n mod 2 = 0 then 1 else a)

let rec add = function
  | Int m, Int n -> Int(m + n)
  | Int 0, f | f, Int 0 -> f
  | f, Int n -> add(Int n, f)
  | f, Add(Int n, g) -> add(Int n, add(f, g))
  | Add(f, g), h -> add(f, add(g, h))
  | f, g -> Add(f, g)
let rec mul = function
  | Int m, Int n -> Int(m * n)
  | Int 0, f | f, Int 0 -> Int 0
  | Int 1, f | f, Int 1 -> f
  | f, Int n -> mul(Int n, f)
  | f, Mul(Int n, g) -> mul(Int n, mul(f, g))
  | Mul(f, g), h -> mul(f, mul(g, h))
  | f, g -> Mul(f, g)
let rec pow = function
  | Int m, Int n -> Int(pown m n)
  | f, Int 0 -> Int 1
  | f, Int 1 -> f
  | Int 0, f -> Int 0
  | f, g -> Pow(f, g)
let ln = function
  | Int 1 -> Int 0
  | f -> Ln f

let rec d x = function
  | Var y when x=y -> Int 1
  | Int _ | Var _ -> Int 0
  | Add(f, g) -> add(d x f, d x g)
  | Mul(f, g) -> add(mul(f, d x g), mul(g, d x f))
  | Pow(f, g) -> mul(pow(f, g), add(mul(mul(g, d x f), pow(f, Int(-1))), mul(ln f, d x g)))
  | Ln f -> mul(d x f, pow(f, Int(-1)))

let rec count = function
  | Int _ | Var _ -> 1
  | Add(f, g) | Mul(f, g) | Pow(f, g) -> count f + count g
  | Ln f -> count f

let rec string_of () = function
  | Int n -> sprintf "%d" n
  | Var x -> x
  | Add(f, g) -> sprintf "%a + %a" string_of f string_of g
  | Mul(f, g) -> sprintf "%a*%a" (bracket 2) f (bracket 2) g
  | Pow(f, g) -> sprintf "%a^%a" (bracket 2) f (bracket 3) g
  | Ln f -> sprintf "ln(%a)" string_of f
and prec = function
  | Int _ | Var _ | Ln _ -> 4
  | Pow _ -> 3
  | Mul _ -> 2
  | Add _ -> 1
and bracket outer () expr =
  if outer > prec expr then
    sprintf "(%a)" string_of expr
  else
    string_of () expr

let string_of_expr expr =
  let n = count expr in
  if n > 100 then sprintf "<<%d>>" n else
    sprintf "%a" string_of expr

let rec nest n f x =
  if n = 0 then x else
    nest (n-1) f (f x)

let deriv f =
  let d = d "x" f in
  printf "D(%s) = %s\n%!" (string_of_expr f) (string_of_expr d);
  d

let () =
  let x = Var "x" in
  let f = pow(x, x) in
  ignore(nest (int_of_string Sys.argv.(1)) deriv f)
