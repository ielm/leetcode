# Largest Sum Contiguous Subarray

This is also called the Kadane's algorithm.

## Problem Statement

Given an array of integers, find the contiguous subarray that has the largest sum and return the sum.

The idea of Kadane's Algorithm is to maintain a variable `max_so_far` that stores the maximum sum of subarray found so far and a variable `max_ending_here` that stores the maximum sum of subarray ending at the current position. Every time there is a positive sum value in `max_ending_here`, compare it with `max_so_far` and update `max_so_far` if it is greater than `max_so_far`.

The main intuition is:

- The subarray with a negative sum is discarded (by assigning max_ending_here to 0) because adding a negative number to the sum will only decrease the sum.
- The subarray with a positive sum is kept because adding a positive number to the sum will only increase the sum.
- We carry subarray until it gives a positive sum
