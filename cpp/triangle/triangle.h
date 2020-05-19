#if !defined(TRIANGLE_H)
#define TRIANGLE_H

namespace triangle {
    enum flavor {
        equilateral,
        isosceles,
        scalene
    };

    flavor kind(
        double side_a,
        double side_b,
        double side_c);
}  // namespace triangle

#endif // TRIANGLE_H
