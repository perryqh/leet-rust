struct Solution {}

#[derive(Debug)]
struct GarbageTruck {
    material: String,
    traveled: i32,
    banked_travel: i32,
    pick_ups: i32,
}
impl GarbageTruck {
    fn new(material: String) -> Self {
        Self {
            material,
            traveled: 0,
            banked_travel: 0,
            pick_ups: 0,
        }
    }
    fn travel(&mut self, distance: i32) {
        self.banked_travel += distance;
    }
    fn pick_up(&mut self) {
        self.pick_ups += 1;
        // The travel was only necessary if something is picked up
        self.traveled += self.banked_travel;
        self.banked_travel = 0;
    }
}

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let mut minutes = 0;
        let mut trucks = vec![
            GarbageTruck::new(String::from("G")),
            GarbageTruck::new(String::from("P")),
            GarbageTruck::new(String::from("M")),
        ];

        for (index, travel_distance) in travel.into_iter().enumerate() {
            Solution::pickup_and_travel(
                &mut trucks,
                garbage[index].as_str(),
                Some(travel_distance),
            );
        }
        Solution::pickup_and_travel(&mut trucks, garbage.last().unwrap().as_str(), None);

        for truck in trucks.iter_mut() {
            if truck.pick_ups > 0 {
                minutes += truck.traveled + truck.pick_ups;
            }
        }
        minutes
    }

    fn pickup_and_travel(
        trucks: &mut Vec<GarbageTruck>,
        materials: &str,
        travel_distance: Option<i32>,
    ) {
        for truck in trucks.iter_mut() {
            for material in materials.chars() {
                if truck.material.contains(material) {
                    truck.pick_up();
                }
            }
            if let Some(travel_distance) = travel_distance {
                truck.travel(travel_distance);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::garbage_collection(
                vec![
                    String::from("G"),
                    String::from("P"),
                    String::from("GP"),
                    String::from("GG")
                ],
                vec![2, 4, 3]
            ),
            21
        );
        assert_eq!(
            Solution::garbage_collection(
                vec![String::from("MMM"), String::from("PGM"), String::from("GP")],
                vec![3, 10]
            ),
            37
        );
    }
}
