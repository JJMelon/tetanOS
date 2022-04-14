all: boot.asm kern.asm
	nasm boot.asm -f bin -o boot.bin
	nasm kern.asm -f bin -o kern.bin
	dd if=/dev/zero of=disk.img bs=512 count=2880
	dd if=boot.bin of=disk.img bs=512 conv=notrunc
	dd if=kern.bin of=disk.img bs=512 seek=1 conv=notrunc
	
qemu:
	qemu-system-i386 -fda disk.img