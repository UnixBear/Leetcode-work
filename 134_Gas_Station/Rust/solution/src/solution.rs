pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        if gas.iter().fold(0, |acc, &x| acc + x) < cost.iter().fold(0, |acc, &x| acc + x) {
            return -1
        }
        let mut totalGas = 0;
        let mut totalCost = 0;
        let mut tracker = 0;
        for (index,value) in gas.iter().enumerate() {
            totalGas += value;
            totalCost += cost[index];
            if totalCost > totalGas {
                totalCost = 0;
                totalGas = 0;
                tracker = (index + 1) % gas.len();
            }
        }
        return tracker as i32

    }
}