pub fn compute() {
    let mut stretch = stretch::Stretch::new();
    let node00 = stretch
        .new_node(
            stretch::style::Style {
                flex_grow: 1f32,
                flex_basis: stretch::style::Dimension::Points(0f32),
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = stretch.new_node(stretch::style::Style { ..Default::default() }, &[node00]).unwrap();
    let node = stretch
        .new_node(
            stretch::style::Style {
                align_items: stretch::style::AlignItems::FlexStart,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100f32),
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
}
