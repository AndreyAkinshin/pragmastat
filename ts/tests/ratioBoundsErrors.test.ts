import { ratioBounds } from '../src/estimators';
import { AssumptionError, AssumptionId, Subject } from '../src/assumptions';

/**
 * Assumption-error priority for ratioBounds: the misrate domain check runs
 * before the positivity check on the sample values, so an invalid misrate is
 * reported even when x also violates positivity.
 */
describe('ratioBounds assumption-error priority', () => {
  function expectViolation(fn: () => unknown, id: AssumptionId, subject: Subject): void {
    let thrown: AssumptionError | null = null;
    try {
      fn();
    } catch (e) {
      if (e instanceof AssumptionError) thrown = e;
    }
    expect(thrown).not.toBeNull();
    expect(thrown!.violation!.id).toBe(id);
    expect(thrown!.violation!.subject).toBe(subject);
  }

  it('reports domain(misrate) before positivity(x)', () => {
    // misrate=-0.1 is out of domain AND x is non-positive; domain(misrate) wins.
    expectViolation(() => ratioBounds([-1], [1], -0.1), AssumptionId.DOMAIN, 'misrate');
  });

  it('reports positivity(x) when misrate is valid', () => {
    expectViolation(() => ratioBounds([-1, -2, -3], [1, 2, 3], 0.5), AssumptionId.POSITIVITY, 'x');
  });
});
