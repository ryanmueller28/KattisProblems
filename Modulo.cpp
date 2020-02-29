#include <iostream>
#include <algorithm>
#include <cstdlib>
#include <set>
#include <string>

int main()
{
	using namespace std;

	int a;
	const int b = 42;
	int c;

	set<int> value;
	

	for (int i = 0; i < 10; i++){
		cin >> a;
		c = a % b;
		value.insert(c);
	}
	cout << value.size();

	return EXIT_SUCCESS;
}

