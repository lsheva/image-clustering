use neon::prelude::*;
use rayon::prelude::*;
use std::convert::TryInto;
use std::fmt::Display;

pub fn cluster_from_js_array(mut cx: FunctionContext) -> JsResult<JsArray> {
    let js_arr_handle: Handle<JsArray> = cx.argument(0)?;
    let vec: Vec<Handle<JsValue>> = js_arr_handle.to_vec(&mut cx)?;
    let mut bitstring_vec: Vec<String> = vec![];
    for i in vec {
        let item = i.to_string(&mut cx).unwrap().value();
        bitstring_vec.push(item);
    }

    let max_linkage = cx.argument::<JsNumber>(1)?.value() as u64;
    let clusters = cluster_from_bitstring(bitstring_vec, max_linkage);

    let clusters_js: Handle<JsArray> = JsArray::new(&mut cx, clusters.len().try_into().unwrap());

    for (i, cluster) in clusters.iter().enumerate() {
        let cluster_js: Handle<JsArray> = JsArray::new(&mut cx, cluster.len().try_into().unwrap());
        for (k, &index) in cluster.iter().enumerate() {
            let n = cx.number(index as f64);
            cluster_js.set(&mut cx, k as u32, n)?;
        }
        clusters_js.set(&mut cx, i as u32, cluster_js)?;
    }

    Ok(clusters_js)
}

pub fn cluster_from_bitstring(bitstring_vec: Vec<String>, max_linkage: u64) -> Vec<Vec<usize>> {
    let mut int_vec: Vec<u64> = vec![];

    for bitstring in &bitstring_vec {
        let num = u64::from_str_radix(bitstring, 2).unwrap();
        int_vec.push(num);
    }

    cluster(int_vec, max_linkage)
}

pub fn cluster(input: Vec<u64>, max_linkage: u64) -> Vec<Vec<usize>> {
    let length = input.len();
    let distances = create_distance_array(input);
    let mut clusters: Vec<Vec<usize>> = Vec::new();

    for i in 0..length {
        let item = vec![i];
        clusters.push(item)
    }

    let min_clusters = 1;

    let mut linkage: u64 = 0;
    let mut from: usize = 0;
    let mut to: usize = 0;

    while clusters.len() > min_clusters && linkage < max_linkage {
        let length = clusters.len();
        let mut min_linkage: Option<u8> = None;

        //

        // let i_iter: Vec<usize> = (0..length).collect();
        let res = (1..length).into_par_iter().map(|i| {
            let mut min_linkage: Option<u8> = None;
            let mut from: usize = 0;
            let mut to: usize = 0;
            for j in 0..i {
                let cluster_a = clusters.get(i).unwrap();
                let cluster_b = clusters.get(j).unwrap();
                let linkage = linkage_of(cluster_a, cluster_b, &distances);

                if min_linkage == None || Some(linkage) < min_linkage {
                    min_linkage = Some(linkage);
                    from = j;
                    to = i;
                }
            }

            // println!(
            //     "min_linkage {}, from {}, to {}",
            //     match min_linkage {
            //         Some(x) => format!("{}", x),
            //         None => "not set".to_string(),
            //     },
            //     from,
            //     to
            // );
            return (min_linkage, from, to);
        });

        let tuple = res.min_by_key(|&(min_linkage, _, _)| min_linkage).unwrap();

        // println!(
        //     "min_linkage {}, from {}, to {}",
        //     match tuple.0 {
        //         Some(x) => x,
        //         None => 0,
        //     },
        //     tuple.1,
        //     tuple.2
        // );

        min_linkage = tuple.0;
        from = tuple.1;
        to = tuple.2;
        // ========= WITHOUT THREADS =========
        //
        // for i in 0..length {
        //     for j in 0..i {
        //         let cluster_a = clusters.get(i).unwrap();
        //         let cluster_b = clusters.get(j).unwrap();
        //         let linkage = linkage_of(cluster_a, cluster_b, &distances);

        //         if min_linkage == None || linkage < min_linkage.unwrap() {
        //             min_linkage = Some(linkage);
        //             from = j;
        //             to = i;
        //         }
        //         println!("min_linkage {}, from {}, to {}", linkage, j, i);
        //     }
        // }

        // splitting mutable references to avoid double borrowing
        let (part1, part2) = clusters.split_at_mut(from + 1);

        part2[(to - from - 1)].append(&mut part1[from]);
        clusters.remove(from);
        linkage = min_linkage.unwrap().into();
    }
    clusters
}

pub fn linkage_of(cluster_a: &Vec<usize>, cluster_b: &Vec<usize>, distances: &Vec<Vec<u8>>) -> u8 {
    let mut dist: Vec<u8> = Vec::new();
    for item_a in cluster_a {
        for item_b in cluster_b {
            let current_dist = distance_of(*item_a, *item_b, distances);
            dist.push(current_dist)
        }
    }
    let link = linkage(dist);
    link
}

pub fn distance_of(i: usize, j: usize, distances: &Vec<Vec<u8>>) -> u8 {
    if i > j {
        return distances[i][j];
    }
    distances[j][i]
}

pub fn linkage(dist: Vec<u8>) -> u8 {
    let max_value = dist.iter().max().unwrap();
    *max_value
}

pub fn create_distance_array(input: Vec<u64>) -> Vec<Vec<u8>> {
    let length = input.len();
    let m = (0..length).into_par_iter().map(|i| {
        let mut j_acc: Vec<u8> = vec![];
        for j in 0..i {
            let distance = compare_hashes(input[i], input[j]);
            j_acc.push(distance);
        }
        j_acc
    });
    let mut matrix: Vec<Vec<u8>> = vec![vec![]; length];
    m.collect_into_vec(&mut matrix);
    matrix
    //
    // ========= WITHOUT THREADS =========
    // let mut matrix: Vec<Vec<u8>> = vec![vec![]; length];
    // for i in 1..length {
    //     let mut j_acc: Vec<u8> = vec![];
    //     for j in 0..i {
    //         let distance = compare_hashes(input[i], input[j]);
    //         j_acc.push(distance);
    //     }
    //     matrix.push(j_acc);
    // }
}

pub fn compare_hashes(hash1: u64, hash2: u64) -> u8 {
    let xor = hash1 ^ hash2;
    let count = xor.count_ones();
    count.try_into().unwrap()
}

pub fn print_vector<T: Display>(vector: &Vec<T>) {
    print!("[");
    for i in vector {
        print!("{},", i)
    }
    println!("]");
}

pub fn print_2d_vector<T: Display>(vector: &Vec<Vec<T>>) {
    println!("==========");
    for i in vector {
        print_vector(i);
    }
    println!("==========\n");
}

register_module!(mut cx, {
    cx.export_function("cluster_from_js_array", cluster_from_js_array)
});
