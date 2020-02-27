#include <iostream>
#include <cstdlib>


int main()
{
    using namespace std;

    int loopNum;

    int count = 0;

    cin >> loopNum;
    int temp;

    for (int i = 0; i < loopNum; i++)
    {
        cin >> temp;
    if (temp<0){
            count++;
        }
    }
    cout << count;

    return EXIT_SUCCESS;
}