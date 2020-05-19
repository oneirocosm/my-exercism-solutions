#include <algorithm>
#include <stdexcept>
#include <unordered_set>
#include "triangle.h"

// function prototypes -------------------------------------------------------

namespace triangle {
    class Triangle {
        private:
            std::unordered_set<double> sides;
            double perimeter;
            bool triangle_sides_valid(void);
            bool triangle_inequality_valid(void);
            double largest_side(void);
        public:
            Triangle(double side_a, double side_b, double side_c);
            flavor get_flavor(void);
    };

} // namespace triangle


// function definitions ------------------------------------------------------

namespace triangle {
    flavor kind(
        double side_a,
        double side_b,
        double side_c)
    {
        Triangle triangle(side_a, side_b, side_c);
        return triangle.get_flavor();
    }

    Triangle::Triangle(double side_a, double side_b, double side_c) :
        sides(std::unordered_set<double>{side_a, side_b, side_c}),
        perimeter(side_a + side_b + side_c)
    {
        if (!triangle_sides_valid()) {
            throw std::domain_error("Side lengths must be greater than 0");
        }

        if (!triangle_inequality_valid()) {
            throw std::domain_error("Triangle inequality not satisfied");
        }
    }

    flavor Triangle::get_flavor(void)
    {
        flavor triangle_flavor;
        switch (sides.size()) {
            case 1:
                triangle_flavor = flavor::equilateral;
                break;
            case 2:
                triangle_flavor = flavor::isosceles;
                break;
            default:
                triangle_flavor = flavor::scalene;
                break;
        }
        return triangle_flavor;
    }

    bool Triangle::triangle_sides_valid(void)
    {
        bool sides_valid_so_far = true;
        for (const auto& side: sides) {
            sides_valid_so_far = sides_valid_so_far && (side > 0);
        }
        return sides_valid_so_far;
    }

    bool Triangle::triangle_inequality_valid(void)
    {
        return 2 * largest_side() <= perimeter;
    }

    double Triangle::largest_side(void)
    {
        double largest = 0;
        for (const auto& side: sides) {
            largest = std::max(largest, side);
        }
        return largest;
    }

}  // namespace triangle
