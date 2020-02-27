#include <iostream>
#include <cstdlib>
#include <string>
    
using namespace std;

bool isHalloween(string month, int day);

int main()
{


    string month;
    int day;

    cin >> month >> day;

    if (isHalloween(month, day)){
        cout << "yup";
    }else 
    {
        cout << "nope";
    }


    return EXIT_SUCCESS;
}

bool isHalloween(string month, int day)
{
    if (month == "OCT" && day == 31)
    {
        return true;
    }else if (month == "DEC" && day == 25){
        return true;
    }else{
        return false;
    }
}