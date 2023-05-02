# Theory

Study material on computer graphics and ray tracing.

## Resources

- [Basic ray tracing](https://gabrielgambetta.com/computer-graphics-from-scratch/basic-ray-tracing.html)
- [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
- [Scratchapixel](https://scratchapixel.com)

## Summaries

Summaries of resources, etc.

### Scratchapixel 2.0 - Geometry

#### Points, Vectors, and Normals

- Conventions in linear algebra, used in Computer Graphics (CG). Understand conventions before
trying to understand techniques or code.

##### Points and Vectors

- A **point** is a position in a three-dimensional space.
- A **vector** is a direction in three-dimensional space. Think of them as arrows pointing in
various directions.
- Three-dimensional points and vectors are represented as an array of numbers: `[x,y,z]` (a
"tuple"), where `x`, `y`, and `z` are real numbers.
- Points and vectors in CG are used in a more restricted sense that in physics and mathematics.

##### Linear Transformation

- Transforming the content of a vector is called **linear transformation**.
- One of the most common operations in CG on points is to move them around in space. This type of
transformation is called a **translation**. A translation operator is a linear transformation of
the original point (the input position point). P -> Translate -> P<sub>T</sub>.
- Applying a translation to a vector has no meaning. Because where the vector begins (where it's
centered) isn't important. For vectors we used the transformation **rotation**. V -> Rotate ->
V<sub>T</sub>.
- The subscript `T` stands for transformed.
- The length of the arrow, the magnitude, of a vector is of great importance in CG. When it's `1`
it's said to be **normalized**. The act of normalizing a vector is to make it's length 1 but
without altering it's direction. Most of the time, we want them to be normalized.

> For instance, imagine that you trace a line from point A to point B. The line created is a vector
> in the sense that it indicates where point B is located relative to point A. That is, it gives
> the direction of B as if you were standing at point A. The length of the vector in this case
> indicates the distance from A to B. This distance is sometimes required in certain algorithms.
>
> Normalization of vectors is often a source of bugs in applications and every time you declare a
> vector (or even use one), we recommend that you always consciously ask yourself if this vector
> is / isn't or should / shouldn't be normalized.

##### Normals

- A **normal** describes the orientation of a surface of a geometric object at a point of that
surface. It could be a vector, that is perpendicular to a given object. Imagine a vector pointing
away from any edge/surface of a sphere; a **surface normal**.
- Normals can be thought of as vectors with the exception that they don't transform in the same
way.

##### Summary

In CG:

- a **vector** is a direction in 3D space (represented by `3` numbers).
- a **point** is a position in 3D space (represented by `3` numbers and a **homogeneous point** is
represented by `4` numbers).
- vectors and points can be transformed using **linear transformation**. Examples are
**translation** for points and **rotation** for vectors.
- a vector with the length of `1` is **normalized**. The length of a vector, before being
normalized, represents the distance between two points. This distance is sometimes needed in
certain algorithms and should be considered when normalizing.

#### Coordinate Systems

- The 3 numbers each represent a **signed distance** from the **origin** (a type of point) of a
line to the position of the point on that line.
- An **axis** is an imaginary line that passes through the origin. It's used as an ruler to measure
the **coordinate** of a point from the origin.

##### Dimensions and Cartesian Coordinate Systems

- Adding a x-axis and y-axis creates a **plane** and **Cartesian coordinates** are a pair of
numbers that specify the signed distance from the axis. These numbers are called an
**ordered pair**, where the first number is the horizontal x-coordinate and the second number is
the vertical y-coordinate.
- The coordinates of a point in a system (a **Cartesian coordinates system**) are unique. However,
a plane can have multiple coordinate systems, and a point could therefore have different
coordinates in the different systems, while still being in the same place.
- Knowing the coordinates in one system and using them to find the point in another system is an
extremely important operation in CG.
- Another linear operator that can be applied to a point is a `scale`. A scale moves a point along
the imaginary line that is going through the point and the origin.

##### The Third Dimension

- A 3D x,y,z coordinate system is known as a **Euclidean space**.
- In linear algebra, the number of axes form what is called the **base** of the coordinate system.

> A basis is a set of linearly independent vectors that, in a linear combination, can represent
> every vector (or point) in a given vector space (the coordinate system). Vectors from a set are
> said to be linearly independent if and only if none of the vectors in the set can be written as a
> linear combination of other vectors in that set.

- **Change of basis**

> [...] or change of coordinate system, is a common operation in mathematics and the graphics
> pipeline.

##### Left-Handed vs Right-Handed Coordination Systems

- **Handedness** make coordinate systems not that simple.
- **Left-hand coordinate system** and **Right-hand coordinate systems** are used to differentiate
the handedness convention.
- What defines the handedness of the coordinate system is the orientation of the left or right
vector relative to the up and forward vector.

##### The Right, Up, Forward Vectors

- Axes don't have any intrinsic meaning themselves. It's the developer who decides how they should
be treated.
- The CG industry standard tends to be the right-hand coordinate system where `x` points to the
right, `y` points up, and `z` points outward (coming out of the screen).
- The z-coordinates for a point might therefore be different in different systems and require
reversal.

##### Things We Need To Remember

- Understand this to create comfort with using the right terminology throughout CG.
- A point's coordinates are relative and unique to it's coordinate system.
- Figure out the handedness of the coordinate system you'll be using. Don't mix up handedness with
labeling of axes.

#### Math Operations on Points and Vectors

- The most common functions in 3D applications and renderer.

##### Vector Length

- A vector tells the direction of point B from A. The **vector length** tells the distance between
A and B. It's sometimes called **norm** or **magnitude**.

##### Normalizing a Vector

- A normalized vector (length 1) is also called a **unit vector** (it has a unit length). Also
called a **Euclidean norm**
- Multiplications are less costly in programs than divisions and are used for normalization
optimization. These optimizations are important because in CG you might operate on millions of
vectors if not more.

##### Dot Product

- **Dot produt** projects one vector onto another. The result is a number that relates to the
**cosine of the angle** between the two vectors. Extremely important and common operation in any 3D
application.
- It can be used to compute many things: orthogonality, the angle between two vectors, the angle
between a vector and the coordinate system.

##### Cross Product

- **Cross product** also works on two vectors (A,B) but the result is a vector (C). The result
vector is perpendicular to the other two. If all three vectors are perpendicular, they will form a
Cartesian coordinate system (if they have unit length). This is useful for creating coordinate
systems.
- The order of A and B in the cross product have an effect on the resulting C. Swapping the
positions negates the result: it is **anticommutative**.
- The result is called a **pseudo vector**

##### Vector/Point Addition and Subtraction

- Some 3D API's make distinctions between points, normals, and vectors. Other just use a single
Vec3 template for all of them and make no distinction between them from a coding point of view.
- There are some subtle differences in transformations, etc., but the complexity of the resulting
code isn't necessarily worth it.
