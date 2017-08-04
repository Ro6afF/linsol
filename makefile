test: build_tests run_tests

build_tests:
	rustc --test --deny warnings tests.rs

run_tests:
	./tests

update_repo: test
	git add -A
	git commit
	git push
