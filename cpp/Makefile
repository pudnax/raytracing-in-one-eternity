CXX = clang++
CFLAGS = -O3

all: src/main.cpp
	$(CXX) $(CFLAGS) src/main.cpp -o build/ppm

view: src/main.cpp
	$(CXX) $(CFLAGS) src/main.cpp -o build/ppm && ./build/ppm > image.ppm && \
		xdg-open image.ppm 2>/dev/null

clean: build/ppm
	rm build/ppm
