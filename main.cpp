#include <iostream>
#include <vector>

using namespace std;

int get_reverse(int x) {
  int reverse = 0;
  while (x) {
    int digit = x % 10;
    reverse = reverse * 10 + digit;
    x /= 10;
  }
  // return a reversed number
  return reverse;
}

bool is_prime(int x) {
  for (int i = 2; i * i < x; ++i) {
    if (x % i == 0)
      return false;
  }

  // return whether a number is prime
  return true;
}

int main() {
  int m;
  cin >> m;
  for (int i = m; i >= 2; --i) {
    if (is_prime(i) && is_prime(get_reverse(i))) {
      cout << i << endl;
      return 0;
    }
  }

  cout << "no\n";
  return 0;
}
