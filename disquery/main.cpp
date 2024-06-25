/// Solution to Distance Query Problem (https://www.spoj.com/problems/DISQUERY/)
/// Implementation is heavily based on https://cp-algorithms.com/graph/lca_binary_liftime_ing.html

#include <iostream>
#include <vector>
#include <cmath>

int tree_size, tree_size_log;
std::vector<std::vector<std::pair<int, int>>> tree_edges;

const int maximum_edge_length = 1000000;
const int max_size = 100005;
int timer;
std::vector<int> time_in(max_size);
std::vector<int> time_out(max_size);
std::vector<std::vector<int>> up(max_size, std::vector<int>(ceil(log2(max_size)) + 1));
std::vector<std::vector<int>> min_lengths(max_size, std::vector<int>(ceil(log2(max_size)) + 1));
std::vector<std::vector<int>> max_lengths(max_size, std::vector<int>(ceil(log2(max_size)) + 1));

void parse_tree_from_input(const int edges_cnt) {
    tree_edges.clear();
    tree_edges.resize(edges_cnt + 1);

    for (size_t edge_idx = 0; edge_idx < edges_cnt; edge_idx++) 
    {
        uint64_t left;
        uint64_t right;
        uint64_t length;

        std::cin >> left >> right >> length;

        tree_edges[left - 1].push_back({right - 1, length});
        tree_edges[right - 1].push_back({left - 1, length});
    }
}

void dfs(const int v, const int parent, int length)
{
    time_in[v] = ++timer;
    up[v][0] = parent;

    // ugly detection of root node.
    if (length == 0)
    {
        min_lengths[v][0] = maximum_edge_length;
    }
    else 
    {
        min_lengths[v][0] = length;
    }
    
    max_lengths[v][0] = length;

    for (int i = 1; i <= tree_size_log; ++i)
    {
        up[v][i] = up[up[v][i-1]][i-1];
        min_lengths[v][i] = std::min(min_lengths[up[v][i - 1]][i - 1], min_lengths[v][i - 1]);
        max_lengths[v][i] = std::max(max_lengths[up[v][i - 1]][i - 1], max_lengths[v][i - 1]);
    }

    for (const auto &u : tree_edges[v]) 
    {
        if (u.first != parent)
        {
            dfs(u.first, v, u.second);
        }
    }

    time_out[v] = ++timer;
}

bool is_ancestor(const int u, const int v)
{
    return time_in[u] <= time_in[v] && time_out[u] >= time_out[v];
}

std::pair<int, int> find_min_and_max_edge_lengths(size_t left, size_t right) 
{
    if (left == right)
    {
        return {0, 0};
    }

    int min_length = maximum_edge_length;
    int max_length = 0;

    for (int i = tree_size_log - 1; i >= 0; --i)
    {
        if (!is_ancestor(up[left][i], right)) 
        {
            min_length = std::min(min_length, min_lengths[left][i]);
            max_length = std::max(max_length, max_lengths[left][i]);
            left = up[left][i];
        }

        if (!is_ancestor(up[right][i], left)) 
        {
            min_length = std::min(min_length, min_lengths[right][i]);
            max_length = std::max(max_length, max_lengths[right][i]);
            right = up[right][i];
        }
    }

    if (up[right][0] == left) 
    {
        min_length = std::min(min_length, min_lengths[right][0]);
        max_length = std::max(max_length, max_lengths[right][0]);
    } 
    else if (up[left][0] == right) 
    {
        min_length = std::min(min_length, min_lengths[left][0]);
        max_length = std::max(max_length, max_lengths[left][0]);
    } 
    else 
    {
        min_length = std::min(min_length, std::min(min_lengths[left][0], min_lengths[right][0]));
        max_length = std::max(max_length, std::max(max_lengths[left][0], max_lengths[right][0]));
    }
    
    return {min_length, max_length};
}

int main()
{
    std::ios::sync_with_stdio(false);
    std::cin >> tree_size;
    
    parse_tree_from_input(tree_size - 1);

    timer = 0;
    tree_size_log = ceil(log2(tree_size));
    dfs(0, 0, 0);

    int queries_cnt;
    std::cin >> queries_cnt;
    for (size_t query_idx = 0; query_idx < queries_cnt; query_idx++)
    {
        size_t left, right;
        std::cin >> left;
        std::cin >> right;

        const auto result = find_min_and_max_edge_lengths(left - 1, right - 1);
        std::cout << result.first << " " << result.second << std::endl;
    }

    return 0;
}