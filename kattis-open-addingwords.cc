// https://open.kattis.com/problems/addingwords

#include <iostream>
#include <unordered_map>
#include <optional>

using namespace std;

int main() {
    cin.sync_with_stdio(false);
    cout.sync_with_stdio(false);

    string command;
    unordered_map<string, int> name2val;
    unordered_map<int, string> val2name;

    while (cin >> command) {
        switch (command[1]) {
            case 'e': {
                string name; int val; cin >> name >> val;
                auto it = name2val.find(name);
                if (it != name2val.end()) {
                    val2name.erase(it->second);
                }
                name2val[name] = val;
                val2name[val] = name;
            } break;
            case 'a': {
                optional<int> val = 0;

                string name, op = "+"; cin >> name; cout << name << ' ';
                while (true) {
                    auto it = name2val.find(name);
                    if (it == name2val.end()) {
                        val = nullopt;
                    } else if (val) {
                        *val += (op[0] == '+' ? it->second : -it->second);
                    }

                    cin >> op; cout << op << ' ';
                    if (op[0] == '=') break;
                    cin >> name; cout << name << ' ';
                }

                if (val) {
                    auto it = val2name.find(*val);
                    if (it == val2name.end()) {
                        cout << "unknown" << endl;
                    } else {
                        cout << it->second << endl;
                    }
                } else {
                    cout << "unknown" << endl;
                }
            } break;
            case 'l': {
                name2val = unordered_map<string, int>();
                val2name = unordered_map<int, string>();
            } break;
        }
    }
}
