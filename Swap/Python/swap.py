if __name__ == "__main__":
	a = 10
	b = 45
	print("Before the Swapping (XOR/BITWISE)")
	print("a: ", a)
	print("b: ", b)
    
	a = a ^ b
	b = a ^ b
	a = a ^ b

	print("After the Swapping (XOR/BITWISE)")
	print("a: ", a)
	print("b: ", b)

	print("=========================")

	a = 10
	b = 45
	print("Before the Swapping (Arithmetic)")
	print("a: ", a)
	print("b: ", b)

	a = a ^ b
	b = a ^ b
	a = a ^ b

	print("After the Swapping (Arithmetic)")
	print("a: ", a)
	print("b: ", b)
