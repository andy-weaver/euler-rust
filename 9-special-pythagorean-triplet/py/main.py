import sympy as sp
from sympy.solvers.diophantine import diophantine

x, a, b, c = sp.symbols("x a b c")
f = sp.Lambda(x, x**2)
""" c = sp.Lambda((a, b), 1000 - a - b)
eq1 = sp.Eq(f(a) + f(b), f(c(a, b)))

constraint1 = sp.Lt(a, b)
constraint2 = sp.Lt(b, c(a, b))
constraint3 = sp.Gt(a, 0)

soln = diophantine([eq1, constraint1, constraint2, constraint3])

print(soln) """
