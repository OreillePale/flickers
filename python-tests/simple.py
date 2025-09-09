import flickers as fl
import matplotlib.pyplot as plt
import allantools as at
import numpy as np
import time

phases = fl.test_suite.phases()

t10 = time.time()
r = fl.dev.compute(phases)
t11 = time.time()
print(t11-t10)

# print(r.devs)

t10 = time.time()
(t2,d2,_,_) = at.oadev(phases, taus='all')
t11 = time.time()
print(t11-t10)

plt.figure()
plt.loglog()
plt.grid()
plt.plot(t2,d2,label='allantools')
plt.plot(r['taus'],r['devs'], label='flickers', color='red', ls=':')
plt.legend()

for k in r:
    print(k)

print(r['noise_id'])
print(r['dev'])

plt.figure()
plt.plot(r['taus'],r['alphas'])

plt.show()