# Things to study

- [ ] Gamma Correction, it's implementation
- [ ] Gamma Correction, it's goals
- [ ] Colors, Energy, Brightness
- [ ] Color gamut, LMS color space

## Quiz facts

- To avoid gaps between abutting triangles, WebGL2 and most
other GPU graphics APIs use **point-like pixels**

- Current computer monitors cannot represent all colors that
the eye can perceive, but we're also seeing improvements in
manufacturing resulting in more accurate primary colors and
larger gamuts of representable color. In the limit as primary
colors improve, we expect the color gamut of screens to approach:

~~all of the LMS colorspace~~

~~all colors the eye is capable of seeing~~

Gamma correction as it is commonly implemented (i.e. sRGB gamma)
**given a 50%-intensity digital color signal emits 74% of its maximum
number of photons**

- The primary goal of gamma correction is to:

~~Decrease the number of distinct byte values in images, making them
easier to compress~~

Match the spacing between binary encodings and visual perception

- Dithering will sometimes wound a color the wrong way, e.g. by
rounding 90.2 to 91 instead of 90. It does this in such a way that

~~Patterns in the boundaries between colors are noisier and harder
to see~~

The average color of a region of nearby pixels is closer to the
non-rounded average
