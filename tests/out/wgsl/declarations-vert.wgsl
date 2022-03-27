struct VertexData {
    position: vec2<f32>,
    a: vec2<f32>,
}

struct FragmentData {
    position: vec2<f32>,
    a: vec2<f32>,
}

struct TestStruct {
    a: f32,
    b: f32,
}

struct VertexOutput {
    @location(0) position: vec2<f32>,
    @location(1) a: vec2<f32>,
    @location(2) out_array: vec4<f32>,
    @location(3) out_array_1: vec4<f32>,
}

var<private> vert: VertexData;
var<private> frag: FragmentData;
var<private> in_array_2: array<vec4<f32>,2u>;
var<private> out_array: array<vec4<f32>,2u>;
var<private> array_2d: array<array<f32,2u>,2u>;
var<private> array_toomanyd: array<array<array<array<array<array<array<f32,2u>,2u>,2u>,2u>,2u>,2u>,2u>;

fn main_1() {
    var positions: array<vec3<f32>,2u> = array<vec3<f32>,2u>(vec3<f32>(-1.0, 1.0, 0.0), vec3<f32>(-1.0, -1.0, 0.0));
    var strct: TestStruct = TestStruct(1.0, 2.0);
    var from_input_array: vec4<f32>;
    var a_1: f32;
    var b: f32;

    let _e34 = in_array_2;
    from_input_array = _e34[1];
    let _e39 = array_2d;
    a_1 = _e39[0][0];
    let _e50 = array_toomanyd;
    b = _e50[0][0][0][0][0][0][0];
    out_array[0] = vec4<f32>(2.0);
    return;
}

@stage(vertex) 
fn main(@location(0) position: vec2<f32>, @location(1) a: vec2<f32>, @location(2) in_array: vec4<f32>, @location(3) in_array_1: vec4<f32>) -> VertexOutput {
    vert.position = position;
    vert.a = a;
    in_array_2[0] = in_array;
    in_array_2[1] = in_array_1;
    main_1();
    let _e30 = frag.position;
    let _e32 = frag.a;
    let _e35 = out_array[0];
    let _e37 = out_array[1];
    return VertexOutput(_e30, _e32, _e35, _e37);
}
