/// Solution to Lowest Common Ancestor Problem (https://www.spoj.com/problems/LCASQ/)
/// Implementation is heavily based on https://cp-algorithms.com/graph/lca_binary_liftime_ing.html

#include <iostream>
#include <vector>
#include <cmath>

int tree_size, tree_size_log;
std::vector<std::vector<int>> tree_edges;

const int max_size = 100001;
int timer;
std::vector<int> time_in(max_size);
std::vector<int> time_out(max_size);
std::vector<std::vector<int>> up(max_size, std::vector<int>(ceil(log2(max_size)) + 1));

void parse_tree_from_input(const int nodes_cnt) {
    tree_edges.clear();
    tree_edges.resize(nodes_cnt);

    for (size_t node_idx = 0; node_idx < nodes_cnt; node_idx++) 
    {
        size_t child_nodes_cnt;
        std::cin >> child_nodes_cnt;
        for (size_t child_idx = 0; child_idx < child_nodes_cnt; child_idx++) 
        {
            uint64_t child_node;
            std::cin >> child_node;

            tree_edges[node_idx].push_back(child_node);

        }
    }
}

void dfs(const int v, const int p)
{
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

int main()
{
    std::ios::sync_with_stdio(false);


    std::cin >> tree_size;
    parse_tree_from_input(tree_size);

    timer = 0;
    tree_size_log = ceil(log2(tree_size));
    dfs(0, 0);

    size_t queries_cnt;
    std::cin >> queries_cnt;

    for (size_t query_idx = 0; query_idx < queries_cnt; query_idx++)
    {
        size_t left, right;
        std::cin >> left;
        std::cin >> right;
        std::cout << lca(left, right) << std::endl;
    }

    return 0;
}