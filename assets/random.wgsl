
struct VertexOutput {
	[[builtin(position)]] clip_position: vec4<f32>;
	[[location(0)]] world_position: vec4<f32>;
	[[location(1)]] world_normal: vec3<f32>;
	[[location(2)]] uv: vec2<f32>;
};

[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
	var output = vec4<f32>(input.uv, 0.0, 0.2);
    return output;
}