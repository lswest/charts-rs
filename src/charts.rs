mod bar_chart;
mod candlestick_chart;
mod canvas;
mod color;
mod common;
mod component;
#[cfg(feature = "image-encoder")]
mod encoder;
mod font;
mod heatmap_chart;
mod horizontal_bar_chart;
mod line_chart;
mod multi_chart;
mod params;
mod path;
mod pie_chart;
mod radar_chart;
mod scatter_chart;
mod table_chart;
mod theme;
mod util;

pub use bar_chart::BarChart;
pub use canvas::Canvas;
pub use canvas::Error as CanvasError;
pub use canvas::Result as CanvasResult;
pub use color::*;
pub use common::*;
pub use component::{
    Axis, Circle, Grid, Legend, LegendCategory, Line, Pie, Polygon, Polyline, Rect, SmoothLine,
    SmoothLineFill, StraightLine, StraightLineFill, Text,
};
#[cfg(feature = "image-encoder")]
pub(crate) use encoder::get_or_init_fontdb;
#[cfg(feature = "image-encoder")]
pub use encoder::Error as EncoderError;
#[cfg(feature = "image-encoder")]
pub use encoder::*;

pub use candlestick_chart::CandlestickChart;
pub use font::Error as FontError;
pub use font::{
    get_font, get_font_families, get_or_try_init_fonts, measure_text_width_family,
    DEFAULT_FONT_DATA, DEFAULT_FONT_FAMILY,
};
pub use heatmap_chart::{HeatmapChart, HeatmapData, HeatmapSeries};
pub use horizontal_bar_chart::HorizontalBarChart;
pub use line_chart::LineChart;
pub use multi_chart::{ChildChart, MultiChart};
pub use path::*;
pub use pie_chart::PieChart;
pub use radar_chart::{RadarChart, RadarIndicator};
pub use scatter_chart::ScatterChart;
pub use table_chart::{TableCellStyle, TableChart};
pub use theme::Theme;
pub use theme::{get_or_init_default_theme, THEME_ANT, THEME_DARK, THEME_GRAFANA};
pub use util::*;

pub trait Chart {
    fn fill_theme(&mut self, t: &Theme);
    fn fill_option(&mut self, data: &str) -> canvas::Result<serde_json::Value>;
    fn get_y_axis_config(&self, index: usize) -> YAxisConfig;
    fn get_y_axis_values(&self, y_axis_index: usize) -> (AxisValues, f32);
    fn render_background(&self, c: Canvas);
    fn render_title(&self, c: Canvas) -> f32;
    fn render_legend(&self, c: Canvas) -> f32;
    fn render_grid(&self, c: Canvas, axis_width: f32, axis_height: f32);
    fn render_y_axis(
        &self,
        c: Canvas,
        data: Vec<String>,
        axis_height: f32,
        axis_width: f32,
        axis_index: usize,
    );
    fn render_x_axis(&self, c: Canvas, data: Vec<String>, axis_width: f32);
    fn render_series_label(&self, c: Canvas, series_labels_list: Vec<Vec<SeriesLabel>>);
    fn render_bar(
        &self,
        c: Canvas,
        series_list: &[&Series],
        y_axis_values: &[&AxisValues],
        max_height: f32,
        series_data_count: usize,
    ) -> Vec<Vec<SeriesLabel>>;
    fn render_line(
        &self,
        c: Canvas,
        series_list: &[&Series],
        y_axis_values: &[&AxisValues],
        max_height: f32,
        axis_height: f32,
        series_data_count: usize,
    ) -> Vec<Vec<SeriesLabel>>;
}
