/**
 * The one spelling of "these two doubles are identical", shared by every suite
 * in this directory that claims exactness.
 *
 * The claim is about bits, so the comparison is about bits: both values are
 * read back as their raw binary64 payloads and those unsigned 64-bit integers
 * are compared. The two numeric predicates JavaScript offers answer a
 * different question. `===` calls -0 and +0 equal, so a wrong sign of zero
 * (reachable for a difference, or for an average of symmetric values) passes a
 * check whose entire claim is that all seven implementations return the same
 * bits. `Object.is` fixes that one, but folds every NaN into a single value, so
 * it cannot see a payload difference either. Neither is the predicate these
 * suites describe; the payload comparison below is.
 *
 * A failure prints both payloads in hex alongside the decimal values, because
 * decimal alone cannot always be read: JavaScript renders -0 and +0 as the same
 * "0" and every NaN as the same "NaN". (A one-ULP gap does render distinctly,
 * since a double prints with enough digits to round-trip, but the hex makes the
 * distance readable rather than something to spot by eye.)
 */

/** Reused so the payload of a double costs no allocation. */
const scratch = new DataView(new ArrayBuffer(8));

/** The raw binary64 payload of a double, as an unsigned 64-bit integer. */
function payload(value: number): bigint {
  scratch.setFloat64(0, value);
  return scratch.getBigUint64(0);
}

/** The raw binary64 payload of a double, as a zero-padded hex literal. */
function f64Hex(value: number): string {
  return `0x${payload(value).toString(16).padStart(16, '0')}`;
}

/** Asserts that two doubles carry the identical binary64 payload. */
export function expectBitwise(subject: string, actual: number, expected: number): void {
  if (payload(actual) !== payload(expected)) {
    throw new Error(
      `${subject}: bitwise mismatch: ` +
        `expected ${expected} (${f64Hex(expected)}), ` +
        `actual ${actual} (${f64Hex(actual)})`,
    );
  }
}

/** The payload every negative zero carries, and the only one that does. */
const NEGATIVE_ZERO = 0x8000000000000000n;

/**
 * Asserts that a double is not a negative zero.
 *
 * The weaker sibling of `expectBitwise`, for results whose value is not fixed in advance: it
 * pins only the sign of a zero. Neither `=== 0` nor `!== -0` can express this, since the two
 * zeros compare equal, so the check reads the payload.
 */
export function expectNoNegativeZero(subject: string, value: number): void {
  if (payload(value) === NEGATIVE_ZERO) {
    throw new Error(
      `${subject}: negative zero (${f64Hex(value)}), expected +0 (0x0000000000000000)`,
    );
  }
}

/** A lower/upper pair, structurally: `Bounds` and the fixture objects both fit. */
interface BoundsLike {
  lower: number;
  upper: number;
}

/** Asserts that both endpoints of an interval carry the identical payloads. */
export function expectBitwiseBounds(
  subject: string,
  actual: BoundsLike,
  expected: BoundsLike,
): void {
  expectBitwise(`${subject}.lower`, actual.lower, expected.lower);
  expectBitwise(`${subject}.upper`, actual.upper, expected.upper);
}

/** Asserts that two sequences agree in length and in every element's payload. */
export function expectBitwiseSequence(subject: string, actual: number[], expected: number[]): void {
  if (actual.length !== expected.length) {
    throw new Error(
      `${subject}: length mismatch: expected ${expected.length}, actual ${actual.length}`,
    );
  }
  for (let i = 0; i < actual.length; i++) {
    expectBitwise(`${subject} at index ${i}`, actual[i], expected[i]);
  }
}
