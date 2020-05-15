/*
 *   Copyright (c) 2020 Goutham Krishna K V <gauthamkrishna9991@live.com>
 *   All rights reserved.

 *   Permission is hereby granted, free of charge, to any person obtaining a copy
 *   of this software and associated documentation files (the "Software"), to deal
 *   in the Software without restriction, including without limitation the rights
 *   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *   copies of the Software, and to permit persons to whom the Software is
 *   furnished to do so, subject to the following conditions:
 
 *   The above copyright notice and this permission notice shall be included in all
 *   copies or substantial portions of the Software.
 
 *   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *   SOFTWARE.
 */

use std::io as stdio;
fn main() {
    let mut input = String::new();
    stdio::stdin().read_line(&mut input).expect("Error in reading project");
    match input.trim().parse::<isize>() {
        Err(e) => {
            println!("No value has been given. Program stopping.");
            println!("ERROR: {}", e);
        },
        Ok(n) => {
            let size = n;
            let mut v : Vec<isize> = Vec::new();
            for _ in 0..size {
                input.clear();
                stdio::stdin().read_line(&mut input).expect("Error in input");
                match input.trim().parse::<isize>() {
                    Err(e) => {
                        println!("Error in parsing string {} <ERR: {}>", input, e);
                    },
                    Ok(x) => {
                        v.push(x);
                    }
                }
            }
            // Displaying the array, for debug purposes.
            // println!("ARR: {:#?}", v);
            // Continue your program from here. Remember that this is very safe code.
            v.sort();
            // Displaying the array, for debug purposes.
            // println!("ARR: {:#?}", v);
            let mut sum : isize = 0;
            let mean : isize;
            for i in &v {
                sum += i;
            }
            mean = sum / (v.len() as isize);
            println!("SUM : {}", sum);
            println!("MEAN: {}", mean);
            if &v.len() %2  == 0 {
                if v.len() != 0 {
                    match v.get(v.len() / 2 - 1) {
                        Some(x) => {
                            println!("MEDIAN 1 : {}", x)
                        },
                        None => {
                            println!("MEDIAN 1 DOESN'T EXIST.");
                        }
                    }
                    match v.get(v.len() / 2) {
                        Some(x) => {
                            println!("MEDIAN 2 : {}", x);
                        },
                        None => {
                            println!("MEDIAN 2 DOESN'T EXIST.");
                        }
                    }
                } else {
                    println!("THE VECTOR IS EMPTY");
                }
            } else {
                match v.get((v.len() - 1) / 2) {
                    Some(x) => {
                        println!("MEDIAN: {}", x);
                    },
                    None => {
                        println!("MEDIAN DOESN'T EXIST");
                    }
                }
            }
        }
    }
}
