use memory::types::FloatType;
use plotters::prelude::*;
use plotters_backend::DrawingBackend;
use polars::{
    prelude::{AnyValue},
    series::Series,
};
use std::error::Error;

use super::backend::{PixelState, TextDrawingBackend};

pub struct PlotContext {
    backend: DrawingArea<TextDrawingBackend, plotters::coord::Shift>,
    caption: Option<String>,
    x_bounds: Option<(FloatType, FloatType)>,
    y_bounds: Option<(FloatType, FloatType)>,
}

impl PlotContext {
    pub fn new() -> Self {
        PlotContext {
            backend: TextDrawingBackend(vec![PixelState::Empty; 5000]).into_drawing_area(),
            caption: None,
            x_bounds: None,
            y_bounds: None,
        }
    }

    fn unwrap_bounds(
        &self,
        bounds: &Option<(FloatType, FloatType)>,
        series: &Series,
    ) -> (FloatType, FloatType) {
        if let Some((min, max)) = bounds {
            (*min, *max)
        } else {
            (
                series.min::<FloatType>().unwrap(),
                series.max::<FloatType>().unwrap(),
            )
        }
    }

    fn _draw_scatter<DB: DrawingBackend>(
        &self,
        backend: &DrawingArea<DB, plotters::coord::Shift>,
        x_series: &Series,
        y_series: &Series,
    ) -> Result<(), Box<dyn Error>>
    where
        DB::ErrorType: 'static,
    {
        let (x_min, x_max) = self.unwrap_bounds(&self.x_bounds, &x_series);

        let (y_min, y_max) = self.unwrap_bounds(&self.x_bounds, &y_series);

        let mut chart = ChartBuilder::on(backend)
            .margin(1)
            .caption(
                self.caption.clone().unwrap_or(String::new()),
                ("sans-serif", (10).percent_height()),
            )
            .set_label_area_size(LabelAreaPosition::Left, (5i32).percent_width())
            .set_label_area_size(LabelAreaPosition::Bottom, (10i32).percent_height())
            .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

        let series: Vec<(FloatType, FloatType)> = std::iter::zip(
            x_series
                .cast(&polars::prelude::DataType::Float64)
                .unwrap()
                .iter(),
            // .f64()
            // .unwrap()
            y_series
                .cast(&polars::prelude::DataType::Float64)
                .unwrap()
                .iter(),
        )
        .filter(|(a, b)| {
            if let AnyValue::Float64(a) = a {
                if let AnyValue::Float64(b) = b {
                    return true;
                }
            }
            return false;
        })
        .map(|(a, b)| {
            if let AnyValue::Float64(a) = a {
                if let AnyValue::Float64(b) = b {
                    return (a, b);
                }
            }
            panic!();
        })
        .collect();

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()?;

        chart.draw_series(series.iter().map(|(x, y)| Circle::new((*x, *y), 1, RED)))?;

        backend.present()?;

        Ok(())
    }

    pub fn draw_scatter<DB: DrawingBackend>(
        &self,
        x_series: &Series,
        y_series: &Series,
    ) -> Result<(), Box<dyn Error>>
    where
        DB::ErrorType: 'static,
    {
        self._draw_scatter(&self.backend, x_series, y_series)?;
        let b = BitMapBackend::new("chart.png", (1024, 768)).into_drawing_area();
        b.fill(&WHITE)?;
        self._draw_scatter(&b, x_series, y_series)?;

        Ok(())
    }

    pub fn set_caption(&mut self, caption: String) {
        self.caption = Some(caption);
    }

    pub fn set_x_bounds(&mut self, x_bounds: (FloatType, FloatType)) {
        self.x_bounds = Some(x_bounds);
    }

    pub fn set_y_bounds(&mut self, y_bounds: (FloatType, FloatType)) {
        self.y_bounds = Some(y_bounds);
    }
}
