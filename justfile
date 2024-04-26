default:
	just --list
dev:
	cargo run --bin 3d_sim
run:
	cargo run --bin 3d_sim --release
test:
	cargo test
perf:
	perf record -g cargo run --bin benchmark --profile analysis
report:
	perf report -g
perf-stat:
	perf stat -d cargo run --bin benchmark --profile analysis
clean:
	cargo clean
