# Smooth Particle Hydrodynamics

> WIP
> 
> still working on it

A repo to store my learning/implementation of SPH.

## What is Smooth Particle Hydrodynamics?
[wiki](https://en.wikipedia.org/wiki/Smoothed-particle_hydrodynamics)

## Todo List
- [X] Understanding the basic [ref](https://matthias-research.github.io/pages/publications/sca03.pdf)
  - 2D simulation to play around
- [X] Replicate the result of of [this paper](http://cg.informatik.uni-freiburg.de/publications/2007_SCA_SPH.pdf)
  - 3D simultion of a drop in zero-gravity scenario
- [X] Dynamic kernel radius implementation
  - "Resolution varying in space and time" (Page 1722) @ [here](https://sci-hub.se/https://iopscience.iop.org/article/10.1088/0034-4885/68/8/R01)
  - [ ] Way too slow, (probably bad neighbour seach impl, and some others factor)
    - [ ] Spatial hash grid
    - [ ] ~kd-tree~ [Reason](https://arxiv.org/pdf/1309.3783) (3 reason is suggested at "II. Algorithm")
    - [ ] Get rid of the dynamic kernel
    - [ ] Others... (doing research...)
- [ ] Find and fix performance issue
  - should be done after every milestone
  - Evaluate with perf, and flamegraph
    - [X] Cargo profile
    - [X] Command expample (done: at the "justfile")
- [ ] Boundary condition
  - [ ] Simple: when ever a particle touch a surface, move its' location to the boundary and reflect the velocity by the normal.
  - [ ] Complex: Boundary particle.
- [ ] Heat conduction
  - Maybe drop hot water on ice
  - Or ice drop in hot water
- [ ] Parallel computation
  - Probably with rayon


## Reference
**Paper**
https://matthias-research.github.io/pages/publications/sca03.pdf

http://cg.informatik.uni-freiburg.de/publications/2007_SCA_SPH.pdf

https://doi.org/10.1006/jcph.1994.1034

http://www.ligum.umontreal.ca/Clavet-2005-PVFS/pvfs.pdf

https://sph-tutorial.physics-simulation.org/pdf/SPH_Tutorial.pdf

https://www.jstage.jst.go.jp/article/qjjws/33/2/33_34s/_pdf/-char/ja

https://www.jstage.jst.go.jp/article/qjjws/38/2/38_84s/_pdf/-char/en

https://arxiv.org/pdf/1608.04400.pdf

**Youtube**

https://www.youtube.com/watch?v=rSKMYc1CQHE


https://www.youtube.com/watch?v=-0m05gzk8nk

