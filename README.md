# Ray Tracing in one Weekend

This is my attempt at Ray Tracing in one Weekend book by Peter Shirley.


I use a SDL2 window to draw rendered image. Each key on Numbers row is mapped to one of the demos.

So, You can press keys from 1-0, -, =(12 keys) to run 12 demos.



# Some Notes

I only added a parallelized renderer. The image is divided into 64 rectangular chunks and each chunk is rendered in parallel.


Right now, Each chunk writes to it's local buffer and that buffer is copied to the correct position in the main frame buffer. I could've avoided this copy with a Arc<Mutex<_>> wrapper but that would have resulted in lock contention among all the threads effectively serializing the whole thing. 


I also wanted to draw a chunk to the canvas/window as soon as it was computed so parts of the image will show up early instead of it all taking a bit more time and showing up all at once but that was more complicated because, 

1. I couldn't send `SDL2_Texture` away from main thread since that is unsafe/results in undefined behavior. `SDL2_Texture` represents a GPU Texture and from the Docs plus from what I was told, It is unsafe to send it off to other threads. I still don't get why is it unsafe to do that and I couldn't get a satisfactory answer anywhere. :(
 
2. So I tried to figure out an alternative approach. To do this, I would have to, Create a MPSC Channel, Spawn a thread, Run the `render_chunk` method in this thread and write the local buffer from each `Chunk` to aforementioned MPSC channel. Then, In the main thread, I could take data from the MPSC channel, Copy it into correct position in the main frame buffer, Update texture and present it.

But of course, it's not easy. Programmatically, It would look something like this. 

```rust
fn render(&self, buf: &mut vec<u8>, width: usize, height: usize, samples: u8) {
    let nx = width / VERTICAL_PARTITION;
    let ny = height / HORIZONTAL_PARTITION;
    let (tx, rx) = mpsc::channel();

    let tx = Arc::new(Mutex::new(tx));
    let v = thread::spawn(move || {
        (0..VERTICAL_PARTITION).into_par_iter().for_each(move |j| {
            let tx = tx.clone();
            (0..HORIZONTAL_PARTITION)
                .into_par_iter()
                .for_each(move |i| {
                    let world = self.world();
                    let camera = self.camera(nx as f32 / ny as f32);

                    let start_y = j * ny;
                    let start_x = i * nx;
                    let x = width;
                    let y = height;
                    let mut chunk = chunk {
                        x,
                        y,
                        nx,
                        ny,
                        start_x,
                        start_y,
                        buffer: vec![0; nx * ny * 4],
                    };
                    self.render_chunk(&mut chunk, camera.as_ref(), world.as_ref(), samples);

                    let tx = tx.lock().unwrap();
                    tx.send(chunk);
                })
        });
    });
    for mut chunk in rx {
        let chunk {
            x,
            y,
            nx,
            ny,
            start_x,
            start_y,
            ref mut buffer,
        } = chunk;
        let mut temp_offset = 0;
        for j in start_y..start_y + ny {
            let real_offset = ((y - j - 1) * x + start_x) * 4;

            buf[real_offset..real_offset + nx * 4]
                .copy_from_slice(&chunk.buffer[temp_offset..temp_offset + nx * 4]);

            temp_offset += nx * 4;
        }
    }
    v.join().unwrap();
}
```

There is really only one problem here. `thread::spawn` requires the objects it access to have `'static` lifetime. Since, I am calling `self.render_chunk`, It wants `self` to have `'static` lifetime which is not really possible because of the render loop and how rest of the program is structured. Maybe making `active_demo` in `src/main.rs` a `static mut` and changing `render` signature to `fn render(&'static self)` can help? I don't know. I am just tired and didn't tried that plus, that would require me to use unsafe to update `active_demo` which is probably not a problem but something to consider.

Also, `rustc` is not yet smart enough to ease up on the lifetime a little bit if I call the join handle on a thread early. So For example, In this code, 

```rust
fn potato(&self) {
    let handle = thread::spawn(move || {
        self.do_stuff();
    });
    handle.join().unwrap();
}
```

It really doesn't need `self` to have `'static` lifetime. `self` only needs to live until I call `.join()` on the handle. So, Logically it makes sense but `rustc` can't figure this out yet so it'll ask `'static'` lifetime on `self` anyway.

One other approach that was suggested to me was to use crossbeam threads instead of `std::thread` but I didn't try this approach.

Soo Yeah, I give up on this present as soon as you have a block approach.


# Demo Renders




![[1] Simple Rectangle](https://dl.ishanjain.me/images/simple_rectangle-2000x1000.png)
[1] Simple Rectangle                                               

![[2] Linear Gradient Rectangle](https://dl.ishanjain.me/images/linear-gradient-rectangle-2000x1000.png)
[2] Linear Gradient Rectangle

![[3] Simple Sphere](https://dl.ishanjain.me/images/simple_sphere-2000x1000.png)
[3] Simple Sphere

![[4] Surface Normal Sphere](https://dl.ishanjain.me/images/surface_normal_sphere-2000x1000.png)
[4] Surface Normal Sphere

![[5] Sphere using Hit Table](https://dl.ishanjain.me/images/sphere-using-hit-table-2000x1000.png)
[5] Sphere using Hit Table

![[6] Simple Anti-aliased Circle](https://dl.ishanjain.me/images/simple-antialiasing-2000x1000.png)
[6] Simple Anti-aliased Circle

![[7] Diffuse Materials](https://dl.ishanjain.me/images/diffuse-materials-2000x1000.png)
[7] Diffuse Materials

![[8] Materials](https://dl.ishanjain.me/images/materials-2000x1000.png)
[8] Materials

![[9] Dielectric Material](https://dl.ishanjain.me/images/dielectric-material-2000x1000.png)
[9] Dielectric Material

![[10] Positionable Camera](https://dl.ishanjain.me/images/positionable-camera-2000x1000.png)
[10] Positionable Camera

![[11] Defocus Blur(DOF)](https://dl.ishanjain.me/images/defocus-blur-2000x1000.png)
[11] Defocus Blur(DOF)

![[12] Final Scene](https://dl.ishanjain.me/images/final-scene-2000x1000.png)
[12] Final Scene
