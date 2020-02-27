#include <iostream>
#include <cstdlib>

int main()
{
    using namespace std;

    string in;
    string out;

    cin >> in;

    for (int i = 0; i < in.length(); i++)
    {
        if(in[i] != in[i - 1])
        {
            out += in[i];
        }
    }

    cout << out;
    return EXIT_SUCCESS;
}