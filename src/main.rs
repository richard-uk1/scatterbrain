use xilem::core::adapt;
use xilem::dpi::LogicalSize;
use xilem::view::{button, checkbox, flex, flex_item, progress_bar, sized_box, Axis, FlexSpacer};
use xilem::winit::error::EventLoopError;
use xilem::winit::window::Window;
use xilem::{Color, EventLoop, EventLoopBuilder, WidgetView, Xilem};

const SPACER_WIDTH: f64 = 10.;

/// The state of the entire application.
///
/// This is owned by Xilem, used to construct the view tree, and updated by event handlers.
struct WidgetGallery {
    progress: Option<f64>,
    checked: bool,
}

fn progress_bar_view(data: Option<f64>) -> impl WidgetView<Option<f64>> {
    flex((
        progress_bar(data),
        checkbox(
            "set indeterminate progress",
            data.is_none(),
            |state: &mut Option<f64>, checked| {
                if checked {
                    *state = None;
                } else {
                    *state = Some(0.5);
                }
            },
        ),
        button("change progress", |state: &mut Option<f64>| match state {
            Some(ref mut v) => *v = (*v + 0.1).rem_euclid(1.),
            None => *state = Some(0.5),
        }),
    ))
}

fn checkbox_view(data: bool) -> impl WidgetView<bool> {
    checkbox("a simple checkbox", data, |data, new_state| {
        *data = new_state;
    })
}

/// Wrap `inner` in a box with a border
fn border_box<State: 'static, Action: 'static>(
    inner: impl WidgetView<State, Action>,
) -> impl WidgetView<State, Action> {
    sized_box(
        flex((
            FlexSpacer::Flex(1.),
            flex((FlexSpacer::Flex(1.), inner, FlexSpacer::Flex(1.))),
            FlexSpacer::Flex(1.),
        ))
        .direction(Axis::Horizontal),
    )
    .border(Color::WHITE, 2.)
    .width(450.)
    .height(200.)
}

/// Top-level view
fn app_logic(data: &mut WidgetGallery) -> impl WidgetView<WidgetGallery> {
    // Use a `sized_box` to pad the window contents
    sized_box(
        flex((
            adapt(
                flex_item(border_box(progress_bar_view(data.progress)), 1.),
                |data: &mut WidgetGallery, thunk| thunk.call(&mut data.progress),
            ),
            adapt(
                flex_item(border_box(checkbox_view(data.checked)), 1.),
                |data: &mut WidgetGallery, thunk| thunk.call(&mut data.checked),
            ),
        ))
        .gap(SPACER_WIDTH)
        .direction(Axis::Horizontal),
    )
    .border(Color::TRANSPARENT, SPACER_WIDTH)
}

fn run(event_loop: EventLoopBuilder) -> Result<(), EventLoopError> {
    // Set up the initial state of the app
    let data = WidgetGallery {
        progress: Some(0.5),
        checked: false,
    };

    // Instantiate and run the UI using the passed event loop.
    let app = Xilem::new(data, app_logic);
    let min_window_size = LogicalSize::new(300., 200.);
    let window_size = LogicalSize::new(650., 500.);
    let window_attributes = Window::default_attributes()
        .with_title("Xilem Widgets")
        .with_resizable(true)
        .with_min_inner_size(min_window_size)
        .with_inner_size(window_size);
    app.run_windowed_in(event_loop, window_attributes)?;
    Ok(())
}

fn main() -> Result<(), EventLoopError> {
    run(EventLoop::with_user_event())
}
