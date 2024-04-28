default:
	just --list
dev:
	cargo run --bin 3d_sim
run:
	cargo run --bin 3d_sim --release
test:
	cargo test --bin 3d_sim
perf:
	perf record -g cargo run --bin benchmark --profile analysis
report:
	perf report -g
perf-stat:
	perf stat -d cargo run --bin benchmark --profile analysis
flamegraph:
	mkdir -p flamegraph
	cargo flamegraph --bin benchmark --profile analysis
	mv ./flamegraph.svg ./flamegraph/$(date +"%Y-%m-%d_%H-%M-%S").svg
clean:
	cargo clean
