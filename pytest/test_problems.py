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
lop = pypermu.problems.lop.Lop('instances/N-be75eec')
test_permu = [[ 41, 48, 32, 2, 47, 36, 22, 24, 37, 33, 21, 49, 27, 23, 18, 4, 10, 44, 40, 6, 26, 19, 14, 31, 25, 46, 17, 20, 15, 38, 1, 16, 9, 13, 42, 7, 35, 45, 8, 28, 0, 43, 39, 29, 30, 11, 12, 34, 3, 5]]
# test_permu = [np.random.permutation(40)]
f = lop.evaluate(test_permu)[0]
expected = 231707 

print('lop fitness: ', f, ', expected: ',
      expected, ', are equal: ', f == expected)
if f != expected:
    print('Bad result for LOP')
