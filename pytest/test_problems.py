import pypermu
import numpy as np

################# QAP ################# 
qap = pypermu.problems.qap.Qap('instances/tai20b.dat')
test_permu = [[12, 6, 18, 16, 7, 2, 5, 3, 14,
               0, 13, 9, 15, 1, 8, 10, 4, 19, 17, 11]]
f = qap.evaluate(test_permu)[0]
expected = 125551590

print('qap fitness: ', f, ', expected: ',
      expected, ', are equal: ', f == expected)
if f != expected:
    print('Bad result for QAP')

print()

################# PFSP ################# 
pfsp = pypermu.problems.pfsp.Pfsp('instances/tai20_5_0.fsp')
test_permu = [[2, 16, 8, 14, 13, 7, 18, 12,
               15, 5, 6, 0, 1, 3, 4, 17, 19, 11, 10, 9]]

f = pfsp.evaluate(test_permu)[0]
expected = 14033

print('pfsp fitness: ', f, ', expected: ',
      expected, ', are equal: ', f == expected)
if f != expected:
    print('Bad result for PFSP')

print()

################# LOP ################# 
lop = pypermu.problems.lop.Lop('instances/N-p40-01')
# TODO: Use a known permutation-fitness pair!
test_permu = [[38, 37,  3, 36, 24, 23, 39, 22, 20,  4, 35, 15, 21,  5,  2, 16,  7,
               33, 19, 31, 30, 14, 25, 32, 27,  8, 12, 29, 13,  6, 17, 18, 34, 10,
               9, 26, 28,  1,  0, 11]]
# test_permu = [np.random.permutation(40)]
f = lop.evaluate(test_permu)[0]
expected = 0 

print('lop fitness: ', f, ', expected: ',
      expected, ', are equal: ', f == expected)
if f != expected:
    print('Bad result for LOP')
