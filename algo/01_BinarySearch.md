#### Run webpack-dev-server
If testing of legacy browsers is required, the development server can still be run using a [webpack](https://webpack.js.org/).

``` c++
int binarySearch(int el, const std::vector<int> &arr) {
    int left = 0, right = arr.size() - 1;
    while (left <= right) {
        int idx = (left + right) / 2;
        int middle = arr[idx];
        if (el < middle) {
            right = idx;
        } else if (el > middle) {
            left = idx;
        } else {
            return idx;
        }
    }
    return -1;
}
```