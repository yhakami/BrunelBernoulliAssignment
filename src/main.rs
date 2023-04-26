//written by yazeed hakami. stu.id: withdrewn due to this being a public repository.
//This program is a simple calculator for the volumetric flow rate, area of cross-section, velocity, and total pressure head of a fluid in a pipe, as those were the values missing from the data collected at the lab.
//The program is written in Rust, and the data is inputted manually. The program is not meant to be used by anyone other than the author, and is not meant to be used for any other purpose than the one it was written for.


// I will take the liberty of assuming that the reader of this code is familiar with the underlying equations, the units of the variables used, and rudementary knowledge of Rust. If not, please follow the comments or refer to the readme file if it exists. 

 use std::f64::consts::PI; // 3.14159265358979323846264338327950288, for precise precision :^).
 fn main() {

    ///INPUT YOUR DATA HERE!!!
    ///////////////////////////////////////////////////////////////
     let locations = vec!{"A", "B", "C", "D", "E", "F"};
     let volumes_collected: Vec<f64> = vec!{2.0,2.0,2.0,2.0,2.0,2.0};// ml
     let times_collected: Vec<f64> = vec!{2.0,2.0,2.0,2.0,2.0,2.0};// s
     let diameters: Vec<f64> = vec!{2.0,2.0,2.0,2.0,2.0,2.0}; // mm, this doesn't change.
     let static_pressure_heads: Vec<f64> = vec!{2.0,2.0,2.0,2.0,2.0,2.0};// mm
     //////////////////////////////////////////////////////////////
     /// 
     /// 
     let g = 9.81; // m/s^2, this is that earthly all-encompassing mysterious constant we call gravity.
 
    ////////////////////////////////////////////////////////////
    /// 
     let mut volumetric_flow_rates = Vec::new();
     let mut areas = Vec::new();
     let mut velocities = Vec::new();
     let mut total_pressure_heads_calculated = Vec::new();
    /// Initializing the vectors. These essentially act as a table, or a dataset.
    ////////////////////////////////////////////////////////////
     for i in 0..locations.len() {
         let volume_collected_m3 = volumes_collected[i] / 1_000_000.0; // m^3 // 1L = 1,000,000 ml for further clarity.
         let volumetric_flow_rate = volume_collected_m3 / times_collected[i]; // m^3/s // Q = V/t
         let diameter_m: f64 = diameters[i] / 1000.0; // m
         let area = PI * diameter_m.powf(2.0) / 4.0; // m^2 // A = pi*D^2/4 // D is diameter.
         let velocity = volumetric_flow_rate / area; // m/s // Q/A = v // Q is volumetric flow rate, A is area, v is velocity.
         let velocity_head = velocity.powi(2) / (2.0 * g); // m // v^2/2g is the underlying equation.
         let total_pressure_head_calculated = (static_pressure_heads[i] / 1000.0) + velocity_head; // m // H+(v^2/2g) is the underlying equation. H is the static pressure head.
 
         volumetric_flow_rates.push(volumetric_flow_rate); 
         areas.push(area);
         velocities.push(velocity);
         total_pressure_heads_calculated.push(total_pressure_head_calculated); // 
 
         println!("Location: {}", locations[i]);
         println!("  Volumetric Flow Rate: {:.6} m^3/s", volumetric_flow_rate);
         println!("  Area of Cross-Section: {:.6} m^2", area);
         println!("  Velocity: {:.6} m/s", velocity);
         println!("  Total Pressure Head Calculated: {:.6} m", total_pressure_head_calculated);
         println!();
         //Yes, I know I didn't use scientific notation. Quite annoying when dealing with code.
     }
 }
 