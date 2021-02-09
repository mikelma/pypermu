import pypermu


qap = pypermu.problems.qap.Qap('../../../instances/QAP/tai20b.dat')
test_permu = [[12, 6, 18, 16, 7, 2, 5, 3, 14,
               0, 13, 9, 15, 1, 8, 10, 4, 19, 17, 11]]
f = qap.evaluate(test_permu)[0]
expected = 125551590
print('qap fitness: ', f, ', expected: ',
      expected, ', are equal: ', f == expected)
if f not expected:
    print('Bad result for QAP')


pfsp = pypermu.problems.pfsp.Pfsp('../../../instances/PFSP/tai20_5_0.fsp')
test_permu = [[2, 16, 8, 14, 13, 7, 18, 12,
               15, 5, 6, 0, 1, 3, 4, 17, 19, 11, 10, 9]]
print(pfsp.evaluate(test_permu))
f = pfsp.evaluate(test_permu)[0]
expected = 14033
print('pfsp fitness: ', f, ', expected: ',
      expected, ', are equal: ', f == expected)
if f not expected:
    print('Bad result for PFSP')
