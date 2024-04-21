from sympy import *
import json

def get_derivatives(func, variable):
    f = simplify(func)
    g = simplify(diff(func,variable))
    l = simplify(diff(diff(func, variable), variable))
    return f,g,l

# Always start with 0, step 0.2, inclusive end
def sampling(func, grad, lapl, variable, inclusive_end, h, file_name = None):
    arange = lambda start, stop, step: [step * i for i in range(int((stop - start) / step))]

    sample = {
        "h_value": h,
        "function": [(i, float(func.subs([(variable, i)]))) for i in arange(0, inclusive_end, 0.2)],
        "gradient": [(i, float(grad.subs([(variable, i)]))) for i in arange(0, inclusive_end, 0.2)],
        "laplacian": [(i, float(lapl.subs([(variable, i)]))) for i in arange(0, inclusive_end, 0.2)],
    }

    if file_name is not None:
        with open(file_name, 'w') as f:
            json.dump(sample, f)

    return sample