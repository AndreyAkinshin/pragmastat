import { AssumptionError, AssumptionId, Subject, checkCumulance } from '../src/assumptions';

describe('checkCumulance', () => {
  it('rejects NaN', () => {
    let thrown: AssumptionError | null = null;
    try {
      checkCumulance([NaN], Subject.Y);
    } catch (e) {
      if (e instanceof AssumptionError) thrown = e;
    }
    expect(thrown).not.toBeNull();
    expect(thrown!.violation!.id).toBe(AssumptionId.CUMULANCE);
    expect(thrown!.violation!.subject).toBe(Subject.Y);
  });

  it('accepts the closed unit interval', () => {
    expect(() => checkCumulance([0, 0.5, 1], Subject.Y)).not.toThrow();
  });

  it('rejects outside the unit interval', () => {
    expect(() => checkCumulance([1.5], Subject.Y)).toThrow(AssumptionError);
  });
});
