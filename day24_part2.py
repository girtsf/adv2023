#!/usr/bin/env python3

import sympy

r_0x, r_0y, r_0z = sympy.symbols(['r_0x', 'r_0y', 'r_0z'])
r_vx, r_vy, r_vz = sympy.symbols(['r_vx', 'r_vy', 'r_vz'])
t0, t1, t2 = sympy.symbols(['t0', 't1', 't2'])

h0_0x, h0_0y, h0_0z, h0_vx, h0_vy, h0_vz = 251454256616722, 382438634889004, 18645302082228, 43, -207, 371
h1_0x, h1_0y, h1_0z, h1_vx, h1_vy, h1_vz = 289124150762025, 364325878532733, 278169080781801, -73, -158, -13
h2_0x, h2_0y, h2_0z, h2_vx, h2_vy, h2_vz = 268852221227649, 10710819924145, 258969710792682, 41, 192, 62

sol = sympy.solve([
r_0x + r_vx * t0 - (h0_0x + (h0_vx) * t0),
r_0y + r_vy * t0 - (h0_0y + (h0_vy) * t0),
r_0z + r_vz * t0 - (h0_0z + (h0_vz) * t0),

r_0x + r_vx * t1 - (h1_0x + (h1_vx) * t1),
r_0y + r_vy * t1 - (h1_0y + (h1_vy) * t1),
r_0z + r_vz * t1 - (h1_0z + (h1_vz) * t1),

r_0x + r_vx * t2 - (h2_0x + (h2_vx) * t2),
r_0y + r_vy * t2 - (h2_0y + (h2_vy) * t2),
r_0z + r_vz * t2 - (h2_0z + (h2_vz) * t2),
])
print(sol)
print(sol[0][r_0x] + sol[0][r_0y] + sol[0][r_0z])
