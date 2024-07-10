/// Solution to Ada and Plum Problem (https://www.spoj.com/problems/ADAVISIT/)
/// Implementation is heavily based on https://cp-algorithms.com/graph/lca_binary_liftime_ing.html

#include <iostream>
#include <vector>
#include <cmath>

int tree_size;
int tree_size_log;

typedef struct 
{
    int start_cnt = 0;
    int lca_count = 0;
    int node_parent_idx = 0;
    int visits_to_add_count = 0;
    int lca_ancestor_count = 0;
} node_t;

std::vector<std::vector<int>> tree_edges;


constexpr int max_size = 400005;
int timer = 0;
int time_in[max_size] = {0};
int time_out[max_size] = {0};
node_t tree_nodes[max_size];

std::vector<std::vector<int>> levels(max_size);
std::vector<std::vector<int>> up(ceil(log2(max_size)) + 1, std::vector<int>(max_size));

void parse_tree_from_input(const int edges_cnt) 
{
    tree_edges.resize(edges_cnt + 1);

    for (size_t edge_idx = 0; edge_idx < edges_cnt; edge_idx++) 
    {
        uint64_t left;
        uint64_t right;

        std::cin >> left >> right;

        tree_edges[left - 1].push_back(right - 1);
        tree_edges[right - 1].push_back(left - 1);
    }
}

void dfs(const int v, const int parent, int level)
{
    time_in[v] = ++timer;
    up[0][v] = parent;

    tree_nodes[v].node_parent_idx = parent;

    levels[level].push_back(v);

    for (int i = 1; i <= tree_size_log; ++i)
    {
        up[i][v] = up[i-1][up[i-1][v]];
    }

    for (const auto &u : tree_edges[v]) 
    {
        if (u != parent)
        {
            dfs(u, v, level + 1);
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
    for (int i = tree_size_log; i >= 0; --i) 
    {
        if (!is_ancestor(up[i][u], v))
        {
            u = up[i][u];
        }
    }
    return up[0][u];
}

void visit_consecutive_nodes(int start_node)
{
    int next_node_idx = (start_node + 1) % tree_edges.size();

    if (is_ancestor(start_node, next_node_idx))
    {
        tree_nodes[next_node_idx].start_cnt += 1;
        tree_nodes[start_node].lca_count += 1;
        tree_nodes[start_node].lca_ancestor_count += 1;
        return;
    }
        
    if (is_ancestor(next_node_idx, start_node))
    {
        tree_nodes[start_node].start_cnt += 1;
        tree_nodes[next_node_idx].lca_count += 1;
        tree_nodes[next_node_idx].lca_ancestor_count += 1;
        return;
    }

    int lca_node = lca(start_node, next_node_idx);
    tree_nodes[lca_node].lca_count += 1;
    tree_nodes[start_node].start_cnt += 1;
    tree_nodes[next_node_idx].start_cnt += 1;
}

void visit_all_nodes()
{
    for (size_t idx = 0; idx < tree_edges.size() - 1; ++idx)
    {
        visit_consecutive_nodes(idx);
    }
}

std::vector<int> count_visits_for_all_nodes()
{
    std::vector<int> visit_counts(tree_edges.size());
    for (int level = levels.size() - 1; level >= 0; --level)
    {
        for (const auto &node: levels[level])
        {
            if (node != 0 && node != tree_edges.size() - 1)
            {
                visit_counts[node] += 2;
            }
            else 
            {
                visit_counts[node] += 1;
            }

            const auto &node_struct = tree_nodes[node];
            visit_counts[node] += node_struct.visits_to_add_count - node_struct.lca_count;
            tree_nodes[node_struct.node_parent_idx].visits_to_add_count += node_struct.visits_to_add_count - node_struct.lca_count * 2 + 
                                                                           node_struct.lca_ancestor_count + node_struct.start_cnt;
        }
    }

    return visit_counts;
} 

int main()
{
    std::ios::sync_with_stdio(false);
    std::cin >> tree_size;
    
    parse_tree_from_input(tree_size - 1);

    tree_size_log = ceil(log2(tree_size));
    dfs(0, 0, 0);

    visit_all_nodes();

    const auto visit_counts = count_visits_for_all_nodes();
    for (auto visit_count: visit_counts)
    {
        std::cout << visit_count << std::endl;
    }

    return 0;
}