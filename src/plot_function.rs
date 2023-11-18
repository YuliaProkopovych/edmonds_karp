use plotters::{
    backend::BitMapBackend,
    chart::ChartBuilder,
    drawing::IntoDrawingArea,
    element::{Circle, EmptyElement},
    series::PointSeries,
    style::{full_palette::{BLACK, WHITE}, TextStyle, IntoFont},
};

pub fn plot(
    series: Vec<(f32, u32)>,
    max_number_of_nodes: u32,
    min_number_of_nodes: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plotters_plot/e_k.png", (820, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(20, 20, 20, 20);

    let mut max_execution_time = series[0].0;
    series.iter().for_each(| value | {
        if value.0 > max_execution_time {
            max_execution_time = value.0
        }
    });

    let mut chart = ChartBuilder::on(&root)
        .caption("Edmonds Karp", ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(
            0f32..(max_execution_time + 0.2),
            (min_number_of_nodes - 1)..(max_number_of_nodes + 1),
        )?;

    chart
        .configure_mesh()
        .x_label_style(TextStyle::from(("sans-serif", 15).into_font()))
        .axis_desc_style(TextStyle::from(("sans-serif", 18).into_font()))
        .y_label_style(TextStyle::from(("sans-serif", 15).into_font()))
        //.y_desc_style(TextStyle::from(("sans-serif", 18).into_font()))
        .x_label_formatter(&|x| format!("{}ms", x))
        .y_label_formatter(&|x| format!("{}", x))
        .x_desc("Time of execution")
        .y_desc("Number of nodes")
        .draw()?;

    chart.draw_series(PointSeries::of_element(series, 1, &BLACK, &|c, s, st| {
        return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
    }))?;
    root.present()?;
    Ok(())
}