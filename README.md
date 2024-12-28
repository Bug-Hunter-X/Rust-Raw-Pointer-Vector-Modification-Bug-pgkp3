# Rust Raw Pointer Vector Modification Bug

This repository demonstrates a common error in Rust involving raw pointers and vector manipulation.  Modifying a vector through a raw pointer after the vector's ownership changes leads to undefined behavior and potential crashes. The bug.rs file contains the buggy code, and bugSolution.rs provides a corrected version.