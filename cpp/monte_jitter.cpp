#include <iomanip>
#include <iostream>
#include <random>

inline double random_double() {
  // Returns a random real in [0,1).
  return rand() / (RAND_MAX + 1.0);
}

inline double random_double(double min, double max) {
  // Returns a random real in [min,max).
  return min + (max - min) * random_double();
}

int main() {
  int inside_circle = 0;
  int inside_circle_stratified = 0;
  int sqrt_N = 10000;
  for (int i = 0; i < sqrt_N; i++) {
    for (int j = 0; j < sqrt_N; j++) {
      auto x = random_double(-1, 1);
      auto y = random_double(-1, 1);
      if (x * x + y * y < 1)
        inside_circle++;
      x = 2 * ((i + random_double()) / sqrt_N) - 1;
      y = 2 * ((j + random_double()) / sqrt_N) - 1;
      if (x * x + y * y < 1)
        inside_circle_stratified++;
    }
  }

  auto N = static_cast<double>(sqrt_N) * sqrt_N;
  std::cout << std::fixed << std::setprecision(12);
  std::cout << "Regular    Estimate of Pi = "
            << 4 * double(inside_circle) / (sqrt_N * sqrt_N) << '\n'
            << "Stratified Estimate of Pi = "
            << 4 * double(inside_circle_stratified) / (sqrt_N * sqrt_N) << '\n';
}
