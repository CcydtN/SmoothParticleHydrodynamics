from sympy import *
import json

def get_derivatives(func, variable):
    f = simplify(func)
    g = simplify(diff(func,variable))
    l = simplify(diff(diff(func, variable), variable))
    return f,g,l

def custom_range(start, end, step_count):
    step = (end - start) / step_count
    for i in range(step_count):
        yield i*step + start

def evaluate(func, variable, start, end, count):
    ret = []
    for i in custom_range(start, end, count):
        tmp = func.subs(variable, i)
        try:
            ret.append((i, float(tmp)))
        except:
            continue
    return ret


# Always give 10 sample between [0, end], unless the result is inf of NaN
def sampling(func, grad, lapl, variable, end, h, file_name = None):
    sampling_count = 10

    f = evaluate(func, variable, 0, end, sampling_count)
    g = evaluate(grad, variable, 0, end, sampling_count)
    l = evaluate(lapl, variable, 0, end, sampling_count)

    sample = {
        "h_value": h,
        "function": f,
        "gradient": g,
        "laplacian": l,
    }

    if file_name is not None:
        with open(file_name, 'w') as f:
            json.dump(sample, f)

    return sample