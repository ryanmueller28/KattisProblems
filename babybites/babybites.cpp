/***
 * babybites.cpp
 * a solution to Baby Bites on Kattis
 * Ryan Mueller
*/
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>

int main()
{
    using namespace std;

    vector<string> bite;


    //how many times Arild speaks
    int loopNum;

    cin >> loopNum;

    bite.push_back("zeroth place");
    //a container for our numbers
    for (int i = 0; i < loopNum; i++)
    {
        string temp;
        cin >> temp;

        bite.push_back(temp);
    }

    while (loopNum != 0)
    {
        if (bite[loopNum] == "mumble")
        {
            loopNum--;
            if (loopNum == 0)
            {
                cout << "makes sense" << endl;
                break;
            }
        }else if (loopNum == stoi(bite[loopNum]))
        {
            loopNum--;
            if (loopNum == 0)
            {
                cout << "makes sense" << endl;
                break;
            }
        }else
        {
            cout << "something is fishy" << endl;
            break;
        }
    }

    return EXIT_SUCCESS;
}