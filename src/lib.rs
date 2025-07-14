use polars::prelude::*;
use plotters::prelude::*;
use std::error::Error;

#[cfg(feature = "interactive")]
use charming::{
    component::{Axis, Title},
    element::AxisType,
    series::{Bar, Line, Scatter, Series},
    Chart as CharmingChart,
};

#[derive(Clone)]
pub enum ChartType {
    Bar,
    Line,
    Scatter,
}

#[derive(Clone)]
pub struct Chart {
    chart_type: ChartType,
    dataframe: DataFrame,
    x_column: String,
    y_column: String,
}

pub fn barchart(dataframe: DataFrame) -> Chart {
    Chart {
        chart_type: ChartType::Bar,
        dataframe,
        x_column: String::new(),
        y_column: String::new(),
    }
}

pub fn linechart(dataframe: DataFrame) -> Chart {
    Chart {
        chart_type: ChartType::Line,
        dataframe,
        x_column: String::new(),
        y_column: String::new(),
    }
}

pub fn scatterplot(dataframe: DataFrame) -> Chart {
    Chart {
        chart_type: ChartType::Scatter,
        dataframe,
        x_column: String::new(),
        y_column: String::new(),
    }
}

impl Chart {
    pub fn x(mut self, column_name: &str) -> Self {
        self.x_column = column_name.to_string();
        self
    }

    pub fn y(mut self, column_name: &str) -> Self {
        self.y_column = column_name.to_string();
        self
    }

    pub fn draw(&self, path: &str) -> std::result::Result<(), Box<dyn Error>> {
        let root = BitMapBackend::new(path, (640, 480)).into_drawing_area();
        root.fill(&WHITE)?;

        println!("Drawing static chart with x='{}' and y='{}'", self.x_column, self.y_column);

        root.present()?;
        Ok(())
    }
}

#[cfg(feature = "interactive")]
impl Chart {
    pub fn to_interactive_html(&self) -> std::result::Result<String, Box<dyn Error>> {
        let x_data: Vec<String> = self
            .dataframe
            .column(&self.x_column)?
            .str()?
            .into_iter()
            .map(|opt_val| opt_val.unwrap_or("").to_string())
            .collect();

        let y_data: Vec<f64> = self
            .dataframe
            .column(&self.y_column)?
            .cast(&DataType::Float64)?
            .f64()?
            .into_iter()
            .map(|opt_val| opt_val.unwrap_or(0.0))
            .collect();

        let series: Series = match self.chart_type {
            ChartType::Bar => Bar::new().data(y_data).into(),
            ChartType::Line => Line::new().data(y_data).into(),
            ChartType::Scatter => Scatter::new().data(y_data).into(),
        };

        let chart = CharmingChart::new()
            .title(Title::new().text("fluent-plots chart"))
            .x_axis(Axis::new().type_(AxisType::Category).data(x_data))
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(series);

        let html = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset="utf-8" />
                <title>Fluent-Plots Chart</title>
                <script src="https://cdn.jsdelivr.net/npm/echarts@5.5.0/dist/echarts.min.js"></script>
            </head>
            <body>
                <div id="main" style="width: 800px;height:600px;"></div>
                <script type="text/javascript">
                    var myChart = echarts.init(document.getElementById('main'));
                    var option = {json_config};
                    myChart.setOption(option);
                </script>
            </body>
            </html>
            "#,
            json_config = chart.to_string()
        );

        Ok(html)
    }
}
