	jal bong
	j exit
one:	addi $sp $sp 0
	lui $t0 4096
	lui $t1 0
	ori $t1 $t1 1
	sw $t1 28($t0)
	lui $t0 4096
	lw $t1 28($t0)
	sw $t1 24($t0)
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
	lw $t1 0($t0)
	add $v0 $t1 $zero 
	addi $sp $sp 0
	jr $ra 
	addi $sp $sp 0
	jr $ra 
double:	addi $sp $sp -4
	sw $a0 0($sp)
	lui $t0 4096
	lw $t1 0($sp)
	sw $t1 32($t0)
	lui $t0 4096
	lw $t1 32($t0)
	sw $t1 28($t0)
	lui $t0 4096
	lw $t1 28($t0)
	sw $t1 24($t0)
	lui $t0 4096
	lw $t1 24($t0)
	sw $t1 20($t0)
	lui $t0 4096
	lw $t1 20($t0)
	sw $t1 16($t0)
	lui $t0 4096
	lw $t1 0($sp)
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
	lw $t1 16($t0)
	lw $t2 36($t0)
	add $t1 $t1 $t2 
	lui $t0 4096
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
	lw $t1 0($t0)
	add $v0 $t1 $zero 
	addi $sp $sp 4
	jr $ra 
	addi $sp $sp 4
	jr $ra 
bong:	addi $sp $sp -4
	lui $t0 4096
	lui $t1 0
	ori $t1 $t1 1
	sw $t1 28($t0)
	lui $t0 4096
	lw $t1 28($t0)
	sw $t1 24($t0)
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
	lw $t1 0($t0)
	sw $t1 0($sp)
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
	sw $v0 24($t0)
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
	jal double
	lw $ra 0($sp)
	addi $sp $sp 4
	lui $t0 4096
	sw $v0 24($t0)
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
	lw $t1 0($t0)
	sw $t1 0($sp)
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
	sw $v0 24($t0)
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
	jal double
	lw $ra 0($sp)
	addi $sp $sp 4
	lui $t0 4096
	sw $v0 24($t0)
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
	lw $t1 0($t0)
	sw $t1 0($sp)
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
	sw $v0 24($t0)
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
	jal double
	lw $ra 0($sp)
	addi $sp $sp 4
	lui $t0 4096
	sw $v0 24($t0)
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
	lw $t1 0($t0)
	sw $t1 0($sp)
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
	sw $v0 24($t0)
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
	jal double
	lw $ra 0($sp)
	addi $sp $sp 4
	lui $t0 4096
	sw $v0 24($t0)
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
	lw $t1 0($t0)
	sw $t1 0($sp)
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
	sw $v0 24($t0)
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
	jal double
	lw $ra 0($sp)
	addi $sp $sp 4
	lui $t0 4096
	sw $v0 24($t0)
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
	lw $t1 0($t0)
	sw $t1 0($sp)
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
	sw $v0 24($t0)
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
	addi $sp $sp 4
	jr $ra 
print_int:	addi $v0 $zero 1
	syscall 
	jr $ra 
print_char:	addi $v0 $zero 11
	syscall 
	jr $ra 
exit:	addi $v0 $zero 10
	syscall 