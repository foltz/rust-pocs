use yata::core::Method;
use yata::methods::EMA;

#[test]
pub fn test_ema() {
    
    
    // EMA of length=3
    let mut ema = EMA::new(3, &3.0).unwrap();

    ema.next(&3.0);
    ema.next(&6.0);

    assert_eq!(ema.next(&9.0), 6.75);
    assert_eq!(ema.next(&12.0), 9.375);
}