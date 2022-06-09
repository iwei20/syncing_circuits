// Random functions based on https://thebookofshaders.com/10/
let random_scale: f32 = 43758.5453123;
let random_x: f32 = 12.9898;
let random_y: f32 = 78.233;

fn random(x: f32) -> f32 {
    return fract(sin(x) * random_scale);
}

fn random_vec2(st: vec2<f32>) -> f32 {
    return random(dot(st, vec2<f32>(random_x, random_y)));
}

struct VertexOutput {
	[[builtin(position)]] clip_position: vec4<f32>;
	[[location(0)]] world_position: vec4<f32>;
	[[location(1)]] world_normal: vec3<f32>;
	[[location(2)]] uv: vec2<f32>;
};

struct Info {
	time: f32;
	power: f32;
};

[[group(1), binding(0)]] var<uniform> info: Info;

[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
	//var r = random_vec2(vec2<f32>(input.world_position[0], input.world_position[1]));
	var r = random(random(info.time) + random_vec2(input.uv)) * 0.5;

	var alpha = 0.6 - info.power * 1024.0 / 255.0;
	var output = vec4<f32>(r, r, r, alpha);
    return output;
}