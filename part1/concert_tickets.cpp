#include <iostream>
#include <vector>
#include <map>
#include <ext/pb_ds/assoc_container.hpp>

using namespace std;
using namespace __gnu_pbds;

typedef tree<int,null_type,less<int>,rb_tree_tag,tree_order_statistics_node_update> indexed_set;

int main() {
    int t_n, c_n;
    cin >> t_n >> c_n;

    indexed_set tickets;
    map<int,int> ticket_amounts;
    for (int i = 1; i <= t_n; i++) {
	int t;
	cin >> t;

	auto ord = tickets.order_of_key(t);
	int key =  *tickets.find_by_order(ord);
	if (t == key) {
	    ticket_amounts[t]++;
	} else {
	    ticket_amounts[t] = 1;
	}
	tickets.insert(t);
    }

    vector<int> customers;
    for (int i = 1; i <= c_n; i++) {
	int c;
	cin >> c;
	customers.push_back(c);
    }

    for (auto customer : customers) {
	int ord = tickets.order_of_key(customer);
	int t;
	t = *tickets.find_by_order(ord);
	if (customer != t) {
	    ord--;
	    t = *tickets.find_by_order(ord);
	    if (t == 0) {
		t = -1;
	    }

	}
	cout << t << "\n";
	ticket_amounts[t]--;
	if (!ticket_amounts[t]) {
	    tickets.erase(t);
	}
	
    }
}
