use crate::assumptions::{AssumptionId, EstimatorError, Subject};
use crate::estimators::ratio_bounds;
use crate::sample::Sample;

#[test]
fn ratio_bounds_domain_before_positivity() {
    // misrate=-0.1 is invalid (domain), x=-1 is non-positive (positivity)
    // domain(misrate) must take priority over positivity(x)
    let x = Sample::new(vec![-1.0]).unwrap();
    let y = Sample::new(vec![1.0]).unwrap();
    let result = ratio_bounds(&x, &y, -0.1);
    match result {
        Err(EstimatorError::Assumption(ref ae)) => {
            assert_eq!(ae.violation().id, AssumptionId::Domain);
            assert_eq!(ae.violation().subject, Subject::Misrate);
        }
        other => panic!("Expected domain(misrate), got {:?}", other),
    }
}

#[test]
fn ratio_bounds_positivity_when_misrate_valid() {
    // Valid misrate but non-positive x → positivity(x)
    let x = Sample::new(vec![-1.0, -2.0, -3.0]).unwrap();
    let y = Sample::new(vec![1.0, 2.0, 3.0]).unwrap();
    let result = ratio_bounds(&x, &y, 0.5);
    match result {
        Err(EstimatorError::Assumption(ref ae)) => {
            assert_eq!(ae.violation().id, AssumptionId::Positivity);
            assert_eq!(ae.violation().subject, Subject::X);
        }
        other => panic!("Expected positivity(x), got {:?}", other),
    }
}
