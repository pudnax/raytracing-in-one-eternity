all:
	cargo build --release

view:
	cargo build --release && time ./target/release/raytrace > final.ppm && \
		feh --auto-zoom -z --scale-down -g 1024x576 final.ppm 2>/dev/null

clean:
	cargo clean
