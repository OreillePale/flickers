import flickers as fl
import matplotlib.pyplot as plt
import allantools as at
import numpy as np

phases = fl.test_suite.phases()

r = fl.dev.compute(phases, tau0=1., dev_type=fl.DevType.Oadev, afs=fl.Afs.Explicit([1,10]))
errs = r.errs()
print(r.taus)

# print(r.devs)

(t2,d2,_,_) = at.adev(phases, taus='octave')

plt.figure()
plt.loglog()
plt.grid()
plt.scatter(t2,d2,label='allantools')
plt.errorbar(r.taus,r.devs,np.array(r.errs()).T, label='flickers', fmt='+-', color='red', capsize=3)
plt.legend()

plt.show()