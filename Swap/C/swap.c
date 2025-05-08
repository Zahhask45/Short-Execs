#include <stdio.h>

int main(){
	
	// Using XOR (bitwise)
	{
		int a = 10; // first number -> 1010
		int b = 45; // second number -> 0010 1101
		
		printf("Before the Swapping (XOR/BITWISE)\n");
		printf("a: %i\n", a);
		printf("b: %i\n", b);

		a = a ^ b; // new value for a is -> 0010 0111
		b = a ^ b; // new value for b is -> 0000 1010
		a = a ^ b; // new value for a is -> 0010 1101

		printf("After the Swapping (XOR/BITWISE)\n");
		printf("a: %i\n", a);
		printf("b: %i\n", b);
	}
	// Using Arithmetics (bitwise)
	printf("\n==================\n");
	{
		int a = 10; // first number
		int b = 45; // second number
		printf("Before the Swapping (Arithmetic)\n");
		printf("a: %i\n", a);
		printf("b: %i\n", b);

		a = a + b; // new value for a is -> 55
		// a needs to be first, because is the higher number
		b = a - b; // new value for b is -> 10
		a = a - b; // new value for a is -> 45

		printf("After the Swapping (Arithmetic)\n");
		printf("a: %i\n", a);
		printf("b: %i\n", b);
	}
}
