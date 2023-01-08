fn main() {
    let s = "gpu_thermal-virtual-0
Adapter: Virtual device
temp1:        +38.8°C  

littlecore_thermal-virtual-0
Adapter: Virtual device
temp1:        +39.8°C  

bigcore0_thermal-virtual-0
Adapter: Virtual device
temp1:        +38.8°C  

tcpm_source_psy_6_0022-i2c-6-22
Adapter: rk3x-i2c
in0:           0.00 V  (min =  +0.00 V, max =  +0.00 V)
curr1:         0.00 A  (max =  +0.00 A)

npu_thermal-virtual-0
Adapter: Virtual device
temp1:        +38.8°C  

center_thermal-virtual-0
Adapter: Virtual device
temp1:        +38.8°C  

bigcore1_thermal-virtual-0
Adapter: Virtual device
temp1:        +38.8°C  

soc_thermal-virtual-0
Adapter: Virtual device
temp1:        +38.8°C  (crit = +115.0°C)";

    println!("{:?}", extract_temperatures(s));
}

fn extract_temperatures(input: &str) -> Vec<f32> {
        let mut temps = Vec::new();
        for line in input.lines() {
            if line.starts_with("temp1:") {
                let temp_str = line.split(":").nth(1).unwrap();
                let temp_str = temp_str.split("°C").nth(0).unwrap();
                let temp_str = temp_str.trim();
                let temp_str = temp_str.trim_start_matches("+");
                let temp: f32 = temp_str.parse().unwrap();
                temps.push(temp);
            }
        }
        temps
    }