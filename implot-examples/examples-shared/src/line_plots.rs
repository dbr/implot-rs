//! This example demonstrates how line plots are to be used, along with some querying features
//! that will be applicable to all kinds of plots.

use imgui::{CollapsingHeader, Condition, Ui};
use implot::{
    get_plot_limits, get_plot_mouse_position, is_legend_entry_hovered,
    is_plot_hovered, pixels_to_plot_vec2, plot_to_pixels_vec2, push_style_color,
    push_style_var_f32, push_style_var_i32,
    AxisFlags, Colormap, ImPlotPoint, ImPlotRange, ImVec2, ImVec4,
    Marker, Plot, PlotColorElement, PlotFlags, PlotLine, PlotLocation, PlotUi,
    StyleVar, YAxisChoice,
};

use std::{cell::RefCell, rc::Rc};

/// State of the line plots demo.
pub struct LinePlotDemoState {
    linked_limits: Rc<RefCell<ImPlotRange>>,
}

impl LinePlotDemoState {
    /// Create a new line plots demo state object with default values in it.
    pub fn new() -> Self {
        Self {
            linked_limits: Rc::new(RefCell::new(ImPlotRange { Min: 0.0, Max: 1.0 })),
        }
    }

    pub fn show_basic_plot(ui: &Ui, plot_ui: &PlotUi) {
        ui.text("This header just plots a line with as little code as possible.");
        let content_width = ui.window_content_region_width();
        Plot::new("Simple line plot")
            // The size call could also be omitted, though the defaults don't consider window
            // width, which is why we're not doing so here.
            .size([content_width, 300.0])
            .build(plot_ui, || {
                // If this is called outside a plot build callback, the program will panic.
                let x_positions = vec![0.1, 0.9];
                let y_positions = vec![0.1, 0.9];
                PlotLine::new("legend label").plot(&x_positions, &y_positions);
            });
    }

    pub fn show_two_yaxis_plot(ui: &Ui, plot_ui: &PlotUi) {
        ui.text("This header shows how to create a plot with multiple Y axes.");
        let content_width = ui.window_content_region_width();
        Plot::new("Multiple Y axis plots")
            // The size call could also be omitted, though the defaults don't consider window
            // width, which is why we're not doing so here.
            .size([content_width, 300.0])
            .y_limits(
                ImPlotRange { Min: 0.0, Max: 1.0 },
                YAxisChoice::First,
                Condition::Always,
            )
            .y_limits(
                // One can also use [f32; 2], (f32, f32) and ImVec2 for limit setting
                [1.0, 3.5],
                YAxisChoice::Second,
                Condition::Always,
            )
            .build(plot_ui, || {
                let x_positions = vec![0.1, 0.9];

                // The first Y axis is the default
                let y_positions = vec![0.1, 0.9];
                PlotLine::new("legend label").plot(&x_positions, &y_positions);

                let y_positions = vec![3.3, 1.2];
                PlotLine::new("legend label two").plot(&x_positions, &y_positions);
            });
    }

    pub fn show_axis_equal_plot(ui: &Ui, plot_ui: &PlotUi) {
        ui.text("This plot has axis equal set (1:1 aspect ratio).");
        let content_width = ui.window_content_region_width();
        Plot::new("Axis equal line plot")
            // The size call could also be omitted, though the defaults don't consider window
            // width, which is why we're not doing so here.
            .size([content_width, 300.0])
            .with_plot_flags(&(PlotFlags::NONE | PlotFlags::AXIS_EQUAL))
            .build(plot_ui, || {
                // If this is called outside a plot build callback, the program will panic.
                let x_positions = vec![0.1, 0.9];
                let y_positions = vec![0.1, 0.9];
                PlotLine::new("legend label").plot(&x_positions, &y_positions);
            });
    }

    pub fn show_configurable_plot(ui: &Ui, plot_ui: &PlotUi) {
        ui.text("This header demos what we can configure about plots.");

        // Settings for the plot
        // - X and Y size in pixels
        let x_size = 300.0;
        let y_size = 200.0;
        // - Strings for the axis labels
        let x_label = "X label!";
        let y_label = "Y label!";
        // - Plot limits
        let x_min = 2.0;
        let x_max = 3.0;
        let y_min = 1.0;
        let y_max = 2.0;
        // - Plot flags, see the PlotFlags docs for more info
        let plot_flags = PlotFlags::NONE;
        // - Axis flags, see the AxisFlags docs for more info. All flags are bitflags-created,
        //   so they support a bunch of convenient operations, see https://docs.rs/bitflags
        let x_axis_flags = AxisFlags::NONE;
        let y_axis_flags = AxisFlags::NONE;

        // - Unlabelled X axis ticks
        let x_ticks = vec![2.2, 2.5, 2.8];

        // - Labelled Y axis ticks
        let y_ticks = vec![(1.1, "A".to_owned()), (1.4, "B".to_owned())];

        // Axis labels
        Plot::new("Configured line plot")
            .size([x_size, y_size])
            .x_label(x_label)
            .y_label(y_label)
            .x_limits(
                ImPlotRange {
                    Min: x_min,
                    Max: x_max,
                },
                // Always means that the limits stay what we force them to here, even if the user
                // scrolls or drags in the plot with the mouse. FirstUseEver sets the limits the
                // first time the plot is drawn, but the user can then modify them and the change
                // will stick.
                Condition::Always,
            )
            .y_limits(
                ImPlotRange {
                    Min: y_min,
                    Max: y_max,
                },
                YAxisChoice::First,
                Condition::Always,
            )
            .x_ticks(&x_ticks, false)
            .y_ticks_with_labels(YAxisChoice::First, &y_ticks, false)
            // If any of these flag setting calls are omitted, the defaults are used.
            .with_plot_flags(&plot_flags)
            .with_x_axis_flags(&x_axis_flags)
            .with_y_axis_flags(YAxisChoice::First, &y_axis_flags)
            .build(plot_ui, || {
                PlotLine::new("A line 2").plot(&[2.4, 2.9], &[1.1, 1.9]);
            });
    }

    pub fn show_query_features_plot(ui: &Ui, plot_ui: &PlotUi) {
        ui.text("This header demos how to use the querying features.");
        let content_width = ui.window_content_region_width();

        // Create some containers for exfiltrating data from the closure below
        let mut hover_pos_plot: Option<ImPlotPoint> = None;
        let mut hover_pos_pixels: Option<ImVec2> = None;
        let mut hover_pos_from_pixels: Option<ImPlotPoint> = None;
        let mut legend1_hovered = false;
        let mut legend2_hovered = false;

        // Draw a plot
        Plot::new("Plot querying")
            .size([content_width, 300.0])
            .x_limits(ImPlotRange { Min: 0.0, Max: 5.0 }, Condition::FirstUseEver)
            .y_limits(
                ImPlotRange { Min: 0.0, Max: 5.0 },
                YAxisChoice::First,
                Condition::FirstUseEver,
            )
            .build(plot_ui, || {
                if is_plot_hovered() {
                    hover_pos_plot = Some(get_plot_mouse_position(None));
                    hover_pos_pixels = Some(plot_to_pixels_vec2(&(hover_pos_plot.unwrap()), None));
                }

                // Getting the plot position from pixels also works when the plot is not hovered,
                // the coordinates are then simply outside the visible range.
                hover_pos_from_pixels = Some(pixels_to_plot_vec2(
                    &ImVec2 {
                        x: ui.io().mouse_pos[0],
                        y: ui.io().mouse_pos[1],
                    },
                    None,
                ));

                // Plot a line so we have a legend entry
                PlotLine::new("Legend1").plot(&[2.0, 2.0], &[2.0, 1.0]);
                PlotLine::new("Legend2").plot(&[0.0, 0.0], &[1.0, 1.0]);
                legend1_hovered = is_legend_entry_hovered("Legend1");
                legend2_hovered = is_legend_entry_hovered("Legend2");
            });

        // Print some previously-exfiltrated info. This is because calling
        // things like is_plot_hovered or get_plot_mouse_position() outside
        // of an actual Plot is not allowed.
        if let Some(pos) = hover_pos_plot {
            ui.text(format!("hovered at {}, {}", pos.x, pos.y));
        }
        if let Some(pixel_position) = hover_pos_pixels {
            // Try out converting plot mouse position to pixel position
            ui.text(format!(
                "pixel pos from plot:  {}, {}",
                pixel_position.x, pixel_position.y
            ));
            ui.text(format!(
                "pixel pos from imgui: {}, {}",
                ui.io().mouse_pos[0],
                ui.io().mouse_pos[1]
            ));
        }
        ui.text(format!(
            "Legend hovering - 1: {}, 2: {}",
            legend1_hovered, legend2_hovered
        ));

        // Try out converting pixel position to plot position
        if let Some(pos) = hover_pos_from_pixels {
            ui.text(format!("plot pos from imgui: {}, {}", pos.x, pos.y,));
        }
    }

    pub fn show_style_plot(ui: &Ui, plot_ui: &PlotUi) {
        ui.text("This header demos how to use the styling features.");
        let content_width = ui.window_content_region_width();

        // The style stack works the same as for other imgui things - we can push
        // things to have them apply, then pop again to undo the change. In implot-rs,
        // pushing returns a value on which we have to call .pop() later. Pushing
        // variables can be done outside of plot calls as well.
        let style = push_style_color(&PlotColorElement::PlotBg, 1.0, 1.0, 1.0, 0.2);
        Plot::new("Style demo plot")
            .size([content_width, 300.0])
            .x_limits(ImPlotRange { Min: 0.0, Max: 6.0 }, Condition::Always)
            .y_limits(
                ImPlotRange {
                    Min: -1.0,
                    Max: 3.0,
                },
                YAxisChoice::First,
                Condition::Always,
            )
            .with_plot_flags(&(PlotFlags::NONE))
            .with_y_axis_flags(YAxisChoice::First, &(AxisFlags::NONE))
            .build(plot_ui, || {
                // Markers can be selected as shown here. The markers are internally represented
                // as an u32, hence this calling style.
                let markerchoice = push_style_var_i32(&StyleVar::Marker, Marker::Cross as i32);
                PlotLine::new("Left eye").plot(&[2.0, 2.0], &[2.0, 1.0]);
                // Calling pop() on the return value of the push above will undo the marker choice.
                markerchoice.pop();

                // Line weights can be set the same way, along with some other things - see
                // the docs of StyleVar for more info.
                let lineweight = push_style_var_f32(&StyleVar::LineWeight, 5.0);
                PlotLine::new("Right eye").plot(&[4.0, 4.0], &[2.0, 1.0]);
                lineweight.pop();

                let x_values = vec![1.0, 2.0, 4.0, 5.0];
                let y_values = vec![1.0, 0.0, 0.0, 1.0];
                PlotLine::new("Mouth").plot(&x_values, &y_values);
            });

        style.pop();
    }

    pub fn show_colormaps_plot(ui: &Ui, plot_ui: &PlotUi) {
        ui.text("This header demos how to select colormaps.");
        let content_width = ui.window_content_region_width();

        Plot::new("Colormap demo plot")
            .size([content_width, 300.0])
            .build(plot_ui, || {
                (1..10)
                    .map(|x| x as f64 * 0.1)
                    .map(|x| PlotLine::new(&format!("{:3.3}", x)).plot(&[0.1, 0.9], &[x, x]))
                    .count();
            });

        Plot::new("Colormap demo plot #2")
            .size([content_width, 300.0])
            .build(plot_ui, || {
                (1..10)
                    .map(|x| x as f64 * 0.1)
                    .map(|x| PlotLine::new(&format!("{:3.3}", x)).plot(&[0.1, 0.9], &[x, x]))
                    .count();
            });

    }

    pub fn show_conversions_plot(ui: &Ui, plot_ui: &PlotUi) {
        ui.text("This header demonstrates (in code) how to convert various ranges into ImRange");
        let content_width = ui.window_content_region_width();
        Plot::new("Simple line plot, conversion 1")
            .size([content_width, 300.0])
            .x_limits(ImVec2 { x: 0.0, y: 1.0 }, Condition::Always)
            .y_limits([0.0, 1.0], YAxisChoice::First, Condition::Always)
            .build(plot_ui, || {
                // If this is called outside a plot build callback, the program will panic.
                let x_positions = vec![0.1, 0.9];
                let y_positions = vec![0.1, 0.9];
                PlotLine::new("legend label").plot(&x_positions, &y_positions);
            });
    }

    pub fn show_linked_x_axis_plots(&mut self, ui: &Ui, plot_ui: &PlotUi) {
        ui.text("These plots have their X axes linked, but not the Y axes");
        let content_width = ui.window_content_region_width();
        Plot::new("Linked plot 1")
            .size([content_width, 300.0])
            .linked_x_limits(self.linked_limits.clone())
            .build(plot_ui, || {
                let x_positions = vec![0.1, 0.9];
                let y_positions = vec![0.1, 0.9];
                PlotLine::new("legend label").plot(&x_positions, &y_positions);
            });
        Plot::new("Linked plot 2")
            .size([content_width, 300.0])
            .linked_x_limits(self.linked_limits.clone())
            .build(plot_ui, || {
                let x_positions = vec![0.1, 0.9];
                let y_positions = vec![0.1, 0.9];
                PlotLine::new("legend label").plot(&x_positions, &y_positions);
            });
    }

    pub fn show_demo_headers(&mut self, ui: &Ui, plot_ui: &PlotUi) {
        if CollapsingHeader::new("Line plot: Basic").build(ui) {
            Self::show_basic_plot(ui, plot_ui);
        }
        if CollapsingHeader::new("Line plot: Configured").build(ui) {
            Self::show_configurable_plot(ui, plot_ui);
        }
        if CollapsingHeader::new("Line Plot: Plot queries").build(ui) {
            Self::show_query_features_plot(ui, plot_ui);
        }
        if CollapsingHeader::new("Line plot: Plot styling").build(ui) {
            Self::show_style_plot(ui, plot_ui);
        }
        if CollapsingHeader::new("Line plot: Colormaps").build(ui) {
            Self::show_colormaps_plot(ui, plot_ui);
        }
        if CollapsingHeader::new("Line plot: Multiple Y Axes").build(ui) {
            Self::show_two_yaxis_plot(ui, plot_ui);
        }
        if CollapsingHeader::new("Line plot: \"Axis equal\"").build(ui) {
            Self::show_axis_equal_plot(ui, plot_ui);
        }
        if CollapsingHeader::new("Line plot: Range conversions").build(ui) {
            Self::show_conversions_plot(ui, plot_ui);
        }
        if CollapsingHeader::new("Line plot: Linked plots").build(ui) {
            self.show_linked_x_axis_plots(ui, plot_ui);
        }
    }
}

impl Default for LinePlotDemoState {
    fn default() -> Self {
        Self::new()
    }
}
