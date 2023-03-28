window.SIDEBAR_ITEMS = {"fn":[["all_simple_paths","Returns an iterator that produces all simple paths from `from` node to `to`, which contains at least `min_intermediate_nodes` nodes and at most `max_intermediate_nodes`, if given, or limited by the graph’s order otherwise. The simple path is a path without repetitions."],["astar","[Generic] A* shortest path algorithm."],["bellman_ford","[Generic] Compute shortest paths from node `source` to all other."],["condensation","[Graph] Condense every strongly connected component into a single node and return the result."],["connected_components","[Generic] Return the number of connected components of the graph."],["dijkstra","[Generic] Dijkstra’s shortest path algorithm."],["find_negative_cycle","[Generic] Find the path of a negative cycle reachable from node `source`."],["floyd_warshall","[Generic] Floyd–Warshall algorithm is an algorithm for all pairs shortest path problem"],["greedy_feedback_arc_set","[Generic] Finds a feedback arc set: a set of edges in the given directed graph, which when removed, make the graph acyclic."],["greedy_matching","[Generic] Compute a matching using a greedy heuristic."],["has_path_connecting","[Generic] Check if there exists a path starting at `from` and reaching `to`."],["is_bipartite_undirected","Return `true` if the graph is bipartite. A graph is bipartite if its nodes can be divided into two disjoint and indepedent sets U and V such that every edge connects U to one in V. This algorithm implements 2-coloring algorithm based on the BFS algorithm."],["is_cyclic_directed","[Generic] Return `true` if the input directed graph contains a cycle."],["is_cyclic_undirected","[Generic] Return `true` if the input graph contains a cycle."],["is_isomorphic","[Generic] Return `true` if the graphs `g0` and `g1` are isomorphic."],["is_isomorphic_matching","[Generic] Return `true` if the graphs `g0` and `g1` are isomorphic."],["is_isomorphic_subgraph","[Generic] Return `true` if `g0` is isomorphic to a subgraph of `g1`."],["is_isomorphic_subgraph_matching","[Generic] Return `true` if `g0` is isomorphic to a subgraph of `g1`."],["k_shortest_path","[Generic] k’th shortest path algorithm."],["kosaraju_scc","[Generic] Compute the strongly connected components using Kosaraju’s algorithm."],["maximum_matching","[Generic] Compute the maximum matching using Gabow’s algorithm."],["min_spanning_tree","[Generic] Compute a minimum spanning tree of a graph."],["scc","Renamed to `kosaraju_scc`."],["subgraph_isomorphisms_iter","Using the VF2 algorithm, examine both syntactic and semantic graph isomorphism (graph structure and matching node and edge weights) and, if `g0` is isomorphic to a subgraph of `g1`, return the mappings between them."],["tarjan_scc","[Generic] Compute the strongly connected components using Tarjan’s algorithm."],["toposort","[Generic] Perform a topological sort of a directed graph."]],"mod":[["astar",""],["bellman_ford","Bellman-Ford algorithms."],["dijkstra",""],["dominators","Compute dominators of a control-flow graph."],["feedback_arc_set",""],["floyd_warshall",""],["isomorphism",""],["k_shortest_path",""],["matching",""],["simple_paths",""],["tred","Compute the transitive reduction and closure of a directed acyclic graph"]],"struct":[["Cycle","An algorithm error: a cycle was found in the graph."],["DfsSpace","Workspace for a graph traversal."],["Matching","Computed matching of the graph."],["MinSpanningTree","An iterator producing a minimum spanning forest of a graph."],["NegativeCycle","An algorithm error: a cycle of negative weights was found in the graph."],["TarjanScc","A reusable state for computing the strongly connected components using Tarjan’s algorithm."]],"trait":[["BoundedMeasure",""],["FloatMeasure","A floating-point measure."],["Measure","Associated data that can be used for measures (such as length)."]]};