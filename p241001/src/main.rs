mod bool;
mod emoji;

fn main() {
    let float1: f32 = 1.1;
    let float2 = 2.2f32;
    let float3 = 3.3; // default f64
    let float4 = 11_000.555_001;

    println!("float1: {}", float1);
    println!("float2: {}", float2);
    println!("float3: {}", float3);
    println!("float4: {}", float4);


    bool::bools();
    emoji::emoji();
}
