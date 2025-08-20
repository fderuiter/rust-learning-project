# Performance Profiling

This document describes the methodology for profiling the performance of the WebAssembly application.

## Serving the Release Build

To profile the application, you must first create and serve a release build.

1.  **Build the application for release:**
    ```bash
    make build-release
    ```

2.  **Serve the application locally:**
    You can use the `trunk serve` command or any static file server.
    ```bash
    trunk serve --release
    ```
    Or, using a simple Python server:
    ```bash
    cd dist
    python3 -m http.server
    ```

## Profiling with Browser DevTools

Once the application is running, you can use your browser's developer tools to profile its performance.

1.  **Open DevTools:** In most browsers, you can press `F12` or `Ctrl+Shift+I` (`Cmd+Option+I` on Mac) to open the developer tools.

2.  **Performance Tab:** Navigate to the "Performance" tab.

3.  **Record a Profile:** Click the "Record" button to start profiling. Interact with the application to capture data for the features you want to analyze (e.g., dragging the mesh, applying filters). Click "Stop" to finish recording.

4.  **Analyze the Profile:** The performance profile will show you a flame graph of function calls, a timeline of events, and other useful information. Look for:
    -   **Frame Render Times:** Aim for a stable ~60 FPS (frames per second). The frame rate chart will help you identify dropped frames.
    -   **Memory Usage:** Use the "Memory" tab to take heap snapshots and analyze memory allocations. Ensure that memory usage remains within acceptable bounds and that there are no memory leaks.

## Adjusting Parameters

If profiling reveals performance bottlenecks, consider adjusting physics parameters or rendering settings in the code to improve performance. Re-run the profiler after making changes to measure their impact.
