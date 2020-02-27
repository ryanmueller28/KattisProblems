#include <iostream>
#include <cstdlib>
#include <algorithm>
#include <string>

int main()
{
    using namespace std;

    int a, b, c;
    char cA, cB, cC;

    cin >> a >> b >> c;
    cin >> cA >> cB >> cC;

    int arr1[3] = {a, b, c};
    int n = sizeof(arr1) / sizeof(arr1[0]);

    sort(arr1, arr1 + n);

    char arr2[3] = {cA, cB, cC};
    int outArr[3];

    for (int i = 0; i < 3; i++)
    {
        if (arr2[i] == 'A')
        {
            outArr[i] = arr1[0];
        }

        if (arr2[i] == 'B')
        {
            outArr[i] = arr1[1];
        }

        if (arr2[i] == 'C')
        {
            outArr[i] = arr1[2];
        }
    }

    string outString;

    for (int i = 0; i < 3; i++)
    {
        outString += to_string(outArr[i]);
        outString += " ";
    }

    cout << outString;
    return EXIT_SUCCESS;

}