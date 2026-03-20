use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::watch::run_watch_loop;

const DEFAULT_PORT: u16 = 4321;

/// State shared between HTTP handler threads and the watch loop.
struct PreviewState {
    pdf: Vec<u8>,
    gen: u64,
}

/// Start the preview HTTP server and file watch loop.
/// `compile_fn` is called on every file change; returns fresh PDF bytes.
pub fn run_preview<F>(paths: Vec<std::path::PathBuf>, mut compile_fn: F) -> anyhow::Result<()>
where
    F: FnMut() -> anyhow::Result<Vec<u8>> + Send + 'static,
{
    let state = Arc::new(Mutex::new(PreviewState { pdf: vec![], gen: 0 }));

    // Initial compile
    match compile_fn() {
        Ok(bytes) => { let mut s = state.lock().unwrap(); s.pdf = bytes; s.gen = 1; }
        Err(e) => eprintln!("[error] {e}"),
    }

    // HTTP server on background thread
    let state_srv = Arc::clone(&state);
    thread::spawn(move || {
        let listener = TcpListener::bind(("127.0.0.1", DEFAULT_PORT))
            .expect("failed to bind preview server port");
        eprintln!("Preview: http://localhost:{DEFAULT_PORT}");
        for stream in listener.incoming().flatten() {
            let s = Arc::clone(&state_srv);
            thread::spawn(move || handle(stream, s));
        }
    });

    // Open browser (best-effort, errors ignored)
    open_browser(&format!("http://localhost:{DEFAULT_PORT}"));

    // Watch loop — runs on this thread, blocks until Ctrl+C
    let state_watch = Arc::clone(&state);
    run_watch_loop(paths, move || {
        let bytes = compile_fn()?;
        let mut s = state_watch.lock().unwrap();
        s.gen += 1;
        s.pdf = bytes;
        Ok(())
    })
}

fn handle(mut stream: TcpStream, state: Arc<Mutex<PreviewState>>) {
    let mut buf = [0u8; 4096];
    let n = stream.read(&mut buf).unwrap_or(0);
    let req = String::from_utf8_lossy(&buf[..n]);
    let path = req.lines().next()
        .and_then(|l| l.split_whitespace().nth(1))
        .unwrap_or("/");

    if path.starts_with("/pdf") {
        let pdf = state.lock().unwrap().pdf.clone();
        let _ = write!(stream,
            "HTTP/1.1 200 OK\r\nContent-Type: application/pdf\r\n\
             Content-Length: {}\r\nCache-Control: no-cache\r\n\r\n", pdf.len());
        let _ = stream.write_all(&pdf);
    } else if path.starts_with("/gen") {
        let gen = state.lock().unwrap().gen.to_string();
        let _ = write!(stream,
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\
             Content-Length: {}\r\nCache-Control: no-cache\r\n\r\n{}", gen.len(), gen);
    } else {
        let html = preview_html();
        let _ = write!(stream,
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\
             Content-Length: {}\r\n\r\n{}", html.len(), html);
    }
}

/// Open URL with the platform-appropriate command.
fn open_browser(url: &str) {
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg(url).status();
    #[cfg(target_os = "linux")]
    let _ = std::process::Command::new("xdg-open").arg(url).status();
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("cmd").args(["/c", "start", url]).status();
}

fn preview_html() -> String {
    format!(r#"<!DOCTYPE html>
<html>
<head>
  <title>md2paper preview</title>
  <style>
    * {{ margin:0; padding:0; box-sizing:border-box; }}
    body {{ background:#333; display:flex; flex-direction:column; height:100vh; }}
    #bar {{ background:#1a1a1a; color:#aaa; padding:6px 12px; font:12px monospace;
            display:flex; justify-content:space-between; align-items:center; }}
    #status {{ color:#4caf50; }}
    embed {{ flex:1; width:100%; border:none; }}
  </style>
</head>
<body>
  <div id="bar">
    <span>md2paper preview</span>
    <span id="status">loading…</span>
  </div>
  <embed id="pdf" type="application/pdf" src="/pdf">
  <script>
    let gen = 0;
    async function poll() {{
      try {{
        const r = await fetch('/gen');
        const g = await r.text();
        if (g !== String(gen)) {{
          gen = g;
          document.getElementById('pdf').src = '/pdf?v=' + g;
          document.getElementById('status').textContent = 'updated ' + new Date().toLocaleTimeString();
        }}
      }} catch(e) {{}}
      setTimeout(poll, 800);
    }}
    poll();
  </script>
</body>
</html>"#)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preview_html_is_not_empty() {
        let html = preview_html();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("/pdf"));
        assert!(html.contains("/gen"));
    }

    #[test]
    fn test_open_browser_does_not_panic() {
        // Just confirms no panic — browser may or may not open in CI
        open_browser("http://localhost:4321");
    }
}
