all:
	cargo build --release --bin raytrace

view:
	cargo build --release --bin raytrace && time ./target/release/raytrace > final.ppm && \
		feh --auto-zoom -z --scale-down -g 1024x576 final.ppm 2>/dev/null

clean:
	cargo clean
