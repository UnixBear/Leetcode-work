from typing import List
from collections import defaultdict
import heapq
class Solution:
    def networkDelayTime(self, times: List[List[int]], n: int, k: int) -> int:
        #First we need to create a list of our adjacency relationships to check.
        #In a language like C++, we would create a map
        adjacencyList = defaultdict(list)
        
        #We'll populate our map with our list relationships 
        for u_i,v_i,w_i in times:
            adjacencyList[u_i].append((w_i, v_i))
        
        #We want to make sure that we have a unique list of elements that corrospond to our visited nodes
        #and the set() object is basically a list that only stores unique elements
        visited=set()

        #We'll initiallize our heap with a list of a zero weight to our starting node k. We do this because
        #we will be popping from this as we explore our node relations and we want our starting node to be the node
        #we compare with our adjacency list first
        heap = [(0, k)]

        #our searching will occur in this loop that 
        while heap:
            
            #since every iteration involves us popping from our heap, it make sense to make our halt condition
            #an empty heap since that will imply there are no more nodes we are connected to
            travel_time, node = heapq.heappop(heap)

            #reminder: this a set so adding an element that is already there will do nothing
            visited.add(node)

            #this is a separate halting condition that procs when we have visited every node.  This is our 
            #intended halting condition because a full visited list implies we've visited every list and
            #we can sum up our weights, which is just travel_time
            if len(visited)==n:
                return travel_time

            #here we iterate through each relation indexed to our key (aka node) and see if any of them
            #have been visited
            for time, adjacent_node in adjacencyList[node]:

                #if we have a new node, we will add a new element to our now empty heap that changes the
                #target node to the current adjacent node paired with the current travel time plus the 
                #time in the adjancy list associated with the relationship
                if adjacent_node not in visited:
                    heapq.heappush(heap, (travel_time+time, adjacent_node))
        
        #if our previous halting condition never occurs (aka when the length of the set 'visited') never
        #reaches n, then there is no edge in our adjacency list left to iterate over such that we can heappush 
        #another element into heap which means we didn't reach every node so we should return -1
        return -1
