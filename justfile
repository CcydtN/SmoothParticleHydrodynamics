default:
	just --list
dev:
	cargo run --bin 3d_sim --features "rendering"
run:
	cargo run --bin 3d_sim --features "rendering" --release
test:
	cargo test
perf:
	perf record -g cargo run --bin 3d_sim --profile analysis
report:
	perf report -g
perf-stat:
	perf stat -d cargo run --bin 3d_sim --profile analysis
clean:
	cargo clean
