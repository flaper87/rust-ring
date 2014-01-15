build:
	rustc -L./lib src/ring/lib.rs
	#rustpkg build -O ring

test: build
	rustc --test -L./lib src/ring/lib.rs -o ring-tests
	./ring-tests

install: build
	rustpkg install ring

clean:
	rustpkg clean
