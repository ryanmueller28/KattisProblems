/**
 * drinkingSong.cpp
 * A solution to the Drinking Song problem on Kattis
*/
#include <iostream>
#include <cstdlib>
#include <string>

int main()
{
    using namespace std;

    //The number of lines in the song
    int lines;

    //the drink variable
    string drink;

    //get input. Duh
    cin >> lines;
    cin >> drink;


    /**
     * The core logic of the problem
     * Using a do/while loop, we iterate along
     * the song lines. 
     **/ 
    do
    {
        /** if the number of lines left in the song is 1
         * here there's a break statement. It was 
         * a solution to the loop continuing onward to the next statement
         **/

        //2? this one
        if (lines == 2)
        {
            cout << lines << " bottles of " << drink << " on the wall, ";
            cout << lines << " bottles of " << drink << "." << endl;
            cout << "Take one down, pass it around, 1 bottle of " << drink << " on the wall.\n" <<endl;
            lines --;
        }


        if (lines == 1)
        {
            cout << lines << " bottle of " << drink << " on the wall, ";
            cout << lines << " bottle of " << drink << "." << endl;
            cout << "Take it down, pass it around, no more bottles of " << drink << "." <<endl;
            lines--;
            break;
        }

        //Not 1 or 2? Do this one
        if(!(lines == 2 && lines == 1))
        {
            cout << lines << " bottles of " << drink << " on the wall, ";
            cout << lines << " bottles of " << drink << "." << endl;
            cout << "Take one down, pass it around, " << lines - 1 << " bottles of " << drink << " on the wall.\n" <<endl;
            lines --;
        }



    }while(lines > 0);

    return EXIT_SUCCESS;
}