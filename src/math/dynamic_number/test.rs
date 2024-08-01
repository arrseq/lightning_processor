use crate::math::dynamic_number::Size;

#[test]
fn minimum_signed() {
    assert_eq!(Size::minimum_signed(-128), Size::X8);
    assert_eq!(Size::minimum_signed(127), Size::X8);
    
    assert_eq!(Size::minimum_signed(-129), Size::X16);
    assert_eq!(Size::minimum_signed(128), Size::X16);
}