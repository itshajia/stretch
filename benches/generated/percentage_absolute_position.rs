pub fn compute() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch::style::Style {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10f32),
                    height: stretch::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Percent(0.3f32),
                    top: stretch::style::Dimension::Percent(0.1f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch::style::Style {
                flex_direction: stretch::style::FlexDirection::Column,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(200f32),
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
