#include <iostream>
#include <cstdlib>

using namespace std;

int reverseDigits(int num);


int main()
{
    int fInt, tInt, rOne, rTwo;

    cin >> fInt >> tInt;

if(reverseDigits(fInt) > reverseDigits(tInt)){
    cout << reverseDigits(fInt);
}else{
    cout << reverseDigits(tInt);
}


    return EXIT_SUCCESS;
}


int reverseDigits(int num){
    int rev_num = 0;
    
    while(num > 0)
    {
        rev_num = rev_num * 10 + num % 10;
        num = num / 10;
    }
    return rev_num;
}
