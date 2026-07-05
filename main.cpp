#include <iostream>
using namespace std;

bool match(string pattern, string text) {
    if (pattern.length() != text.length())
        return false;

    for (int i = 0 ; i < pattern.length() ; ++i) {
        if (pattern[i] != text[i])
            return false;
    }
    return true;
}

int main() {
    cout << match("cat" , "cat") << endl;
    cout << match("sarthak" , "sarthak") << endl;
}