#include <iostream>
#include <cstdlib>
#include <vector> // for a data structure
#include <algorithm> // for sort

int main()
{
    using namespace std;

    //number of tests
    int testCases;


    cin >> testCases;

    for (int i = 0; i < testCases; i++)
    {
        // size variables
        int gSize;
        int mSize;

        //data structures
        vector<int> g; 
        vector<int> m;

        //get the relative army size
        cin >> gSize;
        cin >> mSize;


        //lines 32 through 48:
        //get individual army strength
        // and sort vectors
        for (int i = 0; i < gSize; i++)
        {
            int n;
            cin >> n;
            g.push_back(n);
        }

        sort(g.begin(), g.end());

        for (int i = 0; i < mSize; i++)
        {
            int n;
            cin >> n;
            m.push_back(n);
        }

        sort(m.begin(), m.end());

        int g1 = 0;
        int m1 = 0;


        //Algorithm time
        while (g1 < gSize && m1 < mSize)
        {
            // is army g[g1] < army m[m1]
            if (g[g1] < m[m1])
            {
                g1++; //ded
            }else{
                m1++; //ded
            }
        }

        if(g1 == gSize) //Mecha won, killed all
        {
            cout << "MechaGodzilla" << endl;
        }else { // Godzilla won, earth is saved
            cout << "Godzilla" << endl;
        }
    }

    return EXIT_SUCCESS;
}
