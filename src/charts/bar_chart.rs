use super::canvas;
use super::color::*;
use super::common::*;
use super::component::*;
use super::theme::{get_default_theme, get_theme, Theme, DEFAULT_Y_AXIS_WIDTH};
use super::util::*;
use super::Canvas;
use super::Chart;
use crate::charts::measure_text_width_family;
use charts_rs_derive::Chart;

#[derive(Clone, Debug, Default, Chart)]
pub struct BarChart {
    pub width: f32,
    pub height: f32,
    pub margin: Box,
    pub series_list: Vec<Series>,
    pub font_family: String,
    pub background_color: Color,
    pub is_light: bool,

    // title
    pub title_text: String,
    pub title_font_size: f32,
    pub title_font_color: Color,
    pub title_font_weight: Option<String>,
    pub title_margin: Option<Box>,
    pub title_align: Align,

    // sub title
    pub sub_title_text: String,
    pub sub_title_font_size: f32,
    pub sub_title_font_color: Color,
    pub sub_title_margin: Option<Box>,
    pub sub_title_align: Align,

    // legend
    pub legend_font_size: f32,
    pub legend_font_color: Color,
    pub legend_align: Align,
    pub legend_margin: Option<Box>,
    pub legend_category: LegendCategory,

    // x axis
    pub x_axis_data: Vec<String>,
    pub x_axis_height: f32,
    pub x_axis_stroke_color: Color,
    pub x_axis_font_size: f32,
    pub x_axis_font_color: Color,
    pub x_axis_name_gap: f32,
    pub x_axis_name_rotate: f32,
    pub x_boundary_gap: Option<bool>,

    // y axis
    pub y_axis_font_size: f32,
    pub y_axis_font_color: Color,
    pub y_axis_stroke_color: Color,
    pub y_axis_width: Option<f32>,
    pub y_axis_split_number: usize,
    pub y_axis_name_gap: f32,
    pub y_axis_formatter: Option<String>,

    // grid
    pub grid_stroke_color: Color,
    pub grid_stroke_width: f32,

    // series
    pub series_stroke_width: f32,
    pub series_colors: Vec<Color>,
    pub series_symbol: Option<Symbol>,
    pub series_smooth: bool,
    pub series_fill: bool,
}

impl BarChart {
    pub fn new(mut series_list: Vec<Series>, x_axis_data: Vec<String>) -> BarChart {
        let mut series_index: usize = 0;
        // bar chart 可能同时支持两种图
        // 因此先计算index
        series_list.iter_mut().for_each(|item| {
            item.index = Some(series_index);
            series_index += 1;
        });
        let mut b = BarChart {
            series_list,
            x_axis_data,
            ..Default::default()
        };
        let theme = get_theme(get_default_theme());
        b.fill_theme(theme);
        b
    }
    pub fn svg(&self) -> canvas::Result<String> {
        let mut c = Canvas::new(self.width, self.height);

        self.render_background(c.child(Box::default()));
        c.margin = self.margin.clone();

        let title_height = self.render_title(c.child(Box::default()));

        let legend_height = self.render_legend(c.child(Box::default()));
        // title 与 legend 取较高的值
        let axis_top = if legend_height > title_height {
            legend_height
        } else {
            title_height
        };

        let (y_axis_values, y_axis_width) = self.get_y_axis_values(0);

        let axis_height = c.height() - self.x_axis_height - axis_top;
        let axis_width = c.width() - y_axis_width;
        // 减去顶部文本区域
        if axis_top > 0.0 {
            c = c.child(Box {
                top: axis_top,
                ..Default::default()
            });
        }

        self.render_grid(
            c.child(Box {
                left: y_axis_width,
                right: y_axis_width,
                ..Default::default()
            }),
            axis_width,
            axis_height,
        );

        // y axis
        self.render_y_axis(
            c.child(Box::default()),
            y_axis_values.data.clone(),
            axis_height,
            y_axis_width,
        );

        // x axis
        self.render_x_axis(
            c.child(Box {
                top: c.height() - self.x_axis_height,
                left: y_axis_width,
                ..Default::default()
            }),
            self.x_axis_data.clone(),
            axis_width,
        );

        // bar point
        let max_height = c.height() - self.x_axis_height;
        let mut bar_series_list = vec![];
        let mut line_series_list = vec![];
        self.series_list.iter().for_each(|item| {
            if let Some(ref cat) = item.category {
                if *cat == SeriesCategory::Line {
                    line_series_list.push(item);
                    return;
                }
            }
            bar_series_list.push(item);
        });

        self.render_bar(
            c.child(Box {
                left: y_axis_width,
                ..Default::default()
            }),
            &bar_series_list,
            &y_axis_values,
            max_height,
        );
        self.render_line(
            c.child(Box {
                left: y_axis_width,
                ..Default::default()
            }),
            &line_series_list,
            &y_axis_values,
            max_height,
            axis_height,
        );

        c.svg()
    }
}

#[cfg(test)]
mod tests {
    use super::BarChart;
    use crate::{Box, LegendCategory, Series, SeriesCategory};
    use pretty_assertions::assert_eq;
    #[test]
    fn bar_chart_basic() {
        let mut bar_chart = BarChart::new(
            vec![
                Series::new(
                    "Email".to_string(),
                    vec![120.0, 132.0, 101.0, 134.0, 90.0, 230.0, 210.0],
                ),
                Series::new(
                    "Union Ads".to_string(),
                    vec![220.0, 182.0, 191.0, 234.0, 290.0, 330.0, 310.0],
                ),
                Series::new(
                    "Direct".to_string(),
                    vec![320.0, 332.0, 301.0, 334.0, 390.0, 330.0, 320.0],
                ),
                Series::new(
                    "Search Engine".to_string(),
                    vec![820.0, 932.0, 901.0, 934.0, 1290.0, 1330.0, 1320.0],
                ),
            ],
            vec![
                "Mon".to_string(),
                "Tue".to_string(),
                "Wed".to_string(),
                "Thu".to_string(),
                "Fri".to_string(),
                "Sat".to_string(),
                "Sun".to_string(),
            ],
        );
        bar_chart.y_axis_width = Some(55.0);
        bar_chart.title_text = "Bar Chart".to_string();
        bar_chart.legend_margin = Some(Box {
            top: 30.0,
            bottom: 10.0,
            ..Default::default()
        });
        bar_chart.y_axis_formatter = Some("{c} ml".to_string());
        assert_eq!(
            include_str!("../../asset/bar_chart/basic.svg"),
            bar_chart.svg().unwrap()
        );
    }
    #[test]
    fn bar_chart_line_mixin() {
        let mut bar_chart = BarChart::new(
            vec![
                Series::new(
                    "Email".to_string(),
                    vec![120.0, 132.0, 101.0, 134.0, 90.0, 230.0, 210.0],
                ),
                Series::new(
                    "Union Ads".to_string(),
                    vec![220.0, 182.0, 191.0, 234.0, 290.0, 330.0, 310.0],
                ),
                Series::new(
                    "Direct".to_string(),
                    vec![320.0, 332.0, 301.0, 334.0, 390.0, 330.0, 320.0],
                ),
                Series::new(
                    "Search Engine".to_string(),
                    vec![820.0, 932.0, 901.0, 934.0, 1290.0, 1330.0, 1320.0],
                ),
            ],
            vec![
                "Mon".to_string(),
                "Tue".to_string(),
                "Wed".to_string(),
                "Thu".to_string(),
                "Fri".to_string(),
                "Sat".to_string(),
                "Sun".to_string(),
            ],
        );
        bar_chart.series_list[0].category = Some(SeriesCategory::Line);
        bar_chart.y_axis_width = Some(55.0);
        bar_chart.title_text = "Bar Chart".to_string();
        bar_chart.legend_margin = Some(Box {
            top: 30.0,
            bottom: 10.0,
            ..Default::default()
        });
        bar_chart.legend_category = LegendCategory::Rect;
        bar_chart.y_axis_formatter = Some("{c} ml".to_string());
        assert_eq!(
            include_str!("../../asset/bar_chart/line_mixin.svg"),
            bar_chart.svg().unwrap()
        );
    }
}