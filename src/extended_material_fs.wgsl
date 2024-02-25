
#import bevy_pbr::{
    mesh_view_bindings::globals,
    mesh_view_bindings::view,
    mesh_functions,
    skinning,
    morph::morph,
    forward_io::{Vertex, VertexOutput},
    view_transformations::position_world_to_clip,
}
#import bevy_render::instance_index::get_instance_index



@vertex
fn vertex(vertex_no_morph: Vertex) -> VertexOutput {
    var out: VertexOutput;
    var vertex = vertex_no_morph;
    var model = mesh_functions::get_model_matrix(vertex_no_morph.instance_index);

    out.world_normal = mesh_functions::mesh_normal_local_to_world(
        vertex.normal,
        // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
        // See https://github.com/gfx-rs/naga/issues/2416
        get_instance_index(vertex_no_morph.instance_index)
    );

    out.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    out.position = position_world_to_clip(out.world_position.xyz);
/* 
    let playerpos = view.view[3].xyz;
    let len = distance(playerpos.xz, out.world_position.xz);
    let dist = max(0., pow(distance(playerpos.xz, out.world_position.xz) * 0.1, 2.) - 10.);

        out.position.y += (sin(out.world_position.x * 0.02 + globals.time ) * dist + cos(out.world_position.z * 0.02 + globals.time ) * dist) * 0.2;


 */

 //   out.uv = vertex.uv + my_extended_material.animated_texture_offset.xy;
    out.uv = vertex.uv;

#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416
   //  out.instance_index = get_instance_index(vertex_no_morph.instance_index);
#endif

#ifdef BASE_INSTANCE_WORKAROUND
    // Hack: this ensures the push constant is always used, which works around this issue:
    // https://github.com/bevyengine/bevy/issues/10509
    // This can be removed when wgpu 0.19 is released
    out.position.x += min(f32(get_instance_index(0u)), 0.0);
#endif


    return out;
}
