## Ray tracing in one weekend, in Rust

![Sansara](https://github.com/alexwheezy/ray-tracing-in-one-weekend/blob/main/render/final_render_01.png)

###### This image was rendered with the following settings:
    Height: 720
    Width: 1280
    Samples per pixel: 500
    Maximum depth: 50


This is an implementation in Rust of [Peter Shirley's "Ray Tracing In One Weekend"](https://raytracing.github.io/books/RayTracingInOneWeekend.html) book.

I know I'm not the first to reimplement code from this tutorial, but I tried to improve the modularity of this code a
bit and make it more idiomatic for this language. Nevertheless, it was a useful experience. Not all commits strictly
correspond to work stages.

You can simply clone `git clone` the repository to yourself and yourself and start rendering with a simple command:
`cargo run --release > spheres.ppm`

#### TODO:
- [ ] Ability to set render parameters from the command line.
- [ ] Add lighting sources.
- [ ] Add the ability to use textures.
