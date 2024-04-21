# Equation for kernel function
Notebook to help finding equation and constant from kernel.

## Background
For SPH, one of the key concept is kernel function (or abbreviated as a kernel).

$$W(r_a - r_b, h)$$

In simulation, we need a few more equation that is comes from the kernel, like,

Gradient: 
$$\nabla W(r_a - r_b,h)$$

Laplacian: 
$$\nabla^2 W(r_a - r_b,h)$$

1-D Constant
$$\int_{-h}^h W(\vec{r}, h)\cdot d\vec{r} = 1$$
2-D Constant
$$\int_0^h W(\vec{r}, h)\cdot 2\pi r \cdot dr = 1$$
> $$\int W(\vec{r}, h)\cdot dA = 1$$
> $$A = \pi r^2$$
> $$dA/dr = 2\pi\cdot r$$
3-D Constant
$$\int_0^h W(\vec{r}, h)\cdot 4\pi r^2 dr = 1$$
> $$\int W(\vec{r}, h)\cdot dV = 1$$
> $$V = 4/3\pi r^3$$
> $$dV/dr = 4\pi r^2$$