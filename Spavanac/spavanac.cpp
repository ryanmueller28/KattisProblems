#include <iostream>
#include <cstdlib>
#include <iomanip>
#include <string>

int main()
{
    using namespace std;

    int h, m;

    cin >> h >> m;

    //when m < 45
    
    m -= 45;

    if (m < 0)
    {
        m = 60 + m;
        h -= 1;
    }
    if (h < 0)
    {
        h = 24 + h ;
    }

    //print

    cout << h << " " << m << endl;
    return EXIT_SUCCESS;
}