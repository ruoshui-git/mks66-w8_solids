all:
	cargo test --release --package w8_solids --bin w8_solids -- graphics::parser::tests::script --exact --nocapture