use tracing_error::ExtractSpanTrace;

pub fn display_tracing_error(err: &theseus::Error) {
    match get_span_trace(err) {
        Some(span_trace) => {
            tracing::error!(error = %err, span_trace = %span_trace);
        }
        None => {
            tracing::error!(error = %err);
        }
    }
}

pub fn get_span_trace<'a>(
    error: &'a (dyn std::error::Error + 'static),
) -> Option<&'a tracing_error::SpanTrace> {
    error.source().and_then(|e| e.span_trace())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    #[derive(Debug)]
    struct SimpleError;

    impl fmt::Display for SimpleError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "simple error")
        }
    }

    impl std::error::Error for SimpleError {}

    #[test]
    fn test_get_span_trace_returns_none_for_simple_error() {
        // Test that get_span_trace returns None for errors without span trace
        let error = SimpleError;
        let span_trace = get_span_trace(&error);
        assert!(span_trace.is_none());
    }
}
