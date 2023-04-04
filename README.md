# Xirtam: ICCS311 Functional and Parallel Programming term project
**by Nattamon Santrakul & Krittin Pornthippithak**
## Overview
  Implement variance functions to calculate linear algebra problems using parallel programming. With parallel technique, we aim to optimize time complexity compared to sequential programming.

## Implemented functions

Our project consists of 2 parts, sequential and parallel in total of 15 functions.

matrix_multiply_dot : Dot product of vector and Multiplication of matrix
- `seq_dot_product()`
- `par_dot_product()`
- `seq_matrix_mult()`
- `par_matrix_mult()`

matrix_transpose : Transpose of matrix
- `seq_transpose()`
- `par_transpose()`

matrix_trace : Trace of matrix
- `seq_trace()`
- `par_trace()`

matrix_det : Determinant of matrix
- `par_det()`
- `seq_det()`
- `par_norm()`
- `par_project()`
- `gram_schmidt()`
- `par_qr_decomposition()`
- `seq_qr_decomposition()`

As for matrix determinants, we aim to compute determinants using the QR-decomposition approach, where Q is an orthogonal matrix and R is an upper triangular matrix. As det(A) = det(Q) det(R), since det(Q) = 1, we can conclude that det(A) is the product of the main diagonal of R.

## Report / Presentation slide 
Links to our presentation: https://docs.google.com/presentation/d/1XL6G6v-zR7cseWxuuZ8E2usEFzpTb4fhwZV5MaNWnHE/edit?usp=sharing
or pdf version under `report.pdf`
