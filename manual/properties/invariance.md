## Invariance

Invariance properties determine how estimators respond to data transformations.
These properties are crucial for analysis design and interpretation:

- **Location-invariant** estimators are invariant to additive shifts: $T(\x+k)=T(\x)$
- **Scale-invariant** estimators are invariant to positive rescaling: $T(k \cdot \x)=T(\x)$ for $k>0$
- **Equivariant** estimators change predictably with transformations, maintaining relative relationships

Choosing estimators with appropriate invariance properties ensures that results remain
  meaningful across different measurement scales, units, and data transformations.
For example, when comparing datasets collected with different instruments or protocols,
  location-invariant estimators eliminate the need for data centering,
  while scale-invariant estimators eliminate the need for normalization.

**Location-invariance**: An estimator $T$ is location-invariant if adding a constant to the measurements leaves the result unchanged:

$$
T(\x + k) = T(\x)
$$

$$
T(\x + k, \y + k) = T(\x, \y)
$$

**Location-equivariance**: An estimator $T$ is location-equivariant if it shifts with the data:

$$
T(\x + k) = T(\x) + k
$$

$$
T(\x + k_1, \y + k_2) = T(\x, \y) + f(k_1, k_2)
$$

**Scale-invariance**: An estimator $T$ is scale-invariant if multiplying by a positive constant leaves the result unchanged:

$$
T(k \cdot \x) = T(\x) \quad \text{for } k > 0
$$

$$
T(k \cdot \x, k \cdot \y) = T(\x, \y) \quad \text{for } k > 0
$$

**Scale-equivariance**: An estimator $T$ is scale-equivariant if it scales proportionally with the data:

$$
T(k \cdot \x) = k \cdot T(\x) \text{ or } |k| \cdot T(\x) \quad \text{for } k \neq 0
$$

$$
T(k \cdot \x, k \cdot \y) = k \cdot T(\x, \y) \text{ or } |k| \cdot T(\x, \y) \quad \text{for } k \neq 0
$$

|           | Location     | Scale        |
|-----------|--------------|--------------|
| Center    | Equivariant  | Equivariant  |
| Spread    | Invariant    | Equivariant  |
| RelSpread | –            | Invariant    |
| Shift     | Invariant    | Equivariant  |
| Ratio     | –            | Invariant    |
| AvgSpread | Invariant    | Equivariant  |
| Disparity | Invariant    | Invariant    |