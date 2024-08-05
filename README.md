# Rust Simple 3D renderer
This simple 3d renderer project is based upon the C-lang course of Pikuma, but done in Rust instead of C.

The motivation for this project was as a learning project for Rust, along with testing the idea that Rust's semi-famous learning curve difficulties could possibly be reduced by sticking to a simpler "C like" subset of Rust language features, and not using the more advanced "C++ like" object oriented features.

I had earlier tried writing a simple game engine in Rust using SDL2 that was based off a C++ demo engine. I ran into various issues with the borrow checker and lifetimes, especially with the Rust SDL2 implementation, when trying to implement the C++ code in a more Rust style manner. I paused on working with Rust for a bit, but then the idea occurred to me of what would Rust be like if most of its more complex to learn features were left off the table, and instead just using the more basic foundational elements of Rust and coding it closer to a procedural style C lang app instead of an object oriented style C++ app.

This made a great difference, and then Rust became an extremely simple language to pick up and be immediately productive in. Writing Rust code in a more procedural manner, it was easy to keep track of what was going on and get the development progress moving forward quickly.

I had seen a talk the game developer Jonathan Blow gave, where he spoke about how borrow checked languages like Rust (and now Mojo lang also) could be worth the time penalty of dealing with the borrow checker if working in a systems level programming context where memory security is very important. But he expressed skepticism that Rust would be a good choice for game development where memory security isn't the primary concern, but speed of development and feature iteration (along with performance, but performance wise Rust is very good in this aspect). Working with Rust in a 'C' style procedural way did reduce the amount of times I'd have to struggle with the borrow checker, but even then Jonathan Blow's point seemed insightful that for game development the extra memory security of Rust may not be worth it if you can't iterate new game features quite as fast as in other languages like C/C++ or Jai lang (Jonathan Blow's upcoming language).

I greatly enjoyed working in Rust, but it does seem that other languages like C/C++, Jai, Zig, or Odin may be better matches for game development where they have fast performance along with no borrow checker that can occassionally slow feature development speed in exchange for memory safety. However, for systems level programming working in Rust would be very enjoyable with its more modern language design. And having the extra memory safety would be very nice to have in a systems programming context.

Having most of the functions be simple procedural style code and basic structs, the code felt nice and easy to follow. To keep the style in line with C, instead of using a method for example of subtracting two vectors from each other, I just used a function like "vec2_sub(b, a)" instead.
```rust
pub fn barycentric_weights(a: &Vec2, b: &Vec2, c: &Vec2, p: &Vec2) -> Vec3 {
    let ac = vec2_sub(c, a);
    let ab = vec2_sub(b, a);
    let pc = vec2_sub(c, p);
    let pb = vec2_sub(b, p);
    let ap = vec2_sub(p, a);

    let area_parallelogram_abc = ac.x * ab.y - ac.y * ab.x;
    let alpha = (pc.x * pb.y - pc.y * pb.x) / area_parallelogram_abc;
    let beta = (ac.x * ap.y - ac.y * ap.x) / area_parallelogram_abc;
    let gamma = 1.0 - alpha - beta;

    Vec3 {
        x: alpha,
        y: beta,
        z: gamma
    }
}
```