#include <stdio.h>

int main(){
	int size = 10; // Amount of rows you want
	char str[size]; // str containing the '*'

	for (int i = 1; i <= size; i++){
		str[i - 1] = '*'; // array start at zero
		printf("%.*s\n", i, str);
	}
	
}
