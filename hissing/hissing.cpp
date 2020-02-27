#include <iostream>
#include <cstdlib>
#include <string>

int main()
{
    using namespace std;

    string in;

    cin >> in;

    int tempCount = 0;

    for (int i = 0; i < in.length(); i++)
    {
        if (in[i] == 's' && in[i - 1] == 's'){
            tempCount++;
        }
    }

    if(tempCount > 0)
    {
        cout << "hiss";
    }else
    {
        cout << "no hiss";
    }
    return EXIT_SUCCESS;
}