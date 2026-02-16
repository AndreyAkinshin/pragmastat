#import "/manual/definitions.typ": *

The $Median$ algorithm sorts the sample and selects the middle element:

+ *Sort* --- Arrange the sample to obtain $x_((1)) <= x_((2)) <= ... <= x_((n))$.

+ *Select* ---
  If $n$ is odd, return $x_(((n+1)\/2))$.
  If $n$ is even, return $(x_((n\/2)) + x_((n\/2+1))) / 2$.

The time complexity is $O(n log n)$ dominated by the sort.
This standard algorithm is used as a building block by other estimators;
  no specialized implementation is needed beyond the language's built-in sort.
