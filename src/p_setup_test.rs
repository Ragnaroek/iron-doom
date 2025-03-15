use crate::test_helper::setup_e1m1;

#[test]
fn test_setup_level() -> Result<(), String> {
    let (_, level) = setup_e1m1()?;

    assert_eq!(level.nodes.len(), 236);
    assert_eq!(level.nodes[235].children[0], 128);
    assert_eq!(level.nodes[235].children[1], 234);

    Ok(())
}
