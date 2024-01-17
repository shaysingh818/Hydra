import numpy as np

a = np.array([
	[0, 0, 1],
	[0, 1, 2],
	[1, 1, 3],
	[0, 0, 4]
])

b = np.array([
	[1,1,1],
	[2,2,2],
	[3,3,3]])

c = np.dot(a, b)
print(c)

"""
[[0 0 0]
 [2 2 2]
 [3 3 3]
 [0 0 0]]

"""
