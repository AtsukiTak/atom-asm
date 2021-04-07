samples/minimal.o: samples/minimal.s
	nasm -f macho64 samples/minimal.s

samples/minimal.gas.o: samples/minimal.gas.s
	as -o samples/minimal.gas.o samples/minimal.gas.s

samples/sample.o: samples/sample.c
	gcc -c -o samples/sample.o samples/sample.c
