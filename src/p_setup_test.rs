use crate::{
    fixed::{FRAC_BITS, Fixed, ZERO},
    test_helper::setup_e1m1,
};

#[test]
fn test_setup_level() -> Result<(), String> {
    let (_, level) = setup_e1m1()?;

    //sample test some data

    assert_eq!(level.nodes.len(), 236);
    assert_eq!(level.nodes[235].children[0], 128);
    assert_eq!(level.nodes[235].children[1], 234);

    assert_eq!(level.segs.len(), 732);
    assert_eq!(level.segs[0].v1.x, Fixed::new_from_u32(1552 << FRAC_BITS));
    assert_eq!(level.segs[0].v1.y, Fixed::new_from_i32(-2560 << FRAC_BITS));
    assert_eq!(level.segs[0].v2.x, Fixed::new_from_u32(1552 << FRAC_BITS));
    assert_eq!(level.segs[0].v2.y, Fixed::new_from_i32(-2432 << FRAC_BITS));
    assert_eq!(level.segs[0].angle, 16384 << 16);
    assert_eq!(level.segs[0].offset, ZERO);
    assert_eq!(level.segs[0].line_def, 152);
    assert_eq!(level.segs[0].side, 0);

    assert_eq!(level.segs[731].v1.x, Fixed::new_from_u32(3040 << FRAC_BITS));
    assert_eq!(
        level.segs[731].v1.y,
        Fixed::new_from_i32(-4672 << FRAC_BITS)
    );
    assert_eq!(level.segs[731].v2.x, Fixed::new_from_u32(2976 << FRAC_BITS));
    assert_eq!(
        level.segs[731].v2.y,
        Fixed::new_from_i32(-4672 << FRAC_BITS)
    );
    assert_eq!(level.segs[731].angle, 2147483648);
    assert_eq!(level.segs[731].offset, ZERO);
    assert_eq!(level.segs[731].line_def, 333);
    assert_eq!(level.segs[731].side, 1);

    Ok(())
}
