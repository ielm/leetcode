# Segment Tree

A segment tree is a data structure that stores information about array intervals as a tree. This allows for answering range queries over an array efficiently, while still being flexible enough to allow modifying the array. This includes finding the sum of consecutive array elements `a[l…r]`, or finding the minimum element in a such a range in O(logn) time. Between answering such queries, the segment tree allows moodifying the array by replacing one element, or even changing the elements of a whole subsegment (e.g. assigning all elements `a[l…r]` to any value, or adding a value to all element in the subsegment).

Segment trees are also known as a Statistic Tree.

![A Segment Tree](https://media.geeksforgeeks.org/wp-content/cdn-uploads/segment-tree1.png)

A segment tree for a set `l` of `n` intervals uses `O(nlogn)` storage and can be built in `O(nlogn)` time. Segment trees support searching for all the intervals that contain a query point in `O(logn + k)` time, where `k` is the number of retrieved intervals or segments.
