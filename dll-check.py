"""
1. Prepare DLL:
    cargo build --lib --release
2. Run check:
    python dll-check.py

DLL path: target/release/libclique.so
"""


import os
import ctypes
from ctypes import *


if __name__ == "__main__":
    dll_path = 'target/release/libclique.so'
    dll = CDLL(os.path.abspath(dll_path))

    graph = [0, 3, 0, 4, 0, 7, 1, 2, 1, 4, 1, 5, 1, 7, 2, 3, 2, 8, 3, 5, 3, 8, 4, 5, 4, 6, 4, 7, 4, 8, 5, 7, 6, 7, 7, 8]
    clique = [0] * 9

    dll.solve_clique_wrap.argtypes = (c_uint32, POINTER(c_uint32), POINTER(c_uint32))
    dll.solve_clique_wrap.restype = c_uint32

    graph_c = (c_uint32 * len(graph))(*graph)
    clique_c = (c_uint32 * len(clique))(*clique)

    clique_size_c = dll.solve_clique_wrap(
        c_uint32(len(graph)),
        graph_c,
        clique_c
    )

    print(clique_size_c)
    print(list(clique_c)[:clique_size_c])
