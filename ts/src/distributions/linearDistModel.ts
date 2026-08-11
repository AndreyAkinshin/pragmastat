/**
 * Piecewise-linear distribution reconstructed from a sample (algorithm dist-linear-v1).
 *
 * The model interpolates the order statistics at the anchors (i-1)/(n-1), the Hyndman-Fan type 7
 * rule. It is immutable: the constructor copies and, unless the caller promises otherwise, sorts
 * the input.
 *
 * Support is [min(x), max(x)]. v1 exposes plain numbers on the numeric path and does not extend
 * the runtime unit registry.
 */

import { Rng } from '../rng';
import { Subject, checkCumulance, checkQueryValidity, checkValidity } from '../assumptions';
import { normalizeZero } from '../estimators';
import { distLerp, distUnitRatio, distUpperBound } from '../distNumerics';
import { distPrepare, distToMeasurements, distToRaw } from '../distSampleGuard';
import { MeasurementUnit } from '../measurement-unit';
import { Sample, checkNonWeighted } from '../sample';
import { Distribution } from './distribution';
import { DistMeasuredView } from './distMeasuredView';
import { LinearLandscape } from './linearLandscape';

export class LinearDistModel implements Distribution {
  /** Normative algorithm identifier. */
  static readonly ALGORITHM = 'dist-linear-v1';

  private readonly x: readonly number[];

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
   * input is already ascending and the internal copy is not sorted; passing an unsorted array
   * under this flag is a contract violation with an unspecified but terminating result.
   */
  constructor(x: number[], assumeSorted?: boolean);
  constructor(x: Sample);
  constructor(x: number[] | Sample, assumeSorted = false) {
    if (x instanceof Sample) {
      checkNonWeighted('LinearDistModel', x);
      this.x = distPrepare(x.sortedValues, true);
      this.unit = x.unit;
    } else {
      checkValidity(x, Subject.X);
      this.x = distPrepare(x, assumeSorted);
      this.unit = MeasurementUnit.NUMBER;
    }
  }

  /** Number of observations backing the model. */
  get size(): number {
    return this.x.length;
  }

  /** Lower end of the support. */
  get supportMin(): number {
    return this.x[0];
  }

  /** Upper end of the support. */
  get supportMax(): number {
    return this.x[this.x.length - 1];
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
   * NaN first: every branch below compares false against it, and the fallback arithmetic would
   * otherwise depend on how the platform casts a NaN cumulance to an index. The same guard opens
   * cumulativeAt and densityAt, so all three answer the same way to the same non-input.
   */
  quantileAt(p: number): number {
    if (Number.isNaN(p)) return NaN;

    const n = this.x.length;
    let value: number;
    if (p <= 0) {
      value = this.x[0];
    } else if (p >= 1) {
      value = this.x[n - 1];
    } else {
      const r = (n - 1) * p;
      const j = Math.floor(r);
      const g = r - j;
      value = distLerp(this.x[j], this.x[j + 1], g);
    }

    // Every branch hands back a value the sample supplied -- the clamped ones return an order
    // statistic verbatim, and distLerp reproduces its endpoints exactly -- so a -0 among the
    // observations would otherwise reach the caller unnormalized.
    return normalizeZero(value);
  }

  cumulativeAt(x: number): number {
    if (Number.isNaN(x)) return NaN;

    const n = this.x.length;
    if (x <= this.x[0]) return 0;
    if (x >= this.x[n - 1]) return 1;

    const j = distUpperBound(this.x, x) - 1;
    const t = distUnitRatio(x - this.x[j], this.x[j + 1] - this.x[j]);
    return (j + t) / (n - 1);
  }

  densityAt(x: number): number {
    if (Number.isNaN(x)) return NaN;

    const n = this.x.length;
    if (x < this.x[0] || x > this.x[n - 1]) return 0;

    const j = x === this.x[n - 1] ? n - 2 : distUpperBound(this.x, x) - 1;
    return 1 / (n - 1) / (this.x[j + 1] - this.x[j]);
  }

  /**
   * The staircase this model is: cell boundaries, cumulative values and heights.
   *
   * Point evaluation answers what the density is at a value a caller picked; this answers what
   * the density is, on the boundaries the model itself uses. Every height is the expression
   * densityAt evaluates inside the same cell, so the two agree bit for bit rather than
   * approximately.
   */
  getLandscape(): LinearLandscape {
    const n = this.x.length;
    const cumulative = this.x.map((v) => this.cumulativeAt(v));
    const densities: number[] = [];
    for (let j = 0; j < n - 1; j++) {
      densities.push(1 / (n - 1) / (this.x[j + 1] - this.x[j]));
    }
    return new LinearLandscape([...this.x], cumulative, densities);
  }
}
