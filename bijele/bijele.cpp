#include <iostream>
#include <cstdlib>


int main()
{
    using namespace std;

    int kingMax = 1;
    int queenMax = 1;
    int rookMax = 2;
    int bishopMax = 2;
    int knightMax = 2;
    int pawnMax = 8;

    int kingAct, queenAct, rookAct, bishopAct, knightAct, pawnAct;

    cin >> kingAct >> queenAct >> rookAct >> bishopAct >> knightAct >> pawnAct;

    cout << kingMax - kingAct << " " << queenMax - queenAct << " " << rookMax - rookAct << " " << bishopMax - bishopAct << " " << knightMax - knightAct << " " << pawnMax - pawnAct;

    return EXIT_SUCCESS;
}