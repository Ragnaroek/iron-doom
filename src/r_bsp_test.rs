use crate::{fixed::new_fixed_i32, test_helper::setup_e1m1};

use super::point_on_side;

#[test]
fn test_point_on_side() -> Result<(), String> {
    let (_, level) = setup_e1m1()?;
    // view point start E1M1
    let x = new_fixed_i32(69206016);
    let y = new_fixed_i32(-236978176);

    assert_eq!(point_on_side(x, y, &level.nodes[235]), false);
    assert_eq!(point_on_side(x, y, &level.nodes[128]), true);
    assert_eq!(point_on_side(x, y, &level.nodes[127]), true);
    assert_eq!(point_on_side(x, y, &level.nodes[126]), false);
    assert_eq!(point_on_side(x, y, &level.nodes[113]), false);
    assert_eq!(point_on_side(x, y, &level.nodes[101]), true);
    assert_eq!(point_on_side(x, y, &level.nodes[100]), false);
    assert_eq!(point_on_side(x, y, &level.nodes[99]), false);
    assert_eq!(point_on_side(x, y, &level.nodes[96]), false);
    assert_eq!(point_on_side(x, y, &level.nodes[94]), true);
    assert_eq!(point_on_side(x, y, &level.nodes[95]), true);
    assert_eq!(point_on_side(x, y, &level.nodes[95]), true);
    assert_eq!(point_on_side(x, y, &level.nodes[98]), true);

    Ok(())
}
