/**
 * Local step-density distribution reconstructed from a sample (algorithm dist-neighbor-v1).
 *
 * Each observation contributes a cell whose height reflects how tightly its nearest neighbors are
 * packed, so the reconstruction follows local structure instead of imposing a single global shape.
 * The window is indexed by k0 = ceil(sqrt(n)) with radius r = floor(k0 / 2); windows near the ends
 * are truncated and the numerator is always the actual neighbor count.
 *
 * The construction is kNN-inspired but is not the Loftsgaarden-Quesenberry estimator: support is
 * bounded by the observed range, and the cell masses are explicitly normalized so the density
 * integrates to one.
 *
 * The model is immutable, so concurrent queries on a built model are safe.
 */

import { Rng } from '../rng';
import { Subject, checkCumulance, checkQueryValidity, checkValidity } from '../assumptions';
import { normalizeZero } from '../estimators';
import {
  distCeilSqrt,
  distLerp,
  distLowerBound,
  distMidpoint,
  distNextDown,
  distNextUp,
  distUnitRatio,
  distUpperBound,
} from '../distNumerics';
import { distPrepare, distToMeasurements, distToRaw } from '../distSampleGuard';
import { MeasurementUnit } from '../measurement-unit';
import { Sample, checkNonWeighted } from '../sample';
import { Distribution } from './distribution';
import { DistMeasuredView } from './distMeasuredView';
import { NeighborLandscape } from './neighborLandscape';

export class NeighborDistModel implements Distribution {
  /** Normative algorithm identifier. */
  static readonly ALGORITHM = 'dist-neighbor-v1';

  private readonly b: readonly number[];
  private readonly f: readonly number[];
  private readonly p: readonly number[];
  private readonly d: readonly number[];
  private readonly sources: readonly number[];
  private readonly scaleIndex: number;
  private readonly radius: number;

  /** Unit of the observations this model was built from. */
  readonly unit: MeasurementUnit;

  /**
   * Builds the model from a native numeric vector or a measured Sample.
   *
   * On the Sample path the sample already stores its values in ascending order, so the model
   * does not sort again. A weighted Sample is refused: this reconstruction has no weighted form,
   * and silently ignoring the weights would answer a different question than the caller asked.
   *
   * On the numeric path `values` must be non-empty, finite, of size at least two, with a finite
   * representable range and pairwise distinct elements. `assumeSorted`, when true, promises the
   * input is already ascending and the internal copy is not sorted.
   */
  constructor(x: number[], assumeSorted?: boolean);
  constructor(x: Sample);
  constructor(x: number[] | Sample, assumeSorted = false) {
    let prepared: number[];
    if (x instanceof Sample) {
      checkNonWeighted('NeighborDistModel', x);
      prepared = distPrepare(x.sortedValues, true);
      this.unit = x.unit;
    } else {
      checkValidity(x, Subject.X);
      prepared = distPrepare(x, assumeSorted);
      this.unit = MeasurementUnit.NUMBER;
    }

    const n = prepared.length;
    this.scaleIndex = distCeilSqrt(n);
    this.radius = Math.floor(this.scaleIndex / 2);

    const counts = new Array<number>(n);
    const spans = new Array<number>(n);
    for (let i = 0; i < n; i++) {
      const left = Math.max(0, i - this.radius);
      const right = Math.min(n - 1, i + this.radius);
      counts[i] = right - left + 1;
      spans[i] = prepared[right] - prepared[left];
    }

    const e = new Array<number>(n + 1);
    e[0] = prepared[0];
    for (let i = 1; i < n; i++) {
      e[i] = distMidpoint(prepared[i - 1], prepared[i]);
    }
    e[n] = prepared[n - 1];

    // Rounding can collapse the midpoint of two adjacent binary64 observations onto a
    // neighboring boundary. Such a cell carries no mass and is dropped; the surviving cells keep
    // the index of the observation they came from.
    const boundaries = [e[0]];
    const surviving: number[] = [];
    for (let i = 0; i < n; i++) {
      if (e[i + 1] === e[i]) continue;
      surviving.push(i);
      boundaries.push(e[i + 1]);
    }

    const s = surviving.length;
    const a = new Array<number>(s);
    for (let j = 0; j < s; j++) {
      const i = surviving[j];
      const width = e[i + 1] - e[i];
      a[j] = (counts[i] / n) * (width / spans[i]);
    }

    let total = 0;
    for (let j = 0; j < s; j++) {
      total += a[j];
    }

    // A broken assumeSorted promise can make a cell wider than the window it normalizes against,
    // so its unnormalized mass overflows, or it can collapse every cell and leave a zero total.
    // Either way the divisor is unusable and NaN would reach the query kernels. The sample itself
    // satisfies domain(x) and distinctness(x), so this is not an assumption violation: it is a
    // numerical-conformance failure, reported through the same channel distUnitRatio uses. For a
    // genuinely ascending sample the total is finite and strictly positive -- it is not bounded
    // by 1, since it is a normalizer rather than a probability; it sits above 1 and decays toward
    // it as n grows.
    if (!Number.isFinite(total) || total <= 0) {
      throw new Error(
        'Numerical conformance failure: the NeighborDistModel normalizer is not usable. ' +
          'This indicates an assumeSorted promise that the input does not satisfy.',
      );
    }

    // Reserve one representable step for every cell that has not been placed yet, so a mass too
    // small to register at this point in the prefix still advances the cumulative value instead
    // of flattening it into a plateau.
    const cap = new Array<number>(s + 1);
    cap[s] = 1;
    for (let j = s - 1; j >= 0; j--) {
      cap[j] = distNextDown(cap[j + 1]);
    }

    const f = new Array<number>(s + 1);
    f[0] = 0;
    for (let j = 0; j < s - 1; j++) {
      const raw = f[j] + a[j] / total;
      f[j + 1] = Math.min(cap[j + 1], Math.max(distNextUp(f[j]), raw));
    }
    f[s] = 1;

    this.b = boundaries;
    this.sources = surviving;
    this.f = f;
    const p = new Array<number>(s);
    const d = new Array<number>(s);
    for (let j = 0; j < s; j++) {
      p[j] = f[j + 1] - f[j];
      d[j] = p[j] / (boundaries[j + 1] - boundaries[j]);
    }
    this.p = p;
    this.d = d;
  }

  /** Number of surviving cells in the reconstruction. */
  get cellCount(): number {
    return this.p.length;
  }

  /** Lower end of the support. */
  get supportMin(): number {
    return this.b[0];
  }

  /** Upper end of the support. */
  get supportMax(): number {
    return this.b[this.b.length - 1];
  }

  /** Value at each requested cumulance, in the order the cumulances were given. */
  quantile(cumulances: readonly number[]): number[] {
    checkQueryValidity(cumulances, Subject.Y);
    checkCumulance(cumulances, Subject.Y);
    return this.quantileTrusted(cumulances);
  }

  /** Probability of not exceeding each requested value, in the order given. */
  cumulative(values: readonly number[]): number[] {
    checkQueryValidity(values, Subject.Y);
    return this.cumulativeTrusted(values);
  }

  /** Density at each requested value, in the order the values were given. */
  density(values: readonly number[]): number[] {
    checkQueryValidity(values, Subject.Y);
    return this.densityTrusted(values);
  }

  private quantileTrusted(cumulances: readonly number[]): number[] {
    return cumulances.map((p) => this.quantileAt(p));
  }

  private cumulativeTrusted(values: readonly number[]): number[] {
    return values.map((v) => this.cumulativeAt(v));
  }

  private densityTrusted(values: readonly number[]): number[] {
    return values.map((v) => this.densityAt(v));
  }

  /**
   * The measured view of this model: quantile returns a Measurement in the sample's unit,
   * cumulative and density return plain numbers.
   */
  get measured(): DistMeasuredView {
    return {
      quantile: (cumulances) => distToMeasurements(this.quantile(cumulances), this.unit),
      cumulative: (values) => this.cumulative(distToRaw(values, this.unit)),
      density: (values) => this.density(distToRaw(values, this.unit)),
    };
  }

  sample(rng: Rng): number {
    return this.quantileAt(rng.uniformFloat());
  }

  samples(rng: Rng, count: number): number[] {
    return Array.from({ length: count }, () => this.sample(rng));
  }

  /**
   * NaN first, for the same reason LinearDistModel.quantileAt checks it: every branch below
   * compares false against it, and the fallback would otherwise depend on how the platform casts
   * a NaN cumulance to an index.
   */
  quantileAt(p: number): number {
    if (Number.isNaN(p)) return NaN;

    const s = this.p.length;
    let value: number;
    if (p <= 0) {
      value = this.b[0];
    } else if (p >= 1) {
      value = this.b[s];
    } else {
      const j = distLowerBound(this.f, 1, s + 1, p) - 1;
      const t = distUnitRatio(p - this.f[j], this.p[j]);
      value = distLerp(this.b[j], this.b[j + 1], t);
    }

    // Every branch hands back a value built from the sample's own boundaries -- the clamped ones
    // return one verbatim, and distLerp reproduces its endpoints exactly -- so a -0 among the
    // observations would otherwise reach the caller unnormalized.
    return normalizeZero(value);
  }

  cumulativeAt(x: number): number {
    if (Number.isNaN(x)) return NaN;

    const s = this.p.length;
    if (x <= this.b[0]) return 0;
    if (x >= this.b[s]) return 1;

    const j = distUpperBound(this.b, x) - 1;
    const t = distUnitRatio(x - this.b[j], this.b[j + 1] - this.b[j]);
    return distLerp(this.f[j], this.f[j + 1], t);
  }

  densityAt(x: number): number {
    if (Number.isNaN(x)) return NaN;

    const s = this.p.length;
    if (x < this.b[0] || x > this.b[s]) return 0;

    const j = x === this.b[s] ? s - 1 : distUpperBound(this.b, x) - 1;
    return this.p[j] / (this.b[j + 1] - this.b[j]);
  }

  /**
   * The staircase this model is: cell boundaries, cumulative values, masses and heights.
   *
   * Point evaluation answers what the density is at a value a caller picked; this answers what
   * the density is, on the boundaries the model itself uses. Every height is the expression
   * densityAt evaluates inside the same cell, so the two agree bit for bit rather than
   * approximately.
   */
  getLandscape(): NeighborLandscape {
    return new NeighborLandscape(
      this.scaleIndex,
      this.radius,
      [...this.b],
      [...this.f],
      [...this.p],
      [...this.d],
      [...this.sources],
    );
  }
}
