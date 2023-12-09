#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_app() {
        let app = App::new();
        // Assert that the event loop and window are created successfully
        assert!(app.event_loop.is_running());
        assert!(app.window.id() != winit::window::WindowId::dangling());
    }

    #[test]
    fn test_run_app() {
        let app = App::new();
        // Simulate a close requested event
        let close_requested_event = Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id: app.window.id(),
        };
        let mut control_flow = ControlFlow::Poll;
        app.run_event(close_requested_event, &mut control_flow);
        // Assert that the control flow is set to Exit
        assert_eq!(control_flow, ControlFlow::Exit);
    }
}