# Rustracer
Another Rust implementation of the famous [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) series.  
By Peter Shirley, Trevor David Black, Steve Hollasch - Version 4.0.0-alpha.1 - Book 1.

Commit messages match sections of the book.

If you have FFmpeg installed, you can run
``` bash
cargo run && ffmpeg -i out.ppm -y out.png
```
in order to run the render and convert the file to png.