build:
	rustc foo.rs --crate-type="staticlib" -o ./foo.a &&\
	clang -c use_foo.c -o use_foo.o &&\
	clang -lc -lm -lpthread -lSystem use_foo.o foo.a -o use_foo
clean:
	@rm -f foo.a use_foo.o use_foo