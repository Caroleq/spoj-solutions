/// Solution to Ada and Orange Tree Problem (https://www.spoj.com/problems/ADAORANG/)
/// Implementation is heavily based on https://cp-algorithms.com/graph/lca_binary_liftime_ing.html

#include <iostream>
#include <vector>
#include <bitset>
#include <cmath>

int tree_size, tree_size_log;
std::vector<std::vector<int>> tree_edges;

const int max_size = 450000;
int timer;
std::vector<int> time_in(max_size);
std::vector<int> time_out(max_size);
std::vector<int> orange_color(max_size);
std::vector<std::vector<int>> up(max_size, std::vector<int>(ceil(log2(max_size)) + 1));
std::vector<std::bitset<260>> colors_in_subtree(max_size);

void parse_tree_from_input(const int edges_cnt) {
    tree_edges.clear();
    tree_edges.resize(edges_cnt + 1);

    for (size_t edge_idx = 0; edge_idx < edges_cnt; edge_idx++) 
    {
        uint64_t left;
        uint64_t right;

        std::cin >> left;
        std::cin >> right;

        tree_edges[left].push_back(right);
        tree_edges[right].push_back(left);
    }
}

void dfs(const int v, const int p)
{
    colors_in_subtree[v].set(orange_color[v]);
    time_in[v] = ++timer;
    up[v][0] = p;
    for (int i = 1; i <= tree_size_log; ++i)
    {
        up[v][i] = up[up[v][i-1]][i-1];
    }

    for (int u : tree_edges[v]) 
    {
        if (u != p)
        {
            dfs(u, v);
            colors_in_subtree[v] |= colors_in_subtree[u];
        }
    }

    time_out[v] = ++timer;
}

bool is_ancestor(const int u, const int v)
{
    return time_in[u] <= time_in[v] && time_out[u] >= time_out[v];
}

int lca(int u, const int v)
{
    if (is_ancestor(u, v))
    {
        return u;
    }
        
    if (is_ancestor(v, u))
    {
        return v;
    }
        
    for (int i = tree_size_log; i >= 0; --i) 
    {
        if (!is_ancestor(up[u][i], v))
        {
            u = up[u][i];
        }
    }
    return up[u][0];
}

size_t count_all_orange_colors_in_subtree(const size_t left, const size_t right)
{
    return colors_in_subtree[lca(left, right)].count();
}

int main()
{
    std::ios::sync_with_stdio(false);
    size_t cases_count;
    std::cin >> cases_count;

    for (size_t idx = 0; idx < cases_count; idx++)
    {
        int queries_cnt;
        int root_number;

        std::cin >> tree_size;
        std::cin >> queries_cnt;
        std::cin >> root_number;

        for (size_t color_idx = 0; color_idx < tree_size; color_idx++)
        {
            uint64_t color;
            std::cin >> color;
            orange_color[color_idx] = color;
        }

        parse_tree_from_input(tree_size - 1);

        for (size_t idx = 0; idx < tree_size; idx++)
        {
            colors_in_subtree[idx] = 0;
        }

        timer = 0;
        tree_size_log = ceil(log2(tree_size));
        dfs(root_number, root_number);
        
        for (size_t query_idx = 0; query_idx < queries_cnt; query_idx++)
        {
            size_t left, right;
            std::cin >> left;
            std::cin >> right;
            std::cout << count_all_orange_colors_in_subtree(left, right) << std::endl;
        }
    }

    return 0;
}