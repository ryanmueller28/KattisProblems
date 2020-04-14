#include <iostream>
#include <cstdlib>


int main()
{
    using namespace std;

    unsigned n;

    

    cin >> n;

    unsigned* bottles = new unsigned[n];

    for (unsigned i = 0; i < n; i++)
    {
        unsigned temp;
        cin >> temp;

        bottles[i] = temp;
    }


    unsigned count = 0;
    for (unsigned i = 0; i < n - 2; i++)
    {
        for (unsigned j = i + 1; j < n - 1; j++)
        {
            if ((bottles[i] / 2) >= bottles[j]){
                for (unsigned k = j + 1; k < n; k++)
                {
                    if ((bottles[j] / 2) >= bottles[k])
                    {
                        count++;
                    }   
                }
            }
        }
    }


    cout << count << endl;

    return EXIT_SUCCESS;
}