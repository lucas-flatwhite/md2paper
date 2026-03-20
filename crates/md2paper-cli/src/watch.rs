use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;

use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher, recommended_watcher};

/// Run a compile loop: compile once immediately, then recompile on every
/// write/create event. Calls `compile_fn` on each change.
/// Blocks until Ctrl+C (channel disconnect).
pub fn run_watch_loop<F>(paths: Vec<PathBuf>, mut compile_fn: F) -> anyhow::Result<()>
where
    F: FnMut() -> anyhow::Result<()>,
{
    // Initial compile
    run_and_report(&mut compile_fn);

    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher: RecommendedWatcher = recommended_watcher(tx)?;
    for path in &paths {
        watcher.watch(path, RecursiveMode::NonRecursive)?;
    }

    eprintln!("Watching for changes… (Ctrl+C to stop)");
    loop {
        match rx.recv_timeout(Duration::from_millis(50)) {
            Ok(Ok(event)) if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) => {
                // Debounce: drain burst of events before recompiling
                while rx.recv_timeout(Duration::from_millis(50)).is_ok() {}
                run_and_report(&mut compile_fn);
            }
            Ok(Ok(_)) => {}
            Ok(Err(e)) => eprintln!("Watch error: {e}"),
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }
    Ok(())
}

pub(crate) fn run_and_report<F: FnMut() -> anyhow::Result<()>>(f: &mut F) {
    match f() {
        Ok(()) => eprintln!("[ok] Compiled successfully"),
        Err(e) => eprintln!("[error] {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_and_report_calls_closure() {
        let mut called = false;
        run_and_report(&mut || { called = true; Ok(()) });
        assert!(called);
    }

    #[test]
    fn test_run_and_report_on_error_does_not_panic() {
        run_and_report(&mut || anyhow::bail!("simulated error"));
    }
}
