	jal bong
	j exit
foo:	addi $sp $sp -16
	sw $a0 0($sp)
	sw $a1 4($sp)
	sw $a2 8($sp)
	sw $a3 12($sp)
	lui $t0 4096
	lw $t1 0($sp)
	sw $t1 60($t0)
	lui $t0 4096
	lw $t1 60($t0)
	sw $t1 56($t0)
	lui $t0 4096
	lw $t1 56($t0)
	sw $t1 52($t0)
	lui $t0 4096
	lw $t1 52($t0)
	sw $t1 48($t0)
	lui $t0 4096
	lw $t1 48($t0)
	sw $t1 44($t0)
	lui $t0 4096
	lw $t1 44($t0)
	sw $t1 40($t0)
	lui $t0 4096
	lw $t1 40($t0)
	sw $t1 36($t0)
	lui $t0 4096
	lw $t1 36($t0)
	sw $t1 32($t0)
	lui $t0 4096
	lw $t1 32($t0)
	add $a0 $t1 $zero 
	addi $sp $sp -4
	sw $ra 0($sp)
	jal print_int
	lw $ra 0($sp)
	addi $sp $sp 4
	lui $t0 4096
	lw $t1 24($t0)
	sw $t1 20($t0)
	lui $t0 4096
	lw $t1 20($t0)
	sw $t1 16($t0)
	lui $t0 4096
	lw $t1 16($t0)
	sw $t1 12($t0)
	lui $t0 4096
	lw $t1 12($t0)
	sw $t1 8($t0)
	lui $t0 4096
	lw $t1 8($t0)
	sw $t1 4($t0)
	lui $t0 4096
	lw $t1 4($t0)
	sw $t1 0($t0)
	lui $t0 4096
	lw $t1 4($sp)
	sw $t1 60($t0)
	lui $t0 4096
	lw $t1 60($t0)
	sw $t1 56($t0)
	lui $t0 4096
	lw $t1 56($t0)
	sw $t1 52($t0)
	lui $t0 4096
	lw $t1 52($t0)
	sw $t1 48($t0)
	lui $t0 4096
	lw $t1 48($t0)
	sw $t1 44($t0)
	lui $t0 4096
	lw $t1 44($t0)
	sw $t1 40($t0)
	lui $t0 4096
	lw $t1 40($t0)
	sw $t1 36($t0)
	lui $t0 4096
	lw $t1 36($t0)
	sw $t1 32($t0)
	lui $t0 4096
	lw $t1 32($t0)
	add $a0 $t1 $zero 
	addi $sp $sp -4
	sw $ra 0($sp)
	jal print_int
	lw $ra 0($sp)
	addi $sp $sp 4
	lui $t0 4096
	lw $t1 24($t0)
	sw $t1 20($t0)
	lui $t0 4096
	lw $t1 20($t0)
	sw $t1 16($t0)
	lui $t0 4096
	lw $t1 16($t0)
	sw $t1 12($t0)
	lui $t0 4096
	lw $t1 12($t0)
	sw $t1 8($t0)
	lui $t0 4096
	lw $t1 8($t0)
	sw $t1 4($t0)
	lui $t0 4096
	lw $t1 4($t0)
	sw $t1 0($t0)
	lui $t0 4096
	lw $t1 8($sp)
	sw $t1 60($t0)
	lui $t0 4096
	lw $t1 60($t0)
	sw $t1 56($t0)
	lui $t0 4096
	lw $t1 56($t0)
	sw $t1 52($t0)
	lui $t0 4096
	lw $t1 52($t0)
	sw $t1 48($t0)
	lui $t0 4096
	lw $t1 48($t0)
	sw $t1 44($t0)
	lui $t0 4096
	lw $t1 44($t0)
	sw $t1 40($t0)
	lui $t0 4096
	lw $t1 40($t0)
	sw $t1 36($t0)
	lui $t0 4096
	lw $t1 36($t0)
	sw $t1 32($t0)
	lui $t0 4096
	lw $t1 32($t0)
	add $a0 $t1 $zero 
	addi $sp $sp -4
	sw $ra 0($sp)
	jal print_int
	lw $ra 0($sp)
	addi $sp $sp 4
	lui $t0 4096
	lw $t1 24($t0)
	sw $t1 20($t0)
	lui $t0 4096
	lw $t1 20($t0)
	sw $t1 16($t0)
	lui $t0 4096
	lw $t1 16($t0)
	sw $t1 12($t0)
	lui $t0 4096
	lw $t1 12($t0)
	sw $t1 8($t0)
	lui $t0 4096
	lw $t1 8($t0)
	sw $t1 4($t0)
	lui $t0 4096
	lw $t1 4($t0)
	sw $t1 0($t0)
	lui $t0 4096
	lw $t1 12($sp)
	sw $t1 60($t0)
	lui $t0 4096
	lw $t1 60($t0)
	sw $t1 56($t0)
	lui $t0 4096
	lw $t1 56($t0)
	sw $t1 52($t0)
	lui $t0 4096
	lw $t1 52($t0)
	sw $t1 48($t0)
	lui $t0 4096
	lw $t1 48($t0)
	sw $t1 44($t0)
	lui $t0 4096
	lw $t1 44($t0)
	sw $t1 40($t0)
	lui $t0 4096
	lw $t1 40($t0)
	sw $t1 36($t0)
	lui $t0 4096
	lw $t1 36($t0)
	sw $t1 32($t0)
	lui $t0 4096
	lw $t1 32($t0)
	add $a0 $t1 $zero 
	addi $sp $sp -4
	sw $ra 0($sp)
	jal print_int
	lw $ra 0($sp)
	addi $sp $sp 4
	lui $t0 4096
	lw $t1 24($t0)
	sw $t1 20($t0)
	lui $t0 4096
	lw $t1 20($t0)
	sw $t1 16($t0)
	lui $t0 4096
	lw $t1 16($t0)
	sw $t1 12($t0)
	lui $t0 4096
	lw $t1 12($t0)
	sw $t1 8($t0)
	lui $t0 4096
	lw $t1 8($t0)
	sw $t1 4($t0)
	lui $t0 4096
	lw $t1 4($t0)
	sw $t1 0($t0)
	addi $sp $sp 16
	jr $ra 
bong:	addi $sp $sp 0
	lui $t0 4096
	lui $t1 0
	ori $t1 $t1 3
	sw $t1 60($t0)
	lui $t0 4096
	lw $t1 60($t0)
	sw $t1 56($t0)
	lui $t0 4096
	lw $t1 56($t0)
	sw $t1 52($t0)
	lui $t0 4096
	lw $t1 52($t0)
	sw $t1 48($t0)
	lui $t0 4096
	lw $t1 48($t0)
	sw $t1 44($t0)
	lui $t0 4096
	lw $t1 44($t0)
	sw $t1 40($t0)
	lui $t0 4096
	lw $t1 40($t0)
	sw $t1 36($t0)
	lui $t0 4096
	lw $t1 36($t0)
	sw $t1 32($t0)
	lui $t0 4096
	lui $t1 0
	ori $t1 $t1 4
	sw $t1 96($t0)
	lui $t0 4096
	lw $t1 96($t0)
	sw $t1 92($t0)
	lui $t0 4096
	lw $t1 92($t0)
	sw $t1 88($t0)
	lui $t0 4096
	lw $t1 88($t0)
	sw $t1 84($t0)
	lui $t0 4096
	lw $t1 84($t0)
	sw $t1 80($t0)
	lui $t0 4096
	lw $t1 80($t0)
	sw $t1 76($t0)
	lui $t0 4096
	lw $t1 76($t0)
	sw $t1 72($t0)
	lui $t0 4096
	lw $t1 72($t0)
	sw $t1 68($t0)
	lui $t0 4096
	lui $t1 0
	ori $t1 $t1 4
	sw $t1 136($t0)
	lui $t0 4096
	lw $t1 136($t0)
	sw $t1 132($t0)
	lui $t0 4096
	lw $t1 132($t0)
	sw $t1 128($t0)
	lui $t0 4096
	lw $t1 128($t0)
	sw $t1 124($t0)
	lui $t0 4096
	lw $t1 124($t0)
	sw $t1 120($t0)
	lui $t0 4096
	lui $t1 0
	ori $t1 $t1 2
	sw $t1 152($t0)
	lui $t0 4096
	lw $t1 152($t0)
	sw $t1 148($t0)
	lui $t0 4096
	lw $t1 148($t0)
	sw $t1 144($t0)
	lui $t0 4096
	lw $t1 144($t0)
	sw $t1 140($t0)
	lui $t0 4096
	lw $t1 120($t0)
	lw $t2 140($t0)
	add $t1 $t1 $t2 
	lui $t0 4096
	sw $t1 116($t0)
	lui $t0 4096
	lw $t1 116($t0)
	sw $t1 112($t0)
	lui $t0 4096
	lw $t1 112($t0)
	sw $t1 108($t0)
	lui $t0 4096
	lw $t1 108($t0)
	sw $t1 104($t0)
	lui $t0 4096
	lui $t1 0
	ori $t1 $t1 1
	sw $t1 188($t0)
	lui $t0 4096
	lw $t1 188($t0)
	sw $t1 184($t0)
	lui $t0 4096
	lw $t1 184($t0)
	sw $t1 180($t0)
	lui $t0 4096
	lw $t1 180($t0)
	sw $t1 176($t0)
	lui $t0 4096
	lw $t1 176($t0)
	sw $t1 172($t0)
	lui $t0 4096
	lw $t1 172($t0)
	sw $t1 168($t0)
	lui $t0 4096
	lw $t1 168($t0)
	sw $t1 164($t0)
	lui $t0 4096
	lw $t1 164($t0)
	sw $t1 160($t0)
	lui $t0 4096
	lw $t1 32($t0)
	add $a0 $t1 $zero 
	lui $t0 4096
	lw $t1 68($t0)
	add $a1 $t1 $zero 
	lui $t0 4096
	lw $t1 104($t0)
	add $a2 $t1 $zero 
	lui $t0 4096
	lw $t1 160($t0)
	add $a3 $t1 $zero 
	addi $sp $sp -4
	sw $ra 0($sp)
	jal foo
	lw $ra 0($sp)
	addi $sp $sp 4
	lui $t0 4096
	lw $t1 24($t0)
	sw $t1 20($t0)
	lui $t0 4096
	lw $t1 20($t0)
	sw $t1 16($t0)
	lui $t0 4096
	lw $t1 16($t0)
	sw $t1 12($t0)
	lui $t0 4096
	lw $t1 12($t0)
	sw $t1 8($t0)
	lui $t0 4096
	lw $t1 8($t0)
	sw $t1 4($t0)
	lui $t0 4096
	lw $t1 4($t0)
	sw $t1 0($t0)
	addi $sp $sp 0
	jr $ra 
print_int:	addi $v0 $zero 1
	syscall 
	jr $ra 
print_char:	addi $v0 $zero 11
	syscall 
	jr $ra 
exit:	addi $v0 $zero 10
	syscall 
