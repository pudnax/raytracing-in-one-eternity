#ifndef COLOR_H
#define COLOR_H

#include "rtweekend.h"

#include <iostream>

void write_color(std::ostream &out, color pixel_color, int samples_per_pixels) {
  auto r = pixel_color.x();
  auto g = pixel_color.y();
  auto b = pixel_color.z();

  auto scale = 1.0 / samples_per_pixels;
  r *= scale;
  g *= scale;
  b *= scale;

  out << static_cast<int>(255.999 * clamp(r, 0.0, 0.999)) << ' '
      << static_cast<int>(255.999 * clamp(g, 0.0, 0.999)) << ' '
      << static_cast<int>(255.999 * clamp(b, 0.0, 0.999)) << '\n';
}

#endif
