run:
	systemfd --no-pid -s http::8081 -- watchexec -r -- cargo run

backtrace:
	RUST_BACKTRACE=1 systemfd --no-pid -s http::8081 -- watchexec -r -- cargo run
