import flickers as fl

ps = fl.test_suite.phases()

r = fl.dev.compute(ps)

print(r.taus)