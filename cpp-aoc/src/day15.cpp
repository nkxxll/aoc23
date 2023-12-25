#include <bits/stdc++.h>

#define ll long long
#define ull unsigned long long
#define vi vector<int>
#define pi pair<int, int>
#define MP make_pair

using namespace std;

int my_hash(string s);
void sol1();

int main(int argc, char *argv[])
{
    string line;
    array<vector<pair<string, int> >, 256> arr;
    cin >> line;
    size_t pos = 0;
    ll res = 0;
    while ((pos = line.find(',')) != string::npos) {
        string s = line.substr(0, pos);
        cout << s.size() << "\n";

        // parse the label
        char c[] = { s[0], s[1] };
        char op = s[2];
        int hash_val = my_hash(s.substr(2));

        // add label to array
        if (op == '=') {
            char val = s[3] - '0';
            // add to arr if the label is not already in the arr
            if (arr[hash_val].size() == 0) {
                arr[hash_val].push_back(MP(c, val));
            } else {
                // replace the value at the label
                bool replaced = false;
                for (auto vec : arr[hash_val]) {
                    if (vec.first == c) {
                        vec.second = val;
                        replaced = true;
                        break;
                    }
                }
                if (!replaced) {
                    arr[hash_val].push_back(MP(c, val));
                }
            }
        } else {
            // remove from arr at hash
            for (auto pair : arr[hash_val]) {
                if (pair.first == c) {
                    // remove the pair from the vector
                    arr[hash_val].erase(
                        remove(arr[hash_val].begin(), arr[hash_val].end(), pair),
                        arr[hash_val].end());
                    break;
                }
            }
        }
        line.erase(0, pos + 1);
    }

    // print arr
    for (auto item : arr) {
        for (auto vec : item) {
            cout << vec.first << " " << vec.second << "\n";
        }
    }

    // sum together the values in array with the right formula
    // for item in arr: if item != []: for vec in item:
    // One plus the box number of the lens in question.
    // The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
    // The focal length of the lens.
    for (size_t i = 0; i < 256; i++) {
        auto item = arr[i];
        size_t size = item.size();
        if (size != 0) {
            for (size_t j = 0; j < size; j++) {
                res += (1 + i) * (1 + j) * item[j].second;
            }
        }
    }
    cout << "the res is : " << res << "\nfinished\n";
}

int my_hash(string s)
{
    int res = 0;
    for (auto c : s) {
        res += c;
        res *= 17;
        res %= 256;
    }
    return res;
}

void sol1()
{
    string line;
    cin >> line;
    size_t pos = 0;
    ll res = 0;
    while ((pos = line.find(',')) != string::npos) {
        int sub = 0;
        string s = line.substr(0, pos);
        for (auto c : s) {
            // what we want to do with the things
            sub += c;
            sub *= 17;
            sub %= 256;
        }
        res += sub;
        line.erase(0, pos + 1);
    }
    int sub = 0;
    for (auto c : line) {
        // what we want to do with the things
        sub += c;
        sub *= 17;
        sub %= 256;
    }
    res += sub;
    cout << "the res is : " << res << "\nfinished\n";
}
