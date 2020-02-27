#include <iostream>
#include <cstdlib>
#include <string>
#include <sstream>

int main()
{
    using namespace std;

    //how many times we do it
    int loopNum;

    cin >> loopNum;

    
    int a = 0;
    int b = 0;
    for (int i = 0; i < loopNum; i++)
    {
        string in;
        cin >> in;
        if (in == "P=NP")
        {
            cout << "skipped" << endl;
        }else
        {
            for (int j = 0; j < in.length(); j++)
            {
                if (in[j] == '+')
                {
                    string oper1 = in.substr(0, j);
                    string oper2 = in.substr(j, in.length());

                    stringstream a1(oper1);
                    stringstream b1(oper2);

                    a1 >> a;
                    b1 >> b;
                }
               
            }
            int res = a + b;
            cout << res << endl;
            
        }
        
    }


    return EXIT_SUCCESS;
}