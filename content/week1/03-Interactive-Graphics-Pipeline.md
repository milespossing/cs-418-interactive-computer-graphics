# Interactive Graphics

[Reading](https://www.coursera.org/learn/cs418-interactive-computer-graphics/supplement/12VuH/interactive-graphics)
## Rasterizing vs Raytracing

Rasterizing is drawing one object at a time.

- It is extremely fast
- It does not allow for light interactions between objects

Raytracing is drawing one pixel at a time. These algorithms
attempt to find all the light paths which contribute to a
given pixel.

- Much nicer pictures
- Much slower than rasterizing (orders of magnitude)

So we have slow vs fast algorithms for rendering. We also
describe **rasterizing** as **interactive** and **raytracing**
as **production**

This course focuses on the rasterization technique and will
do so on GPUs.

## Pipeline Outline

[Reading](https://cs418.cs.illinois.edu/website/text/pipeline.html)

### High-level overview

1. Specify 3D Geometry
    * geometry refers to the stuff we want to draw
      * vertex--A point in space; also includes more information
      on color (and many other data points)
      * Primitive--How we connect vertices
        * Points--Draw a square on the screen at the point
        * lines--Fill in all pixels between 2 points (rarely used in 3D)
        * triangles (by far the dominant choice)
          * Given 3 vertices, fill in all points between the vertices
          * We also define **quads** which are two triangles which together
          define a rectangular area
2. Move vertices to view scene
    * We move the vertices for objects from the object space (some enumeration
    of objects) to the world space
    * We don't stop there, though, we transform again from those world
    coordinates to place the eye at the origin and then transform everything
    with that same transformation. This way we render the scene relative to
    where the "camera" or viewer is.
    * Because we do this on the GPU, the code to perform this is the "shader"
      * This "shader" is what performs the transformation from $A->A'$
3. Convert shapes to the pixels they cover
    * This is also called "Rasterization"
    * A pixel covered by a **single object** is called a **fragment**.
    * We find all the **fragments** within a primitive, and we interpolate the
    data in the vertices onto each of those fragments.
    * We now know all the rendering information required for each fragment
4. Color each pixel
    * Having looped over (in parallel) each object in the scene, we have a lot
    of fragments for each pixel.
    * We need to color each pixel, using the lighting, and the Z axis
    * We call the lighting a **fragment shader**. This object decides how we
    light the individual fragments.
5. Write into the frame buffer
    * Buffer generally refers to any kind of **array of information**

## Appearance: Mirages and Total Internal Reflection

[Appearance: The Student, the Fish, and Agassiz](https://cs418.cs.illinois.edu/website/text/fish.html)

