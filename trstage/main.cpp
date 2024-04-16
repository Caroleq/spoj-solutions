/// Solution to Traveling by Stagecoach Problem (https://www.spoj.com/problems/TRSTAGE/)

#include <iostream>
#include <vector>
#include <queue>
#include <algorithm>

using graph_type = std::vector<std::vector<std::pair<uint64_t,uint64_t>>>;

const uint32_t scale_factor = 100000;

graph_type read_roads_to_graph(uint64_t roads_cnt, uint64_t cities_cnt, uint64_t start_idx)
{
    graph_type graph(cities_cnt);

    for (size_t idx = 0; idx < roads_cnt; idx++)
    {
        uint64_t src_road_idx;
        uint64_t dst_road_idx;
        uint64_t distance;

        std::cin >> src_road_idx;
        std::cin >> dst_road_idx;
        std::cin >> distance;

        src_road_idx -= 1;
        dst_road_idx -= 1;
        distance *= scale_factor;

        if (dst_road_idx != start_idx)
        {
            graph[src_road_idx].push_back(std::make_pair(dst_road_idx, distance));
        }

        if (src_road_idx != start_idx)
        {
            graph[dst_road_idx].push_back(std::make_pair(src_road_idx, distance));
        }
    }

    return graph;
}

struct queue_comparator
{
    bool operator()(std::vector<uint64_t> const& route_a, std::vector<uint64_t> const& route_b) const
    {
        return route_a[0] > route_b[0];
    }
};

float compute_min_travel_time(uint64_t start_idx, uint64_t dest_idx, graph_type &graph, std::vector<uint64_t> &horses_cnts)
{
    std::priority_queue<std::vector<uint64_t>, std::vector<std::vector<uint64_t>>, queue_comparator> priority_queue;
    priority_queue.push({0, start_idx, 0, 1ULL << start_idx});

    std::vector<std::vector<bool>> visited;
    for (size_t idx = 0; idx < graph.size(); idx++)
    {
        visited.emplace_back(1 << horses_cnts.size());
    }

    while (!priority_queue.empty())
    {
        const auto best = priority_queue.top();
        priority_queue.pop();

        const auto distance = best[0];
        const auto city_idx = best[1];
        const auto horses = best[2];
        const auto cities = best[3];

        if (city_idx == dest_idx) {
            return (float)distance / scale_factor;
        }

        if (visited[city_idx][horses])
        {
            continue;
        }
        visited[city_idx][horses] = true;

        for (auto &neighbor_city: graph[city_idx])
        {
            if ((cities & (1ULL << neighbor_city.first)) > 0) {
                continue;
            }

            for (size_t horse_idx = 0; horse_idx < horses_cnts.size(); horse_idx++)
            {
                if ((horses & (1 << horse_idx)) > 0) {
                    continue;
                }

                if (visited[neighbor_city.first][horses | (1 << horse_idx)]) {
                    continue;
                }

                const auto new_dist_to_neighbor = distance + neighbor_city.second / horses_cnts[horse_idx];
                priority_queue.push({new_dist_to_neighbor, neighbor_city.first, horses | (1 << horse_idx), cities | (1ULL  << neighbor_city.first)});
            }
        }
    }
    
    return -1;
}

int main()
{
    std::ios::sync_with_stdio(false);

    while (true)
    {
        uint64_t ticket_cnts;
        uint64_t cities_cnt;
        uint64_t roads_cnt;
        uint64_t dest_idx;
        uint64_t src_idx;

        std::cin >> ticket_cnts;
        std::cin >> cities_cnt;
        std::cin >> roads_cnt;
        std::cin >> src_idx;
        std::cin >> dest_idx;

        if (ticket_cnts == 0)
        {
            break;
        }

        src_idx -= 1;
        dest_idx -= 1;

        std::vector<uint64_t> horses_cnts(ticket_cnts);
        for (size_t idx = 0; idx < ticket_cnts; idx++)
        {
            std::cin >> horses_cnts[idx];
        }

        auto roads = read_roads_to_graph(roads_cnt, cities_cnt, src_idx);

        const auto result = compute_min_travel_time(src_idx, dest_idx, roads, horses_cnts);
        if (result < 0)
        {
            std::cout << "Impossible" << std::endl;
        }
        else 
        {
            std::cout << result << std::endl;
        }
    }

    return 0;
}