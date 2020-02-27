#include <iostream>
#include <cstdlib>
#include <string>

int main()
{
    using namespace std;
    string jM;
    string dr;

    cin >> jM;
    cin >> dr;

    if (jM.length() >= dr.length())
    {
        cout << "go";
    }else
    {
        cout << "no";
    }
    

    return EXIT_SUCCESS;
}