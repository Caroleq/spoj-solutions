"""
   The script generates test case and answer for LISTREE (LIS and tree) Problem.
"""


from random import choice

def generate_input_tree(tree_length):
    tree = { 1: [] }
    unused_nodes = [node_idx for node_idx in range(2, tree_length + 1)]

    while len(unused_nodes) > 0:
        child_node = choice(unused_nodes)
        unused_nodes.remove(child_node)
    
        parent_node = choice(list(tree.keys()))

        tree[parent_node].append(child_node)
        tree[child_node] = [parent_node]

    return tree

def save_tree_to_test_file(tree, file_name):
    tree_edges = set()
    for parent, children in tree.items():
        tree_edges.update([
            (min(parent, child), max(parent, child)) 
        for child in children])

    with open(file_name, "w") as test_in:
        test_in.write("1\n")
        test_in.write(str(len(tree)) + "\n")
    
        for edge in tree_edges:
            test_in.write(str(edge[0]) + " " + str(edge[1]) + "\n")

# Source: https://www.geeksforgeeks.org/longest-increasing-subsequence-dp-3/
def lisEndingAtIdx(arr, idx):
    if idx == 0:
        return 1

    mx = 1
    for prev in range(idx):
        if arr[prev] < arr[idx]:
            mx = max(mx, lisEndingAtIdx(arr, prev) + 1)
    return mx

def lis(arr):
    n = len(arr)
    res = 1
    for idx in range(1, n):
        res = max(res, lisEndingAtIdx(arr, idx))
    return res

def dfs(leaf, tree, parent, path):

    if len(tree[leaf]) == 1 and parent is not None:
        return [path[:] + [leaf]]

    paths = []

    for neighbor in tree[leaf]:
        local_path = path[:] + [leaf]
        if neighbor != parent:
            paths += dfs(neighbor, tree, leaf, local_path)

    return paths

def compute_maximum_path_length(tree):
    leaves = filter(lambda node: len(tree[node]) == 1, tree.keys())
    paths = []
    for leaf in leaves:
        paths += dfs(leaf, tree, None, [])

    maximum_path_length = 0
    for path in paths:
        maximum_path_length = max(maximum_path_length, lis(path))

    return maximum_path_length

if __name__ == "__main__":
    tree_length = 200
    tree = generate_input_tree(tree_length)
    save_tree_to_test_file(tree, 'test.in')
    maximum_path_length = compute_maximum_path_length(tree)
    with open("test.out", "w") as test_out:
        test_out.write(str(maximum_path_length) + "\n")